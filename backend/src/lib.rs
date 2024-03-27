use app_state::AppState;
use router::create_routes;

mod routes;
mod router;
pub mod app_state;

pub async fn run(app_state: AppState) {
    let app = create_routes(app_state).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
