use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;


#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct LennyDish {
    MealPeriods: Vec<MealPeriod>,
}


#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct MealPeriod {
    MealPeriod: String,
    Stations: Vec<Station>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Station {
    Id: String,
    Name: String,
    Sort: u32,
    SubCategories: Vec<SubCategory>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct SubCategory {
    Name: String,
    Sort: u32,
    Items: Vec<Item>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Item {
    ProductName: String,
    Serving: String,
    Calories: String,
    CaloriesFromFat: String,
    TotalFat: String,
    SaturatedFat: String,
    TransFat: String,
    Cholesterol: String,
    Sodium: String,
    TotalCarbohydrates: String,
    DietaryFiber: String,
    Sugars: String,
    Protein: String,
    IsVegetarian: bool,
    Allergens: String,
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
