use crate::schemas::food_data_apis::{CleanedFoodApi, UncleanedFoodApi};
use chrono::Utc;
use redis::Commands;
use serde::{Deserialize, Serialize};

// convert uncleaned food api to cleaned food api
impl From<UncleanedFoodApi> for Vec<CleanedFoodApi> {
    fn from(uncleaned_food_api: UncleanedFoodApi) -> Self {
        let mut cleaned_food_apis: Vec<CleanedFoodApi> = Vec::new();
        for meal_period in uncleaned_food_api.meal_periods {
            for station in meal_period.stations {
                for sub_category in station.sub_categories {
                    for item in sub_category.items {
                        cleaned_food_apis.push(CleanedFoodApi {
                            product_name: item.product_name,
                            short_description: item.short_description,
                            serving: item.serving,
                            calories: item.calories,
                            calories_from_fat: item.calories_from_fat,
                            total_fat: item.total_fat,
                            saturated_fat: item.saturated_fat,
                            trans_fat: "".to_string(),
                            cholesterol: "".to_string(),
                            sodium: "".to_string(),
                            total_carbohydrates: "".to_string(),
                            dietary_fiber: "".to_string(),
                            sugars: "".to_string(),
                            protein: "".to_string(),
                            is_vegetarian: false,
                            allergens: "".to_string(),
                        });
                    }
                }
            }
        }
        cleaned_food_apis
    }
}

pub async fn updater(con: &mut r2d2::PooledConnection<redis::Client>) {
    let location_ids = vec![("lenny", 14627), ("ban_righ", 14628), ("jean_royce", 14629)];

    // get current date
    let current_date = Utc::now();
    let date_string = current_date.format("%m-%d-%Y").to_string();

    // make all requests concurrently
    let mut futures = vec![];

    for location_id in location_ids {
        for meal_period in vec!["breakfast", "lunch", "dinner"] {
            let url = format!(
                "https://api.dineoncampus.com/v1/location/menu?client_id=5e3b3c7c-0c7e-11e9-9c4a-0a580a2803e9&location_id={}&date={}&meal_period={}",
                location_id.1,
                date_string,
                meal_period
            );
            futures.push(reqwest::get(url));
        }
    }

    // run futures all at the same time and store responses as cleaned food apis
    let mut cleaned_food_apis: Vec<CleanedFoodApi> = Vec::new();

    for response in futures::future::join_all(futures).await {
        cleaned_food_apis.append(&mut Vec::from(
            response
                .unwrap()
                .json::<UncleanedFoodApi>()
                .await
                .unwrap(),
        ));
    }


    // store cleaned food apis in redis as a hash with cleanedfoodapi as string
    let _: () = con
        .hset(
            "lenny",
            "Breakfast",
            serde_json::to_string(&cleaned_food_apis).unwrap(),
        )
        .unwrap();
}

// fn cleanup(data: &UncleanedFoodApi) -> Vec<CleanedFoodApi> {
//     // make vector of food data
//     let mut food_data: Vec<CleanedFoodApi> = Vec::new();
//
//     // recreate data as vector of food data
//     food_data.extend(
//         data.meal_periods
//             .iter()
//             .flat_map(|meal_period| &meal_period.stations)
//             .flat_map(|station| &station.sub_categories)
//             .flat_map(|sub_category| &sub_category.items)
//             .map(|item| CleanedFoodApi {
//                 product_name: item.product_name.clone(),
//                 short_description: item.short_description.clone(),
//                 serving: item.serving.clone(),
//                 calories: item.calories.clone(),
//                 calories_from_fat: item.calories_from_fat.clone(),
//                 total_fat: item.total_fat.clone(),
//                 saturated_fat: item.saturated_fat.clone(),
//                 trans_fat: item.trans_fat.clone(),
//                 cholesterol: item.cholesterol.clone(),
//                 sodium: item.sodium.clone(),
//                 total_carbohydrates: item.total_carbohydrates.clone(),
//                 dietary_fiber: item.dietary_fiber.clone(),
//                 sugars: item.sugars.clone(),
//                 protein: item.protein.clone(),
//                 is_vegetarian: item.is_vegetarian,
//                 allergens: item.allergens.clone(),
//             }),
//     );
//
//     food_data
// }
