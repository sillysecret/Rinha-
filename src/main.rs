use std::env;

use database::Repository;
use serde::{Serialize, Deserialize};

use uuid::Uuid;

use time::Date;
use axum::extract::Query;
use std::sync::Arc;


mod database;

 
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Router, 
    extract::{State,Path}, Json,
};

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Serialize,Clone,sqlx::FromRow)]
pub struct Pessoa {
    pub id: Uuid,
    pub nome: String,
    pub apelido:String,
    #[serde(with = "date_format" )]
    pub nascimento: Date,
    pub stack: Option<Vec<String>>
}



#[derive(Serialize,Clone,Deserialize,sqlx::FromRow,)]
pub struct Newp{
    pub nome: String,
    pub apelido:String,
    #[serde(with = "date_format" )]
    pub nascimento: Date,
    pub stack: Option<Vec<String>>
}


#[derive(Deserialize)]
pub struct Querysearch{
    pub query: String
}


type AppState = Arc<Repository>;

#[tokio::main]
async fn main() {
    
    let port =env::var("DATABASE_URL")
        .unwrap_or(String::from("postgres://rinha:rinha@localhost:5432/rinha"));
   
    let db = Repository::conn(port).await;

    let app_state = Arc::new(db);

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
                    
    match localbd.find(id).await{
        Ok(Some(pessoa)) => Ok(Json(pessoa.clone())), 
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
    
}

async fn search(
    State(localbd): State<AppState>, 
    Query(Querysearch { query }) :Query<Querysearch>,) 
    -> impl IntoResponse {
        match localbd.search(query).await {
            Ok(pessoa) => Ok(Json(pessoa)),
            Err(_)=> Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
}
async fn create(State(localbd): State<AppState>,Json(payload): Json<Newp>) -> impl IntoResponse {
    
    if payload.nome.len() > 100 || payload.apelido.len() > 32{
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    match payload.stack {
        Some(ref stack)=>{
            if stack.iter().any(|x| x.len() > 32){
                return Err(StatusCode::UNPROCESSABLE_ENTITY);
            }
        }
        None =>{}
    }

    match localbd.create(payload).await{
        Ok(pessoa) => Ok(Json(pessoa.clone())), 
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => 
        {
            Err(StatusCode::UNPROCESSABLE_ENTITY)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

}

async fn count(State(localbd): State<AppState>,) -> impl IntoResponse {
    match localbd.count().await{
        Ok(count) => Ok(Json(count)),
        Err(_)=> Err(StatusCode::INTERNAL_SERVER_ERROR),
    }  
    

} 

