use serde::{Deserialize, Serialize};


#[derive(Serialize, Debug)]
struct CleanedFoodApi {
    product_name: String,
    short_description: String,
    dietary_information: String,
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
}
