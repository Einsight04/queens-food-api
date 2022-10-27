mod api;
mod schemas;
mod services;

extern crate redis;

use crate::services::food_data_handler;
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use api::controllers::food_data_controller;
use chrono::Utc;
use dotenv::dotenv;
use env_logger::Env;
use redis::Commands;
use std::env::var;
use std::sync::Arc;
use tokio::time::sleep;
use crate::api::controllers::location_data_controller;

async fn hello(pool: web::Data<r2d2::Pool<redis::Client>>) -> impl Responder {
    // get connectoin from r2d2 pool
    let mut con = pool.get().unwrap();

    // get key lenny, value Breakfast
    let breakfast: String = con.hget("lenny", "Breakfast").unwrap();

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
        food_data_handler::clear_all(&mut con);
        food_data_handler::updater(&mut con).await;

        loop {
            let now = Utc::now();
            let tomorrow = now + chrono::Duration::days(1);
            let tomorrow = tomorrow.date().and_hms(0, 0, 0);
            let duration = tomorrow - now;
            sleep(duration.to_std().unwrap()).await;
            food_data_handler::updater(&mut con).await;
        }
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
            .service(web::scope("/api")
                .service(web::scope("/food").service(food_data_controller::food_data))
                .service(web::scope("/locations").service(location_data_controller::locations)))
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
        .bind("192.168.0.107:4000")?
        .run()
        .await
}
