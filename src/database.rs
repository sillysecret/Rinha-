use crate::{Pessoa, Newp};
use sqlx::{PgPool,postgres::PgPoolOptions, Row};
use uuid::Uuid;

pub struct Repository{
    pool: PgPool, 
}

impl Repository {
    pub async fn conn(url : String) -> Self {
        Repository{
        pool : PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap(),
        }
    }


    pub async fn find(&self  , id: Uuid) -> Result<Option<Pessoa>, sqlx::Error> {
        sqlx::query_as("
            SELECT * FROM people WHERE id=$1
        ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    
    
    }
    pub  async fn create(&self  , newp:Newp) -> Result<Pessoa, sqlx::Error>{
        sqlx::query_as("
            INSERT INTO people (id,name,nick,birth_date,stack)
            VALUES($1, $2, $3, $4, $5)
            RETURNING id,name,birth_date,stack
        ",
        )
        .bind(Uuid::now_v7())
        .bind(newp.nome)
        .bind(newp.apelido)
        .bind(newp.nascimento)
        .bind(newp.stack)
        .fetch_one(&self.pool)
        .await
    
    }
 
    pub  async fn search(&self  , query: String) ->Result<Option<Pessoa>, sqlx::Error>{
        sqlx::query_as("
            SELECT * 
            FROM people 
            WHERE to_tsquery('people',$1) @@ search
            LIMIT 50
        ",
        )
        .bind(query)
        .fetch_optional(&self.pool)
        .await
    
    }

    pub async fn count(&self) ->Result<i32, sqlx::Error>{
            sqlx::query(
            "
            SELECT count(*) FROM people 
            ",
            )
        .fetch_one(&self.pool)
        .await
        .map(|row| row.get(0))
        
    }
}
