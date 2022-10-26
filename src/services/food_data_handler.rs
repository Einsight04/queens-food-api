use crate::schemas::food_data_apis::{CleanedFoodApi, UncleanedFoodApi};
use chrono::Utc;
use redis::Commands;
use serde::{Deserialize, Serialize};

pub async fn updater(con: &mut r2d2::PooledConnection<redis::Client>) {
    let location_ids = vec![("lenny", 14627), ("ban_righ", 14628), ("jean_royce", 14629)];

    // get current date
    let current_date = Utc::now();
    let date_string = current_date.format("%m-%d-%Y").to_string();

    // loop through location ids
    for (location_name, location_id) in location_ids {
        // loop through meal periods
        for meal_period in vec!["Breakfast", "Lunch", "Dinner"] {
            // get food info
            let resp = reqwest::get(&format!(
                "https://studentweb.housing.queensu.ca/public/campusDishAPI/campusDishAPI.php?locationId={}&mealPeriod={}&selDate={}",
                location_id, meal_period, date_string
            ))
                .await
                .unwrap()
                .json::<UncleanedFoodApi>()
                .await
                .unwrap();

            let cleaned = cleanup(&resp);

            // store as hashset
            let _: () = con
                .hset(
                    location_name,
                    meal_period,
                    serde_json::to_string(&cleaned).unwrap(),
                )
                .unwrap();
        }
    }
}

fn cleanup(data: &UncleanedFoodApi) -> Vec<CleanedFoodApi> {
    // make vector of food data
    let mut food_data: Vec<CleanedFoodApi> = Vec::new();

    // recreate data as vector of food data
    food_data.extend(
        data.meal_periods
            .iter()
            .flat_map(|meal_period| &meal_period.stations)
            .flat_map(|station| &station.sub_categories)
            .flat_map(|sub_category| &sub_category.items)
            .map(|item| CleanedFoodApi {
                product_name: item.product_name.clone(),
                short_description: item.short_description.clone(),
                serving: item.serving.clone(),
                calories: item.calories.clone(),
                calories_from_fat: item.calories_from_fat.clone(),
                total_fat: item.total_fat.clone(),
                saturated_fat: item.saturated_fat.clone(),
                trans_fat: item.trans_fat.clone(),
                cholesterol: item.cholesterol.clone(),
                sodium: item.sodium.clone(),
                total_carbohydrates: item.total_carbohydrates.clone(),
                dietary_fiber: item.dietary_fiber.clone(),
                sugars: item.sugars.clone(),
                protein: item.protein.clone(),
                is_vegetarian: item.is_vegetarian,
                allergens: item.allergens.clone(),
            }),
    );

    food_data
}
