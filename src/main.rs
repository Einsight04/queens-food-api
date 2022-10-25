mod food_api;

extern crate redis;

use std::env::var;
use std::sync::Arc;
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{middleware::Logger, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger::Env;
use redis::Commands;
use serde::Deserialize;
use food_api::LennyDish;
use tokio::time::{sleep, Duration};


#[tokio::main]
async fn request() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(
        "https://studentweb.housing.queensu.ca/public/campusDishAPI/campusDishAPI.php?locationId=14627&mealPeriod=Lunch&selDate=10-22-2022")
        .await?
        .json::<LennyDish>()
        .await?;

    // print serialized json
    println!("{:#?}", resp);

    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // print out hello world every 5 seconds
    // tokio::spawn(async move {
    //     loop {
    //         println!("Hello world!");
    //         sleep(Duration::from_secs(1 * 60)).await;
    //     }
    // });

    //load env vars
    dotenv().expect("dotenv load fail");
    let (user, pass, address) = (
        var("REDIS_USER").expect("failed to load redis user"),
        var("REDIS_PASS").expect("failed to load redis pass"),
        var("REDIS_ADDRESS").expect("failed to load redis address"),
    );

    //setup env logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // setup redis client
    let client = redis::Client::open("redis://127.0.0.1:6379")
        .expect("Failed to connect to redis");

    // create r2d2 pool
    let pool = r2d2::Pool::builder()
        .build(client)
        .expect("failed to create pool");

    // create shared pool state
    let wrapped = Arc::new(pool);

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
            .service(hello)
    })
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
