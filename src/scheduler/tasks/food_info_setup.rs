use crate::LennyDish;
use chrono::{DateTime, Duration, Utc};
use redis::Commands;
use std::sync::Arc;

pub async fn handler(con: &mut r2d2::PooledConnection<redis::Client>) {
    /*
        let client = redis::Client::open("redis://127.0.0.1:6379")
            .expect("Failed to connect to redis");


        let mut con = client.get_connection()
            .expect("Failed to get connection");
    */

    food_info_builder(con).await;
}

async fn food_info_builder(con: &mut r2d2::PooledConnection<redis::Client>) {
    let location_ids = vec![
        ("lenny", 14627),
        ("ban_righ", 14628),
        ("jean_royce", 14629),
    ];

    // get current date
    let current_date = Utc::now();
    let date_string = current_date.format("%m-%d-%Y").to_string();

    // loop through location ids
    for (name, id) in location_ids {
        // get food info
        let resp = reqwest::get(&format!(
            "https://studentweb.housing.queensu.ca/public/campusDishAPI/campusDishAPI.php?locationId={}&mealPeriod=Lunch&selDate={}",
            id, date_string
        ))
            .await
            .unwrap()
            .json::<LennyDish>()
            .await
            .unwrap();
    }


    // let mut food_info: Vec<String> = Vec::new();
    // let date = Utc::now().date().naive_utc();
    //
    // println!("date: {}", date.format("%m-%d-%Y"));
    //
    // con.hset::<String, String, String, ()>(
    //     "food_info".to_string(),
    //     date.format("%m-%d-%Y").to_string(),
    //     "food info".to_string(),
    // )
    // .expect("Failed to set hash");
    //
    // let my_key: String = con.get("my_key").expect("Failed to get my_key");
    //
    // println!("my_key: {}", my_key);
    //
    // let resp = reqwest::get(
    //     format!("https://studentweb.housing.queensu.ca/public/campusDishAPI/campusDishAPI.php?locationId=14627&mealPeriod=Lunch&selDate={}", date.format("%m-%d-%Y")))
    //     .await?
    //     .json::<LennyDish>()
    //     .await?;
    // food_info.push(resp);
}
