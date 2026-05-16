use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};

use crate::state::AppStruct;

pub struct AuthenticatedUser {
    pub username: String,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    AppStruct: FromRef<S>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppStruct::from_ref(state);

        let cookie_header = parts
            .headers
            .get(axum::http::header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let session_id = cookie_header
            .split(';')
            .find(|s| s.contains("session_id="))
            .map(|s| s.replace("session_id=", "").trim().to_string())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let sessions = app_state.sessions.read().unwrap();
        let username = sessions
            .get(&session_id)
            .cloned()
            .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(AuthenticatedUser { username })
    }
}
