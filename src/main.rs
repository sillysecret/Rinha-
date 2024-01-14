use std::{collections::HashMap, vec};

use serde::{Serialize, Deserialize};

use uuid::Uuid;

use time::Date;

use time::macros::date;

use std::sync::Arc;

use tokio::sync::Mutex;
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Router, 
    extract::{State,Path}, Json,
};

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Serialize,Clone)]
struct Pessoa {
    pub id: Uuid,
    pub nome: String,
    pub apelido:String,
    #[serde(with = "date_format" )]
    pub nascimento: Date,
    pub stack: Vec<String>
}



#[derive(Serialize,Clone,Deserialize)]
struct Newp{
    pub nome: String,
    pub apelido:String,
    #[serde(with = "date_format" )]
    pub nascimento: Date,
    pub stack: Vec<String>
}
 
type AppState = Arc<Mutex<HashMap<Uuid,Pessoa>>>;

#[tokio::main]
async fn main() {

    let eu = Pessoa {
        id: Uuid::now_v7(),
        nome: "João".to_string(),
        apelido:"kriger".to_string(), 
        nascimento: date!(2004 - 05 - 11),
        stack: vec!["Rust".to_string(), "Java".to_string()]
    };

    
    let mut localbd : HashMap<Uuid,Pessoa> = HashMap::new(); 
    
    println!("{}",eu.id);
    
    localbd.insert(eu.id,eu);

    let app_state : AppState = Arc::new(Mutex::new(localbd));

    // build our applica wtion with a single route
    let app = Router::new()
    .route("/pessoa", get(search))
    .route("/pessoas/:id",get(find))
    .route("/pessoas",post(create))
    .route("/count", get(count))
    .with_state(app_state);


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}  

async fn find(State(localbd): State<AppState>,Path(id):Path<Uuid>,) -> impl IntoResponse {
                    
    match localbd.lock().await.get(&id){
        Some(pessoa) => Ok(Json(pessoa.clone())), 
        None => Err(StatusCode::NOT_FOUND),
    }
    
}

async fn search() -> impl IntoResponse {
    
    (StatusCode::NOT_FOUND, "ok")
}

async fn create(State(localbd): State<AppState>,Json(payload): Json<Newp>) -> impl IntoResponse {
    let id = Uuid::now_v7();
    let newp = Pessoa {
        id, 
        nome: payload.nome,
        apelido: payload.apelido,
        nascimento:payload.nascimento,
        stack:payload.stack,
    };
    localbd.lock().await.insert(id, newp.clone());
    (StatusCode::OK,Json(newp))
         
}

async fn count(State(localbd): State<AppState>,) -> impl IntoResponse {
    let tam = localbd.lock().await.len().to_string(); 
    
    (StatusCode::OK, tam)
} 

