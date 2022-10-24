mod food_api;
mod database;

extern crate redis;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use redis::Commands;
use serde::Deserialize;
use food_api::LennyDish;
use tokio::time::{sleep, Duration};
use database::CLIENT;


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

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
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

    // redis://default:6tY4kfWONMp92txOmFNFp9ek3wNTAQdI@redis-13163.c83.us-east-1-2.ec2.cloud.redislabs.com:13163
    // let client = redis::Client::open("redis://127.0.0.1:6379")
    //     .expect("Failed to connect to redis");

    let mut con = CLIENT.get_connection()
        .expect("Failed to get connection");

    let _: () = con.set("my_key", "Hello Redis!")
        .expect("Failed to set key");

    let value: String = con.get("my_key")
        .expect("Failed to get key");

    println!("Got value: {}", value);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
