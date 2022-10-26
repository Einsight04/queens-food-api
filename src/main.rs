mod api;
mod schemas;
mod services;

extern crate redis;

use crate::services::food_data;
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder, HttpRequest
};
use dotenv::dotenv;
use env_logger::Env;
use schemas::uncleaned_food_api::UncleanedFoodApi;
use std::env::var;
use std::sync::{Arc};
use std::time::Duration;
use redis::Commands;
use tokio::time::sleep;

#[tokio::main]
async fn request() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(
        "https://studentweb.housing.queensu.ca/public/campusDishAPI/campusDishAPI.php?locationId=14627&mealPeriod=Lunch&selDate=10-22-2022")
        .await?
        .json::<UncleanedFoodApi>()
        .await?;

    // print serialized json
    println!("{:#?}", resp);

    Ok(())
}

async fn hello(pool: web::Data<r2d2::Pool<redis::Client>>) -> impl Responder {
    // get connectoin from r2d2 pool
    let mut con = pool.get().unwrap();

    // get key lenny, value Breakfast
    let breakfast: String = con.hget("lenny", "Breakfast").unwrap();

    // return as stringified json
    HttpResponse::Ok().body(breakfast)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //load env vars
    dotenv().expect("dotenv load fail");
    let (user, pass, address) = (
        var("REDIS_USER").expect("failed to load redis user"),
        var("REDIS_PASS").expect("failed to load redis pass"),
        var("REDIS_ADDRESS").expect("failed to load redis address"),
    );

    // setup env logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // setup redis client
    let client = redis::Client::open("redis://127.0.0.1:6379").expect("Failed to connect to redis");

    // create r2d2 pool
    let pool = r2d2::Pool::builder()
        .build(client)
        .expect("failed to create pool");

    // create shared pool state
    let wrapped = Arc::new(pool);

    // get connection from pool
    let mut con: r2d2::PooledConnection<redis::Client> = wrapped.get().unwrap();

    actix_web::rt::spawn(async move {
        food_data::updater(&mut con).await;

        actix_web::rt::spawn(async move {
            loop {
                food_data::updater(&mut con).await;
                sleep(Duration::from_secs(60)).await;
            }
        });
    });

    // rate limiter, 3 requests per second
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(5)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::from(wrapped.clone()))
            .wrap(Governor::new(&governor_conf))
            .wrap(Logger::default())
            .wrap(cors)
            .route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
