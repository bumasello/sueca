use crate::components::req_button;
use yew::prelude::*;

#[component]
pub fn Login() -> Html {
    let username = use_state(|| String::from(""));

    let on_set_username = {
        let username = username.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                username.set(input.value());
            }
        })
    };

    html! {
        <div class="min-h-screen flex items-center justify-center bg-gray-950">
            <div class="bg-gray-900 p-8 rounded-xl shadow-sm w-full max-w-sm">
                <h1 class="text-2xl font-semibold text-gray-100 mb-6 text-center">
                    {if username.is_empty() {
                        "Sueca do Miciro".to_string()
                    } else {
                        format!("Olá {}", *username)
                    }}
                </h1>
                <label class="block text-sm font-medium text-gray-300 mb-1" for="username">
                    {"Usuário"}
                </label>
                <input
                    type="text"
                    name="username"
                    id="username"
                    value={(*username).clone()}
                    oninput={on_set_username}
                    class="w-full px-3 py-2 border border-gray-700 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-gray-600"
                />
                <req_button::ReqButton
                    text="Entrar"
                    username={(*username).clone()}
                    path="/user/login"
                />
            </div>
        </div>
    }
}
