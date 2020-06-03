use actix_web::{web, App, Responder, HttpServer, HttpResponse};


async fn index() -> impl Responder {
    "Hello world!"
}

async fn add_book() ->impl Responder{
    let mut books: Vec<String> = Vec::new();
    books.push("new book".to_string());
    HttpResponse::Ok().body("A book has been added")
}

async fn get_books() -> impl Responder{
    let mut books: Vec<String> = Vec::new();
    books.push("be rusty".to_string());
    books.push("be rustacean".to_string());
    return web::Json(books);
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut v: Vec<String> = Vec::new();
    HttpServer::new(|| {
        App::new()
            .route("/books",web::post().to(add_book))
            .route("/books",web::get().to(get_books))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}