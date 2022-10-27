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

    for (location_name, location_id) in LOCATION_IDS {
        for meal_period in MEAL_PERIODS {
            let client = &client;
            let resp = client.get(format!(
                    "https://studentweb.housing.queensu.ca/public/campusDishAPI/campusDishAPI.php?locationId={}&mealPeriod={}&selDate={}",
                    location_id, meal_period, current_date)).send()
                .await
                .unwrap()
                .json::<UncleanedFoodApi>()
                .await
                .unwrap();

            // let cleaned = Vec::<HashMap<String, CleanedFoodApi>>::from(resp);
            let cleaned = HashMap::<String, Vec<CleanedFoodApi>>::from(resp);

            // store as hashset
            let _: () = con
                .hset(
                    location_name,
                    meal_period.to_lowercase(),
                    serde_json::to_string(&cleaned).unwrap(),
                )
                .unwrap();
        }
    }
}
