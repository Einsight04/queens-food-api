use std::collections::HashMap;
use serde::{Deserialize, Serialize};


// impl CleanedFoodApi {
impl From<UncleanedFoodApi> for HashMap<String, Vec<CleanedFoodApi>> {
    fn from(food: UncleanedFoodApi) -> Self {
        let mut cleaned_data: HashMap<String, Vec<CleanedFoodApi>> = HashMap::new();

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
                            trans_fat: item.trans_fat.clone(),
                            cholesterol: item.cholesterol.clone(),
                            sodium: item.sodium.clone(),
                            total_carbohydrates: item.total_carbohydrates.clone(),
                            dietary_fiber: item.dietary_fiber.clone(),
                            sugars: item.sugars.clone(),
                            protein: item.protein.clone(),
                            is_vegetarian: item.is_vegetarian,
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


#[derive(Serialize, Debug)]
pub struct CleanedFoodApi {
    pub(crate) product_name: String,
    pub(crate) short_description: String,
    pub(crate) serving: String,
    pub(crate) calories: String,
    pub(crate) calories_from_fat: String,
    pub(crate) total_fat: String,
    pub(crate) saturated_fat: String,
    pub(crate) trans_fat: String,
    pub(crate) cholesterol: String,
    pub(crate) sodium: String,
    pub(crate) total_carbohydrates: String,
    pub(crate) dietary_fiber: String,
    pub(crate) sugars: String,
    pub(crate) protein: String,
    pub(crate) is_vegetarian: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UncleanedFoodApi {
    pub(crate) meal_periods: Vec<MealPeriod>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MealPeriod {
    meal_period: String,
    pub(crate) stations: Vec<Station>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Station {
    id: String,
    pub(crate) name: String,
    sort: u32,
    pub(crate) sub_categories: Vec<SubCategory>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SubCategory {
    name: String,
    sort: u32,
    pub(crate) items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    pub(crate) product_name: String,
    pub(crate) short_description: String,
    pub(crate) serving: String,
    pub(crate) calories: String,
    pub(crate) calories_from_fat: String,
    pub(crate) total_fat: String,
    pub(crate) saturated_fat: String,
    pub(crate) trans_fat: String,
    pub(crate) cholesterol: String,
    pub(crate) sodium: String,
    pub(crate) total_carbohydrates: String,
    pub(crate) dietary_fiber: String,
    pub(crate) sugars: String,
    pub(crate) protein: String,
    pub(crate) is_vegetarian: bool,
    pub(crate) allergens: String,
}
