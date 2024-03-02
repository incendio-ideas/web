use yew::{Callback, hook, use_effect, use_state, UseStateHandle};
use std::rc::Rc;
use crate::hooks::GraphqlResponse;

#[hook]
pub fn use_mutation<T: 'static + serde::de::DeserializeOwned,  Vars: serde::de::DeserializeOwned>(
    body: &str,
) -> (UseStateHandle<Option<T>>, Callback<Vars>) {
    let body = use_state(|| body.to_string());
    let state: UseStateHandle<Option<T>> = use_state(|| None::<T>);

    let controller = match web_sys::AbortController::new() {
        Ok(controller) => Rc::new(controller),
        Err(err) => {
            web_sys::console::error_1(&err.into());
            return (state, Callback::noop());
        }
    };

    let signal = use_state(|| controller.signal());

    use_effect(move || move || controller.abort());

    let mutate = {
        let state = state.clone();
        let signal = signal.clone();
        let body = body.clone();

        Callback::from(move |_| {
            let state = state.clone();
            let signal = signal.clone();
            let body = body.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let request = match gloo_net::http::Request::get("/api/v0/graphql")
                    .abort_signal(Some(&signal))
                    .header("Content-Type", "application/json")
                    .body(&*body)
                {
                    Ok(request) => request,
                    Err(err) => {
                        web_sys::console::error_1(&err.to_string().into());
                        return;
                    }
                };

                let response = match request.send().await {
                    Ok(response) => response,
                    Err(err) => {
                        web_sys::console::error_1(&err.to_string().into());
                        return;
                    }
                };

                let response: GraphqlResponse<T> = match response.json().await {
                    Ok(response_json) => response_json,
                    Err(err) => {
                        web_sys::console::error_1(&"Failed to parse response".into());
                        web_sys::console::error_1(&err.to_string().into());
                        return;
                    }
                };

                state.set(Some(response.data));
            });
        })
    };

    (state, mutate)
}
