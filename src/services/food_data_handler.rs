use std::collections::HashMap;

use crate::schemas::food_data_apis::{CleanedFoodApi, UncleanedFoodApi};
use chrono::Utc;
use futures::future::{self, try_join_all};
use redis::Commands;

static LOCATION_IDS: &'static [(&str, i32)] =
    &[("lenny", 14627), ("ban_righ", 14628), ("jean_royce", 14629)];

static MEAL_PERIODS: &'static [&str] = &["Breakfast", "Lunch", "Dinner"];

pub async fn updater(con: &mut r2d2::PooledConnection<redis::Client>) {
    let current_date = Utc::now().format("%Y-%m-%d").to_string();
    let client = reqwest::Client::new();

    let mut collected_futures = vec![];

    for (_, location_id) in LOCATION_IDS {
        for meal_period in MEAL_PERIODS {
            let client = &client;
            let resp = client.get(format!(
                "https://studentweb.housing.queensu.ca/public/campusDishAPI/campusDishAPI.php?locationId={}&mealPeriod={}&selDate={}",
                location_id, meal_period, current_date)).send();
            collected_futures.push(resp);
        }
    }

    let mut responses = future::try_join_all(collected_futures).await.unwrap();

    for (location_name, _) in LOCATION_IDS {
        for meal_period in MEAL_PERIODS {
            // take top thing off vec, this shit is all in still in order lol
            let food_data = responses
                .remove(0)
                .json::<UncleanedFoodApi>()
                .await
                .unwrap();

            // if empty ignore it
            if food_data.meal_periods.is_empty() {
                continue;
            }

            let cleaned = HashMap::<String, Vec<CleanedFoodApi>>::from(food_data);

            // pass in as json
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

pub fn clear_all(con: &mut r2d2::PooledConnection<redis::Client>) {
    let keys: Vec<String> = con.keys("*").unwrap();

    for key in keys {
        let _: () = con.del(key).unwrap();
    }
}