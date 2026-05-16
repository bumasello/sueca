use gloo::net::http::Request;
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
pub fn JoinButton(Props { room_id }: &Props) -> Html {
    let room_id = room_id.clone();
    let nav = use_navigator().unwrap();

    let onclick = Callback::from(move |_: MouseEvent| {
        let room_id = room_id.clone();
        let nav_handle = nav.clone();
        spawn_local(async move {
            let resp = Request::post(&format!(
                "{}/rooms/{}/join",
                crate::config::API_URL,
                &room_id
            ))
            .credentials(web_sys::RequestCredentials::Include)
            .send()
            .await
            .unwrap();

            if resp.ok() {
                nav_handle.push(&Route::Game { room_id });
            }
        });
    });

    html! {
        <div>
            <button
                class="px-4 py-2 bg-gray-100 text-gray-900 rounded-lg text-sm hover:bg-gray-200 transition-colors"
                {onclick}
            >
                {"Entrar na Sala"}
            </button>
        </div>
    }
}
