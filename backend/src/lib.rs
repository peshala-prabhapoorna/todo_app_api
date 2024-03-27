use router::create_routes;

mod routes;
mod router;

pub async fn run() {
    let app = create_routes().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
