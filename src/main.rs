mod schemas;
mod scheduler;
mod api;

extern crate redis;

use std::env::var;
use std::sync::Arc;
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{middleware::Logger, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use redis::Commands;
use dotenv::dotenv;
use serde::Deserialize;
use tokio::time::{sleep};
use env_logger::Env;
use clokwerk::{Scheduler, TimeUnits};
use clokwerk::Interval::*;
use std::thread;
use std::time::Duration;
use actix_web::cookie::time::ext::NumericalDuration;
use schemas::food_api::LennyDish;
use scheduler::{scheduler_setup, tasks::food_info_setup};


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
    //load env vars
    dotenv().expect("dotenv load fail");
    let (user, pass, address) = (
        var("REDIS_USER").expect("failed to load redis user"),
        var("REDIS_PASS").expect("failed to load redis pass"),
        var("REDIS_ADDRESS").expect("failed to load redis address"),
    );

    //setup env logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    food_info_setup::handler()
        .await;
    scheduler_setup::setup_scheduler();

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
