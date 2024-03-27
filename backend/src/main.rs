use backend::{app_state::AppState, run};
use dotenvy::dotenv;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = std::env::var("DATABASE_URL")
        .map_err(|error| {
            eprintln!("Error loading environment variables. Error: {:?}", error);
            panic!()
        })
        .unwrap();

    let db = match Database::connect(database_uri).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database {:?}", error);
            panic!()
        }
    };

    let app_state = AppState {
        db,
    };

    run(app_state).await;
}
