use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[serde(rename_all = "PascalCase")]
#[derive(Deserialize, Debug)]
struct LennyDish {
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


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(
        "https://studentweb.housing.queensu.ca/public/campusDishAPI/campusDishAPI.php?locationId=14627&mealPeriod=Lunch&selDate=10-22-2022")
        .await?
        .json::<LennyDish>()
        .await?;

    // print serialized json
    println!("{:#?}", resp);

    Ok(())
}

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }
//
// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }
//
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//     })
//         .bind(("127.0.0.1", 3000))?
//         .run()
//         .await
// }
