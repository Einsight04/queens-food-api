use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LennyDish {
    meal_periods: Vec<MealPeriod>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct MealPeriod {
    meal_period: String,
    stations: Vec<Station>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Station {
    id: String,
    name: String,
    sort: u32,
    sub_categories: Vec<SubCategory>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SubCategory {
    name: String,
    sort: u32,
    items: Vec<Item>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Item {
    product_name: String,
    serving: String,
    calories: String,
    calories_from_fat: String,
    total_fat: String,
    saturated_fat: String,
    trans_fat: String,
    cholesterol: String,
    sodium: String,
    total_carbohydrates: String,
    dietary_fiber: String,
    sugars: String,
    protein: String,
    is_vegetarian: bool,
    allergens: String,
}
