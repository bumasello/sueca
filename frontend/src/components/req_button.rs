use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub path: String,
    pub text: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct PayloadUsername {
    pub username: String,
}

#[component]
pub fn ReqButton(
    Props {
        path,
        text,
        username,
    }: &Props,
) -> Html {
    let nav = use_navigator().unwrap();
    let errors = use_state(|| Vec::new());
    let errors_handle = errors.clone();
    let username = (*username).clone();
    let path = path.clone();
    let onclick = Callback::from(move |_: MouseEvent| {
        let username = (username).clone();
        let path = path.clone();
        let errors = errors_handle.clone();
        let nav_handle = nav.clone();
        spawn_local(async move {
            let payload = PayloadUsername {
                username: username.clone(),
            };

            if payload.username.is_empty() {
                errors.set(vec!["Username precisa estar preenchido".to_string()]);
                return;
            }
            Request::post(&format!("{}{}", crate::config::API_URL, path))
                .credentials(web_sys::RequestCredentials::Include)
                .json(&payload)
                .unwrap()
                .send()
                .await
                .unwrap();

            nav_handle.push(&Route::Lobby);
        });
    });

    html! {
        <div>
            <button
                class="w-full mt-4 py-2 px-4 bg-gray-100 text-gray-900 rounded-lg text-sm hover:bg-gray-200 transition-colors"
                {onclick}
            >
                {format!("{}", &text)}
            </button>
            {(*errors)
                .iter()
                .map(|e| html! { <span class="text-red-500 text-xs mt-2 block">{e}</span> })
                .collect::<Html>()}
        </div>
    }
}
