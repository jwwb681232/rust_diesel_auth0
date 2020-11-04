use dotenv;
use sqlx::mysql::MySqlPool;
use sqlx::{FromRow};

#[derive(Debug, FromRow)]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[async_std::main]
async fn main() -> Result<(), sqlx::error::Error> {
    dotenv::dotenv().ok();


    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = MySqlPool::connect(&database_url).await?;


    let mut users = vec![];
    let recs = sqlx::query!( r#" SELECT * FROM users "# ).fetch_all(&db_pool).await?;

    for rec in recs {
        users.push(User {
            id: rec.id,
            first_name: rec.first_name,
            last_name: rec.last_name,
            email: rec.email,
            password: rec.password,
            created_at: rec.created_at,
            updated_at: rec.updated_at,
        });
    }

    println!("{:#?}", users);
    
    Ok(())
}


