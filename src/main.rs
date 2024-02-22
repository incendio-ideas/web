use yew::prelude::*;
use yew_router::prelude::*;

mod hooks;

use hooks::use_query::use_query;

#[derive(serde::Deserialize)]
struct LoginResponse {
    login: String,
}

#[function_component]
fn Home() -> Html {
    let state: UseStateHandle<Option<LoginResponse>> =
        use_query::<LoginResponse>(r#"{"query":"{ login }"}"#);

    let token = state
        .as_ref()
        .map(|response| response.login.as_str())
        .unwrap_or("loading...");

    html! {
        <>
            <h1>{ "Home" }</h1>
            <p>{ token }</p>
        </>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/404")]
    #[not_found]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <h1>{ "Not Found" }</h1> },
    }
}

#[function_component]
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
