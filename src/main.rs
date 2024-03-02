use yew::prelude::*;
use yew_router::prelude::*;

mod hooks;
mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/auth/signup")]
    SignUp,
    #[at("/404")]
    #[not_found]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <pages::home::Home /> },
        Route::SignUp => html! { <pages::signup::SignUp /> },
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
