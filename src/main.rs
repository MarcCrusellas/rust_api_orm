use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};


#[derive(Debug, serde::Deserialize)]
struct User {
    name: String,
    password: String,
}

#[post("/new")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    // In this example, we simply return the received user's name and password
    HttpResponse::Ok().body(format!("Created user: {:?}", user))
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}


#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}