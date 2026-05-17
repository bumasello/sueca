use crate::{extractors::auth::AuthenticatedUser, state::AppStruct};
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use mongodb::bson::{doc, Document, Uuid};
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct UserLogin {
    pub username: String,
}

#[derive(Serialize)]
struct LoginResponse {
    session_id: String,
}

pub fn create_user_router() -> Router<AppStruct> {
    Router::new()
        .route("/login", post(login))
        .route("/me", get(get_user))
        .route("/new_me", get(new_me))
}

async fn new_me(user: AuthenticatedUser) -> String {
    format!("Olá, {}", user.username)
}

async fn login(
    State(state): State<AppStruct>,
    Json(payload): Json<UserLogin>,
) -> impl IntoResponse {
    let col: Collection<Document> = state.db.collection(&state.collection_name);

    let filter_doc = doc! {"username": &payload.username};
    let existing_user = col.find_one(filter_doc.clone()).await.ok().flatten();

    match existing_user {
        Some(_) => {
            let session_id = Uuid::new().to_string();

            {
                let mut sessions = state.sessions.write().unwrap();
                sessions.insert(session_id.clone(), payload.username.clone());
            }

            (StatusCode::OK, Json(LoginResponse { session_id })).into_response()
        }
        None => match col
            .insert_one(doc! {"username": &payload.username,
            "sueca": {
                "games": 0,
                "wins": 0,
                "points": 0
            }})
            .await
        {
            Ok(_) => {
                let session_id = Uuid::new().to_string();
                {
                    let mut sessions = state.sessions.write().unwrap();
                    sessions.insert(session_id.clone(), payload.username.clone());
                }

                (StatusCode::OK, Json(LoginResponse { session_id })).into_response()
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    }
}

async fn get_user(State(state): State<AppStruct>, headers: HeaderMap) -> impl IntoResponse {
    let session_id = headers
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|c| c.split(";").find(|s| s.contains("session_id=")))
        .map(|s| s.replace("session_id=", "").trim().to_string());

    let session_id = match session_id {
        Some(id) => id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let sessions = state.sessions.read().unwrap();
    if let Some(username) = sessions.get(&session_id) {
        (StatusCode::OK, format!("Olá, {}", username)).into_response()
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
