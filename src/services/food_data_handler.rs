use crate::schemas::food_data_apis::{CleanedFoodApi, UncleanedFoodApi};
use chrono::Utc;
use redis::Commands;
use std::collections::HashMap;

static LOCATION_IDS: &'static [(&str, i32)] =
    &[("lenny", 14627), ("ban_righ", 14628), ("jean_royce", 14629)];

static MEAL_PERIODS: &'static [&str] = &["Breakfast", "Lunch", "Dinner"];

pub async fn updater(con: &mut r2d2::PooledConnection<redis::Client>) {
    // get current date
    let current_date = Utc::now().format("%Y-%m-%d").to_string();
    let client = reqwest::Client::new();

    let mut collected_futures = vec![];

    for (location_name, location_id) in LOCATION_IDS {
        for meal_period in MEAL_PERIODS {
            let client = &client;
            let resp = client.get(format!(
                    "https://studentweb.housing.queensu.ca/public/campusDishAPI/campusDishAPI.php?locationId={}&mealPeriod={}&selDate={}",
                    location_id, meal_period, current_date)).send();
            collected_futures.push((location_name, meal_period, resp));
        }
    }

    // collect all futures
    for (name, period, fut) in collected_futures {
        let resp = fut.await.unwrap().json::<UncleanedFoodApi>().await.unwrap();
        let cleaned = HashMap::<String, Vec<CleanedFoodApi>>::from(resp);

        let _: () = con
            .hset(
                name,
                period.to_lowercase(),
                serde_json::to_string(&cleaned).unwrap(),
            )
            .unwrap();
    }
}
