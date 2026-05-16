mod components;
mod config;
mod pages;
mod storage;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/lobbies")]
    Lobby,
    #[at("/game/:room_id")]
    Game { room_id: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <pages::login::Login /> },
        Route::Lobby => html! { <pages::lobby::Lobby /> },
        Route::Game { room_id } => html! { <pages::game::Game room_id={room_id} /> },
    }
}

#[component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
