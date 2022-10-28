use std::collections::HashMap;
use std::iter::Map;
use actix_web::{get, post};
use actix_web::{web, HttpResponse, Responder};
use redis::Commands;


#[get("")]
pub async fn locations(pool: web::Data<r2d2::Pool<redis::Client>>) -> impl Responder {
    let mut con = pool.get().unwrap();

    let mut location_data: HashMap<String, Vec<String>> = HashMap::new();

    let keys: Vec<String> = con.keys("*").unwrap();

    // get all fields of those keys
    for key in keys {
        let fields: Vec<String> = con.hkeys(&key).unwrap();
        location_data.insert(key, fields);
    }

    HttpResponse::Ok().json(location_data)
}
