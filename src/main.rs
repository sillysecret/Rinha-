use std::{collections::HashMap, vec};
use uuid::Uuid;
use time::Date;
use time::macros::date;

use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Router,
};

struct Pessoa {
    id: Uuid,
    nome: String,
    nascimento: Date,
    stack: Vec<String>
}



#[tokio::main]
async fn main() {

    let x = Pessoa {
        id: Uuid::now_v7(),
        nome: "João".to_string(),
        nascimento: date!(2004 - 05 - 11),
        stack: vec!["Rust".to_string(), "Java".to_string()]
    };

    let localbd : HashMap<Uuid,Pessoa> = HashMap::new();  
    
    // build our application with a single route
    let app = Router::new()
    .route("/pessoa", get(search))
    .route("/pessoas/:id", get(find))
    .route("/pessoas",post(create))
    .route("/count", get(count));


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn search() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "ok")
}

async fn find() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "ok")
}

async fn create() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "ok")
}

async fn count() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "ok")
}

