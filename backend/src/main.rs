use backend::{
    app_state::AppState, fetch_environment_variable, run, utilities::token_wrapper::TokenWrapper,
};
use dotenvy::dotenv;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = fetch_environment_variable("DATABASE_URL");
    let jwt_secret = fetch_environment_variable("JWT_SECRET");

    let db = match Database::connect(database_uri).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database {:?}", error);
            panic!()
        }
    };

    let app_state = AppState {
        db,
        jwt_secret: TokenWrapper(jwt_secret),
    };

    run(app_state).await;
}
