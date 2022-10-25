use crate::LennyDish;
use redis::Commands;
use std::sync::Arc;
use chrono::{DateTime, Utc, Duration};

pub async fn handler() {
    let client = redis::Client::open("redis://127.0.0.1:6379")
        .expect("Failed to connect to redis");


    let mut con = client.get_connection()
        .expect("Failed to get connection");

    clear_food_info(&mut con);
    food_info_builder(&mut con)
        .await;
}

fn clear_food_info(con: &mut redis::Connection) {
    // delete all food info
    let _: () = con.del("food_info")
        .expect("Failed to delete food_info");
}

async fn food_info_builder(con: &mut redis::Connection) {
    // for loop to get past 7 days including today
    let mut food_info: Vec<String> = Vec::new();

    for day in 0..7 {
        let date = Utc::today() - Duration::days(day);

        println!("date: {}", date.format("%m-%d-%Y"));

        con.hset::<String, String, String, ()>(
            "food_info".to_string(), date.format("%m-%d-%Y").to_string(), "food info".to_string())
            .expect("Failed to set hash");

        // get my_key
        // let my_key: String = con.get("my_key")
        //     .expect("Failed to get my_key");
        //
        // println!("my_key: {}", my_key);

        // add to db

        // let resp = reqwest::get(
        //     format!("https://studentweb.housing.queensu.ca/public/campusDishAPI/campusDishAPI.php?locationId=14627&mealPeriod=Lunch&selDate={}", date.format("%m-%d-%Y")))
        //     .await?
        //     .json::<LennyDish>()
        //     .await?;
        // food_info.push(resp);
    }
}
