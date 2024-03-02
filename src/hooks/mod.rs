pub mod use_query;
pub mod use_mutation;

#[derive(serde::Deserialize)]
struct GraphqlResponse<T> {
    data: T,
}
