use crate::hooks::use_query::use_query;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(serde::Deserialize)]
struct LoginResponse {
    login: String,
}

#[function_component]
pub fn Home() -> Html {
    let state: UseStateHandle<Option<LoginResponse>> =
        use_query::<LoginResponse>(r#"{"query":"{ register }"}"#);

    let token = state
        .as_ref()
        .map(|response| response.login.as_str())
        .unwrap_or("loading...");

    html! {
        <>
            <h1>{ "Home" }</h1>
            <Link<Route> to={Route::SignUp}>{ "Sign Up" }</Link<Route>>
            <p>{ token }</p>
        </>
    }
}
