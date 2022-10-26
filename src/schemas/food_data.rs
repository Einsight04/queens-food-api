use serde::{Deserialize, Serialize};

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
    pub is_vegetarian: bool,
    pub allergens: String,
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
    name: String,
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
