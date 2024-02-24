use yew::{hook, use_effect_with, use_state, UseStateHandle};

#[derive(serde::Deserialize)]
struct GraphqlResponse<T> {
    data: T,
}

#[hook]
pub fn use_query<T: 'static + serde::de::DeserializeOwned>(
    body: &str,
) -> UseStateHandle<Option<T>> {
    let state: UseStateHandle<Option<T>> = use_state(|| None::<T>);
    let controller = web_sys::AbortController::new().unwrap();
    let signal = controller.signal();

    let request_builder = match gloo_net::http::Request::post("/api/v0/graphql")
        .abort_signal(Some(&signal))
        .header("Content-Type", "application/json")
        .body(body)
    {
        Ok(request_builder) => request_builder,
        Err(err) => {
            web_sys::console::error_1(&err.to_string().into());
            return state;
        }
    };

    {
        let state = state.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let response = match request_builder.send().await {
                    Ok(response) => response,
                    Err(err) => {
                        web_sys::console::error_1(&err.to_string().into());
                        return;
                    }
                };

                let response: GraphqlResponse<T> = match response.json().await {
                    Ok(response_json) => response_json,
                    Err(_) => {
                        web_sys::console::error_1(&"Failed to parse response".into());
                        return;
                    }
                };

                state.set(Some(response.data));
            });

            move || controller.abort()
        });
    }

    state
}
