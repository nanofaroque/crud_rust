use actix_web::{web, App, Responder, HttpServer, HttpResponse};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput, PutItemInput, AttributeValue, PutItemOutput, PutItemError};
use std::collections::HashMap;


async fn index() -> impl Responder {
    "Hello world!"
}

async fn add_book() -> impl Responder {
    let client = DynamoDbClient::new(Region::UsEast1);
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("bookId".into(), AttributeValue {
        s: Some(String::from("2")),
        ..Default::default()
    });
    item.insert("name".into(), AttributeValue {
        s: Some(String::from("Welcome to AWS Dynamodb")),
        ..Default::default()
    });
    let put = PutItemInput {
        table_name: "books".into(),
        item,
        ..Default::default()
    };
    client.put_item(put).await;

    let mut books: Vec<String> = Vec::new();
    books.push("new book".to_string());
    HttpResponse::Ok().body("A book has been added")
}

async fn get_books() -> impl Responder {
    let mut books: Vec<String> = Vec::new();
    books.push("be rusty".to_string());
    books.push("be rustacean".to_string());
    return web::Json(books);
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let client = DynamoDbClient::new(Region::UsEast1);
    let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(list_tables_input).await {
        Ok(output) => match output.table_names {
            Some(table_name_list) => {
                println!("Tables in database:");

                for table_name in table_name_list {
                    println!("{}", table_name);
                }
            }
            None => println!("No tables in database!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }


    let mut v: Vec<String> = Vec::new();
    HttpServer::new(|| {
        App::new()
            .route("/books", web::post().to(add_book))
            .route("/books", web::get().to(get_books))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}