use serde::{Deserialize, Serialize};

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
    pub(crate) serving: String,
    pub(crate) calories: String,
    pub(crate) calories_from_fat: String,
    pub(crate) total_fat: String,
    pub(crate) saturated_fat: String,
    trans_fat: String,
    cholesterol: String,
    sodium: String,
    total_carbohydrates: String,
    dietary_fiber: String,
    sugars: String,
    protein: String,
    is_vegetarian: bool,
    pub(crate) allergens: String,
}
