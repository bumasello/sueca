use std::collections::HashMap;

use axum::{
    extract::Path,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use common::{Card, GameRoomStateStruct, RoomStatus, RoomSummaryStruct, Suit};
use mongodb::bson::Uuid;

use crate::{
    extractors::auth::AuthenticatedUser,
    state::{AppStruct, RoomStruct},
};

pub fn create_room_router() -> Router<AppStruct> {
    Router::new()
        .route("/", post(create_room))
        .route("/", get(list_rooms))
        .route("/{id}/join", post(join_room))
        .route("/{id}/state", get(room_state))
        .route("/{id}/play", post(play_card))
}

async fn create_room(State(state): State<AppStruct>) -> impl IntoResponse {
    let room_id = Uuid::new().to_string();

    let new_game = RoomStruct {
        room_id: room_id.clone(),
        players: vec![],
        status: RoomStatus::Waiting,
        deck: Card::return_shuffled_deck(),
        trump: Suit::get_trump(),
        turn_index: 0,
        starter_index: 0,
        current_trick: vec![],
        hands: HashMap::new(),
        scores: [0, 0],
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
    };

    {
        let mut rooms = state.rooms.write().unwrap();
        rooms.insert(room_id.clone(), new_game.clone());
    }

    (StatusCode::CREATED, Json(new_game.clone())).into_response()
}

async fn list_rooms(State(state): State<AppStruct>, user: AuthenticatedUser) -> impl IntoResponse {
    let user = user.username.clone();
    if user.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            "Você precisa estar logado para ver as salas",
        )
            .into_response();
    }
    let rooms = state.rooms.read().unwrap();

    let active_rooms: Vec<RoomSummaryStruct> = rooms
        .values()
        .filter(|r| r.status == RoomStatus::Waiting)
        .map(|r| RoomSummaryStruct {
            room_id: r.room_id.clone(),
            players: r.players.clone(),
            status: r.status.clone(),
            created_at: r.created_at,
        })
        .collect();

    (StatusCode::OK, Json(active_rooms)).into_response()
}

async fn join_room(
    State(state): State<AppStruct>,
    Path(id_room): Path<String>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    let user = user.username;
    let mut rooms = state.rooms.write().unwrap();
    if let Some(room) = rooms.get_mut(&id_room) {
        if room.status != RoomStatus::Waiting {
            return (
                StatusCode::BAD_REQUEST,
                "A partida já iniciou ou foi concluída.",
            )
                .into_response();
        }

        if room.players.contains(&user) {
            return (StatusCode::OK, "Você já está na sala").into_response();
        }

        room.players.push(user);

        if room.players.len() >= 4 {
            room.status = RoomStatus::InGame;

            let hands_vec = common::Card::distribute_cards(room.deck.clone(), room.players.clone());

            room.hands = hands_vec.into_iter().collect();

            room.deck.clear();
        }

        (
            StatusCode::OK,
            format!("Usuário adicionado a sala {}", id_room),
        )
            .into_response()
    } else {
        (StatusCode::NOT_FOUND, "Sala não encontrada").into_response()
    }
}

async fn room_state(
    State(state): State<AppStruct>,
    Path(id_room): Path<String>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    let rooms = state.rooms.read().unwrap();

    if let Some(room) = rooms.get(&id_room) {
        if room.status == RoomStatus::Waiting {
            return (StatusCode::BAD_REQUEST, Json("Sala não iniciada")).into_response();
        }

        let game_status = GameRoomStateStruct {
            player: user.username.clone(),
            status: room.status.clone(),
            trump: room.trump,
            hand: room.hands.get(&user.username).cloned().unwrap_or_default(),
            current_trick: room.current_trick.clone(),
            lead: room.current_trick.first().map(|(_, c)| c.suit),
            scores: room.scores,
            current_turn: room.players[room.turn_index].clone(),
            players: room.players.clone(),
        };

        (StatusCode::OK, Json(game_status)).into_response()
    } else {
        (StatusCode::NOT_FOUND, Json("Sala não encontrada")).into_response()
    }
}

async fn play_card(
    State(state): State<AppStruct>,
    Path(id_room): Path<String>,
    user: AuthenticatedUser,
    Json(card): Json<Card>,
) -> impl IntoResponse {
    let mut rooms = state.rooms.write().unwrap();

    let room = match rooms.get_mut(&id_room) {
        Some(r) => r,
        None => return (StatusCode::BAD_REQUEST, "Sala não existe").into_response(),
    };

    match room.process_move(user.username, card) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}
