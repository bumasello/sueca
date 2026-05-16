use crate::components::join_room_button::JoinButton;
use common::RoomSummaryStruct;
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[component]
pub fn Lobby() -> Html {
    let rooms: UseStateHandle<Vec<RoomSummaryStruct>> = use_state(|| vec![]);
    let rooms_handle = rooms.clone();
    let rooms_create = rooms.clone();
    let refresh = use_state(|| 0u32);

    use_effect_with(*refresh, move |_| {
        let rooms_handler = rooms_handle.clone();
        spawn_local(async move {
            let resp = Request::get(&format!("{}/rooms", crate::config::API_URL))
                .credentials(web_sys::RequestCredentials::Include)
                .send()
                .await
                .unwrap();

            if resp.ok() {
                let mut data = resp.json::<Vec<RoomSummaryStruct>>().await.unwrap();
                data.sort_by_key(|r| r.created_at);
                rooms_handler.set(data);
            }
        });
    });

    let onclick = Callback::from(move |_: MouseEvent| {
        let rooms_create_handler = rooms_create.clone();
        let refresh_handler = refresh.clone();
        spawn_local(async move {
            let mut updated = (*rooms_create_handler).clone();
            let resp = Request::post(&format!("{}/rooms", crate::config::API_URL))
                .credentials(web_sys::RequestCredentials::Include)
                .send()
                .await
                .unwrap();

            if resp.ok() {
                let data = resp.json::<common::RoomSummaryStruct>().await.unwrap();
                updated.push(data);
                rooms_create_handler.set(updated);
                refresh_handler.set(*refresh_handler + 1)
            }
        });
    });

    html! {
        <div class="min-h-screen bg-gray-950 px-4 py-8">
            <div class="max-w-2xl mx-auto">
                <div class="flex items-center justify-between mb-6">
                    <h1 class="text-2xl font-semibold text-gray-100">{"Lobby"}</h1>
                    <button
                        class="px-4 py-2 bg-gray-100 text-gray-900 rounded-lg text-sm hover:bg-gray-200 transition-colors"
                        onclick={onclick}
                    >
                        {"Criar nova sala"}
                    </button>
                </div>
                <div>
                    {if rooms.is_empty() {
                        html! {
                            <p class="text-gray-500 text-center py-12">
                                {"Não existem salas no momento"}
                            </p>
                        }
                    } else {
                        (*rooms)
                            .iter()
                            .map(|r| {
                                html! {
                                    <div class="bg-gray-900 rounded-xl p-4 mb-3 flex items-center justify-between">
                                        <div>
                                            <p class="text-gray-100 text-sm font-mono">
                                                {&r.room_id[..8]}
                                            </p>
                                            <p class="text-gray-400 text-xs">
                                                {format!("{}/4 jogadores", r.players.len())}
                                            </p>
                                        </div>
                                        <JoinButton room_id={r.room_id.clone()} />
                                    </div>
                                }
                            })
                            .collect::<Html>()
                    }}
                </div>

            </div>
        </div>
    }
}
