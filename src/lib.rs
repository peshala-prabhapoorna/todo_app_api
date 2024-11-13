use app_state::AppState;
use router::create_routes;

pub mod app_state;
mod database;
mod middleware;
mod queries;
mod router;
mod routes;
pub mod utilities;

pub async fn run(app_state: AppState) {
    let app = create_routes(app_state).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn fetch_environment_variable(key: &str) -> String {
    std::env::var(key)
        .map_err(|error| {
            eprintln!("Error loading environment variables. Error: {:?}", error);
            panic!()
        })
        .unwrap()
}
