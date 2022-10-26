use crate::LennyDish;
use chrono::{DateTime, Duration, Utc};
use redis::Commands;
use std::sync::Arc;


pub async fn updater(con: &mut r2d2::PooledConnection<redis::Client>) {
    let location_ids = vec![
        ("lenny", 14627),
        ("ban_righ", 14628),
        ("jean_royce", 14629),
    ];

    // get current date
    let current_date = Utc::now();
    let date_string = current_date.format("%m-%d-%Y").to_string();

    // loop through location ids
    for (location_name, location_id) in location_ids {
        //
        for meal_period in vec!["Breakfast", "Lunch", "Dinner"] {

            // get food info
            let resp = reqwest::get(&format!(
                "https://studentweb.housing.queensu.ca/public/campusDishAPI/campusDishAPI.php?locationId={}&mealPeriod=Lunch&selDate={}",
                location_id, date_string
            ))
                .await
                .unwrap()
                .json::<LennyDish>()
                .await
                .unwrap();

            // store as hashset
            let _: () = con
                .hset(
                    location_name,
                    meal_period,
                    serde_json::to_string(&resp).unwrap(),
                )
                .unwrap();
        }
    }
}
