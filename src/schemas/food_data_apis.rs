use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From<UncleanedFoodApi> for HashMap<String, Vec<CleanedFoodApi>> {
    fn from(food: UncleanedFoodApi) -> Self {
        let mut cleaned_data: HashMap<String, Vec<CleanedFoodApi>> = HashMap::new();

        for meal_period in food.meal_periods {
            for station in meal_period.stations {
                for sub_category in station.sub_categories {
                    for item in sub_category.items {
                        let cleaned_food_api = CleanedFoodApi::from(item);

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

impl From<Item> for CleanedFoodApi {
    fn from(item: Item) -> Self {
        CleanedFoodApi {
            product_name: item.product_name,
            short_description: item.short_description,
            serving: item.serving,
            calories: item.calories,
            calories_from_fat: item.calories_from_fat,
            total_fat: item.total_fat,
            saturated_fat: item.saturated_fat,
            trans_fat: item.trans_fat,
            cholesterol: item.cholesterol,
            sodium: item.sodium,
            total_carbohydrates: item.total_carbohydrates,
            dietary_fiber: item.dietary_fiber,
            sugars: item.sugars,
            protein: item.protein,
            is_vegetarian: item.is_vegetarian,
        }
    }
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
