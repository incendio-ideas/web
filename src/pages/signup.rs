use web_sys::HtmlFormElement;
use yew::prelude::*;
use crate::hooks::use_mutation::use_mutation;

#[derive(serde::Deserialize)]
struct RegisterResponse {
    login: String,
}

#[derive(serde::Deserialize)]
struct RegisterVariables {
    email: String,
    password: String,
}

#[function_component]
pub fn SignUp() -> Html {
    let (state, mutate)= use_mutation::<RegisterResponse, RegisterVariables>(r#"{"query":"mutation { register }"}"#);

    let submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let form = match event.target_dyn_into::<HtmlFormElement>() {
            Some(target) => target,
            None => return,
        };

        let form_data = match web_sys::FormData::new_with_form(&form) {
            Ok(data) => data,
            Err(_) => return,
        };

        let email = match form_data.get("email").as_string() {
            Some(email) => email,
            None => return,
        };

        let password = match form_data.get("password").as_string() {
            Some(password) => password,
            None => return,
        };

        mutate.emit(RegisterVariables {
            email: email,
            password: password,
        });
    });

    html! {
        <form onsubmit={submit}>
            <input type="email" name="email" placeholder="Email" />
            <input type="password" name="password" placeholder="Password" />
            <button>{ "Sign up" }</button>
        </form>
    }
}
