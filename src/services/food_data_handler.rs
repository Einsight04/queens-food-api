use crate::schemas::food_data_apis::{CleanedFoodApi, UncleanedFoodApi};
use chrono::Utc;
use redis::Commands;
use std::collections::HashMap;
use std::iter::Map;

// impl CleanedFoodApi {
impl From<UncleanedFoodApi> for HashMap<String, Vec<CleanedFoodApi>> {
    fn from(food: UncleanedFoodApi) -> Self {
        let mut cleaned_data: HashMap<String, Vec<CleanedFoodApi>> = HashMap::new();

        /*
        // key represents the meal type
        [
            meal type: [ CleanedFoodApi],
            meal type: [ CleanedFoodApi],
            meal type: [ CleanedFoodApi],
        ]
        */

        // food.meal_periods[0].stations[0].name this is the meal type

        for meal_period in &food.meal_periods {
            for station in &meal_period.stations {
                for sub_category in &station.sub_categories {
                    for item in &sub_category.items {
                        let cleaned_food_api = CleanedFoodApi {
                            product_name: item.product_name.clone(),
                            short_description: item.short_description.clone(),
                            serving: item.serving.clone(),
                            calories: item.calories.clone(),
                            calories_from_fat: item.calories_from_fat.clone(),
                            total_fat: item.total_fat.clone(),
                            saturated_fat: item.saturated_fat.clone(),
                            trans_fat: "".to_string(),
                            cholesterol: "".to_string(),
                            sodium: "".to_string(),
                            total_carbohydrates: "".to_string(),
                            dietary_fiber: "".to_string(),
                            sugars: "".to_string(),
                            protein: "".to_string(),
                            is_vegetarian: false,
                            allergens: "".to_string(),
                        };

                        cleaned_data
                            .entry(station.name.clone())
                            .or_insert(Vec::new())
                            .push(cleaned_food_api);
                    }
                }
            }
        }

        cleaned_data
    }
}

pub async fn updater(con: &mut r2d2::PooledConnection<redis::Client>) {
    let location_ids = vec![("lenny", 14627), ("ban_righ", 14628), ("jean_royce", 14629)];
    let meal_periods = vec!["Breakfast", "Lunch", "Dinner"];

    // get current date
    let current_date = Utc::now().format("%Y-%m-%d").to_string();

    for (location_name, location_id) in location_ids {
        for meal_period in &meal_periods {
            let resp = reqwest::get(&format!(
                "https://studentweb.housing.queensu.ca/public/campusDishAPI/campusDishAPI.php?locationId={}&mealPeriod={}&selDate={}",
                location_id, meal_period, current_date
            ))
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
                    meal_period,
                    serde_json::to_string(&cleaned).unwrap(),
                )
                .unwrap();
        }
    }
}
