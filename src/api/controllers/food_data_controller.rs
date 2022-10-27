use actix_web::{HttpResponse, Responder, web};
use redis::Commands;
use actix_web::{get, post};

#[get("/{location}/{meal_period}")]
pub async fn food_data(path: web::Path<(String, String)>, pool: web::Data<r2d2::Pool<redis::Client>>) -> impl Responder {
    let (location, meal_period) = path.into_inner();
    let mut con = pool.get().unwrap();

    match con.hexists::<String, String, bool>(location.clone(), meal_period.clone()) {
        Ok(status) => {
            if !status {
                return HttpResponse::NotFound().body("food not found");
            }
        }
        Err(e) => {
            return HttpResponse::NotFound().body(e.to_string());
        }
    }

    // let food_data: String = match con.hget(location, meal_period) {
    //     Ok(data) => data,
    //     Err(e) => return HttpResponse::NotFound().body(e.to_string()),
    // };
    //
    // HttpResponse::Ok().body(food_data)

    // rewrite without variable and without unwrap
    match con.hget::<String, String, String>(location, meal_period) {
        Ok(data) => HttpResponse::Ok().body(data),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}