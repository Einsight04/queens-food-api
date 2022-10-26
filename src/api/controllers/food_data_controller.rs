use actix_web::{HttpResponse, Responder, web};
use redis::Commands;
use actix_web::{get, post};

#[get("/{location}/{meal_period}")]
pub async fn food_data(path: web::Path<(String, String)>, pool: web::Data<r2d2::Pool<redis::Client>>) -> impl Responder {
    let (location, meal_period) = path.into_inner();
    let mut con = pool.get().unwrap();

    if !con.hexists::<String, String, bool>(location.clone(), meal_period.clone()).unwrap() {
        return HttpResponse::NotFound().body("Food data not found");
    }

    let food_data: String = con.hget(location, meal_period).unwrap();

    HttpResponse::Ok().body(food_data)
}