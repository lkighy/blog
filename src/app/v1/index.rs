use actix_web::get;

#[get("/")]
pub async fn index() -> String {
    // HttpResponse::Ok().body("hello")
    String::from("hello")
}

