use common::GameRoomStateStruct;
use gloo::{net::http::Request, timers::callback::Interval};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub room_id: String,
}

#[component]
pub fn Game(Props { room_id }: &Props) -> Html {
    let room_id_ini = room_id.clone();
    let room_id_int = room_id.clone();
    let game_room_state: UseStateHandle<Option<common::GameRoomStateStruct>> = use_state(|| None);
    let game_room_state_effect = game_room_state.clone();
    let nav = use_navigator().unwrap();

    use_effect_with((), move |_| {
        let room_id_ini_handler = room_id_ini.clone();
        let room_id_int_handler = room_id_int.clone();
        let state_ini = game_room_state_effect.clone();
        let state_int = game_room_state_effect.clone();

        spawn_local(async move {
            let mut req = Request::get(&format!(
                "{}/rooms/{}/state",
                crate::config::API_URL,
                room_id_ini_handler.clone()
            ));
            if let Some(token) = crate::storage::get_token() {
                req = req.header("Authorization", &format!("Bearer {}", token));
            }
            let resp = req.send().await.unwrap();

            if resp.ok() {
                let data = resp.json::<GameRoomStateStruct>().await.unwrap();
                state_ini.set(Some(data));
            }
        });

        let interval = Interval::new(2000, move || {
            let int_game_room_state_handler = state_int.clone();
            let room_id_int_handler = room_id_int_handler.clone();
            spawn_local(async move {
                let mut req = Request::get(&format!(
                    "{}/rooms/{}/state",
                    crate::config::API_URL,
                    room_id_int_handler.clone()
                ));
                if let Some(token) = crate::storage::get_token() {
                    req = req.header("Authorization", &format!("Bearer {}", token));
                }
                let resp = req.send().await.unwrap();

                if resp.ok() {
                    let data = resp.json::<GameRoomStateStruct>().await.unwrap();
                    int_game_room_state_handler.set(Some(data));
                }
            });
        });
        move || drop(interval)
    });

    let onclick = Callback::from(move |_: MouseEvent| {
        let nav_handle = nav.clone();

        nav_handle.push(&Route::Lobby);
    });

    html! {
        <>
            {match (*game_room_state).clone() {
                None => {
                    html! {
                        <div class="min-h-screen bg-gray-950 flex flex-col items-center justify-center gap-4">
                            <p class="text-gray-100 text-xl font-semibold">
                                {"Suequinha do Miciro"}
                            </p>
                            <p class="text-gray-400 text-sm">{"Aguardando jogadores..."}</p>
                        </div>
                    }
                }
                Some(state) => {
                    match state.status {
                        common::RoomStatus::Waiting => {

                            html! {
                                <div class="min-h-screen bg-gray-950 flex flex-col items-center justify-center gap-4">
                                    <p class="text-gray-100 text-xl font-semibold">
                                        {"Suequinha do Miciro"}
                                    </p>
                                    <p class="text-gray-400 text-sm">{"Aguardando jogadores..."}</p>
                                </div>
                            }
                        }
                        common::RoomStatus::Finished => {
                            let (w1, w2, score_w, score_l) = if state.scores[0] > state.scores[1] {
                                (
                                    &state.players[0],
                                    &state.players[2],
                                    state.scores[0],
                                    state.scores[1],
                                )
                            } else {
                                (
                                    &state.players[1],
                                    &state.players[3],
                                    state.scores[1],
                                    state.scores[0],
                                )
                            };
                            html! {
                                <div class="min-h-screen bg-gray-950 flex flex-col items-center justify-center">
                                    <div class="bg-gray-900 rounded-xl p-8 text-center max-w-sm w-full">
                                        <h2 class="text-gray-100 text-2xl font-semibold mb-6">
                                            {"Fim de Jogo!"}
                                        </h2>
                                        <p class="text-yellow-400 text-lg font-semibold mb-1">
                                            {format!("{} & {}", w1, w2)}
                                        </p>
                                        <p class="text-gray-400 text-sm mb-6">{"venceram"}</p>
                                        <p class="text-gray-300 text-sm">
                                            {format!("Placar: {} x {}", score_w, score_l)}
                                        </p>
                                        <button
                                            class="mt-8 w-full py-2 px-4 bg-gray-100 text-gray-900 rounded-lg text-sm hover:bg-gray-200 transition-colors"
                                            {onclick}
                                        >
                                            {"Voltar ao lobby"}
                                        </button>
                                    </div>
                                </div>
                            }
                        }
                        common::RoomStatus::InGame => {
                            let mut hand = state.hand.clone();
                            hand.sort_by_key(|c| {
                                let suit_order = match c.suit {
                                    common::Suit::Copas => 0,
                                    common::Suit::Espadas => 1,
                                    common::Suit::Ouros => 2,
                                    common::Suit::Paus => 3,
                                };
                                (suit_order, std::cmp::Reverse(c.strength()))
                            });
                            let my_idx = state
                                .players
                                .iter()
                                .position(|p| p == &state.player)
                                .unwrap_or(0);
                            let duo = &state.players[(my_idx + 2) % 4];
                            let left_opp = &state.players[(my_idx + 1) % 4];
                            let right_opp = &state.players[(my_idx + 3) % 4];
                            let card_of = |player: &str| {
                                state
                                    .current_trick
                                    .iter()
                                    .find(|(p, _)| p == player)
                                    .map(|(_, c)| *c)
                            };
                            let render_card = |card: Option<common::Card>| -> Html {
                                match card {
                                    None => {

                                        html! {
                                            <div class="w-10 h-14 bg-gray-700 rounded opacity-30"></div>
                                        }
                                    }
                                    Some(c) => {
                                        let (suit_symbol, color) = match c.suit {
                                            common::Suit::Copas => ("♥", "text-red-500"),
                                            common::Suit::Ouros => ("♦", "text-red-500"),
                                            common::Suit::Espadas => ("♠", "text-gray-900"),
                                            common::Suit::Paus => ("♣", "text-gray-900"),
                                        };
                                        let value_str = match c.value {
                                            1 => "A".to_string(),
                                            13 => "K".to_string(),
                                            12 => "Q".to_string(),
                                            11 => "J".to_string(),
                                            _ => c.value.to_string(),
                                        };
                                        html! {
                                            <div class="w-10 h-14 bg-white rounded flex flex-col items-center justify-center shadow">
                                                <span class={format!(
                                                    "text-sm font-bold {}",
                                                    color,
                                                )}>{value_str}</span>
                                                <span class={format!(
                                                    "text-lg font-bold {}",
                                                    color,
                                                )}>{suit_symbol}</span>
                                            </div>
                                        }
                                    }
                                }
                            };
                            html! {
                                <div class="min-h-screen bg-gray-950 flex flex-col  py-8 px-4">
                                    <div class="flex justify-between w-full max-w-sm mx-auto mb-4">
                                        <div class="text-center">
                                            <p class="text-gray-400 text-xs">
                                                {format!("{} & {}", state.players[0], state.players[2])}
                                            </p>
                                            <p class="text-gray-400 text-lg font-semibold">
                                                {state.scores[0]}
                                            </p>
                                        </div>
                                        <div class="text-center">
                                            <p class="text-xs text-yellow-400 font-semibold">{"vs"}</p>
                                        </div>
                                        <div class="text-center">
                                            <p class="text-gray-400 text-xs">
                                                {format!("{} & {}", state.players[1], state.players[3])}
                                            </p>
                                            <p class="text-gray-400 text-lg font-semibold">
                                                {state.scores[1]}
                                            </p>
                                        </div>
                                    </div>
                                    <div class="flex-1 flex flex-col items-center justify-center gap-6">
                                        <div class="text-center">
                                            <p class={if state.current_turn == *duo {
                                                "text-white text-sm font-semibold"
                                            } else {
                                                "text-gray-500 text-sm"
                                            }}>{duo}</p>
                                        </div>

                                        <div class="flex items-center gap-12">
                                            <div class="text-center w-24">
                                                <p class={if state.current_turn == *left_opp {
                                                    "text-white text-sm font-semibold"
                                                } else {
                                                    "text-gray-500 text-sm"
                                                }}>{left_opp}</p>
                                            </div>

                                            <div class="relative w-48 h-48 bg-green-900 rounded-full">
                                                <div class="absolute top-2 left-1/2 -translate-x-1/2 h-14 ">
                                                    {render_card(card_of(duo))}
                                                </div>
                                                <div class="absolute bottom-2 left-1/2 -translate-x-1/2 h-14 ">
                                                    {render_card(card_of(&state.player))}
                                                </div>
                                                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-center">
                                                    <p class="text-green-300 text-xs opacity-60">
                                                        {format!("{:?}", state.trump)}
                                                    </p>
                                                </div>
                                                <div class="absolute left-2 top-1/2 -translate-y-1/2 h-14 ">
                                                    {render_card(card_of(left_opp))}
                                                </div>
                                                <div class="absolute right-2 top-1/2 -translate-y-1/2 w-10 h-14 ">
                                                    {render_card(card_of(right_opp))}
                                                </div>
                                            </div>

                                            <div class="text-center w-24">
                                                <p class={if state.current_turn == *right_opp {
                                                    "text-white text-sm font-semibold"
                                                } else {
                                                    "text-gray-500 text-sm"
                                                }}>{right_opp}</p>
                                            </div>
                                        </div>

                                        <div class="text-center">
                                            <p class={if state.current_turn == state.player {
                                                "text-white text-sm font-semibold"
                                            } else {
                                                "text-gray-500 text-sm"
                                            }}>{&state.player}</p>
                                            <div class="flex gap-2 mt-4 flex-wrap justify-center">
                                                {hand
                                                    .iter()
                                                    .map(|c| {
                                                        let room_id = room_id.clone();
                                                        let card = *c;
                                                        let is_my_turn = state.player == state.current_turn;
                                                        let (suit_symbol, color) = match c.suit {
                                                            common::Suit::Copas => ("♥", "text-red-500"),
                                                            common::Suit::Ouros => ("♦", "text-red-500"),
                                                            common::Suit::Espadas => ("♠", "text-gray-900"),
                                                            common::Suit::Paus => ("♣", "text-gray-900"),
                                                        };
                                                        let value_str = match c.value {
                                                            1 => "A".to_string(),
                                                            13 => "K".to_string(),
                                                            12 => "Q".to_string(),
                                                            11 => "J".to_string(),
                                                            _ => c.value.to_string(),
                                                        };
                                                        let onclick = Callback::from(move |_: MouseEvent| {
                                                            let room_id = room_id.clone();
                                                            spawn_local(async move {
                                                                let mut builder = Request::post(&format!(
                                                                    "{}/rooms/{}/play",
                                                                    crate::config::API_URL,
                                                                    room_id,
                                                                ));
                                                                if let Some(token) = crate::storage::get_token() {
                                                                    builder = builder.header("Authorization", &format!("Bearer {}", token));
                                                                }
                                                                builder.json(&card).unwrap().send().await.unwrap();
                                                            });
                                                        });

                                                        html! {
                                                            <button
                                                                {onclick}
                                                                disabled={!is_my_turn}
                                                                class="w-12 h-18 bg-white rounded flex flex-col items-center justify-center shadow hover:shadow-lg transition-shadow disabled:opacity-40"
                                                            >
                                                                <span class={format!(
                                                                    "text-sm font-bold {}",
                                                                    color,
                                                                )}>{value_str}</span>
                                                                <span class={format!(
                                                                    "text-lg {}",
                                                                    color,
                                                                )}>{suit_symbol}</span>
                                                            </button>
                                                        }
                                                    })
                                                    .collect::<Html>()}
                                            </div>
                                        </div>

                                    </div>
                                </div>
                            }
                        }
                    }
                }
            }}
        </>
    }
}
