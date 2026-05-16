mod extractors;
mod routes;
mod services;
mod state;

use axum::{
    http::{HeaderValue, Method},
    Router,
};
use dotenvy::dotenv;
use std::{
    collections::HashMap,
    env,
    sync::{Arc, RwLock},
};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let frontend_url = env::var("FRONTEND_URL").unwrap_or("http://localhost:8080".to_string());
    let origin: HeaderValue = frontend_url.parse().unwrap();

    let db = services::database_service::database().await;

    let sessions = Arc::new(RwLock::new(HashMap::new()));
    let rooms = Arc::new(RwLock::new(HashMap::new()));
    let layer = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE])
        .allow_origin(origin);

    let state = crate::state::AppStruct {
        db: db.unwrap(),
        collection_name: env::var("USER_COLLECTION_NAME")
            .expect("Var: DATABASE_URL. Não encontrada"),
        sessions,
        rooms,
    };

    let app: Router = Router::new()
        .nest("/rooms", routes::game::create_room_router())
        .nest("/user", routes::user::create_user_router())
        .layer(layer)
        .with_state(state);

    let port = env::var("PORT").unwrap_or("3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap()
}
