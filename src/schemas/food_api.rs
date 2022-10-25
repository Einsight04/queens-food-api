use serde::{Deserialize, Serialize};


#[serde(rename_all = "PascalCase")]
#[derive(Deserialize, Debug)]
pub struct LennyDish {
    meal_periods: Vec<MealPeriod>,
}


#[serde(rename_all = "PascalCase")]
#[derive(Deserialize, Debug)]
struct MealPeriod {
    meal_period: String,
    stations: Vec<Station>,
}

#[serde(rename_all = "PascalCase")]
#[derive(Deserialize, Debug)]
struct Station {
    id: String,
    name: String,
    sort: u32,
    sub_categories: Vec<SubCategory>,
}

#[serde(rename_all = "PascalCase")]
#[derive(Deserialize, Debug)]
struct SubCategory {
    name: String,
    sort: u32,
    items: Vec<Item>,
}

#[serde(rename_all = "PascalCase")]
#[derive(Deserialize, Debug)]
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