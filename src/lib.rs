pub mod configuration;
pub mod routes;
pub mod startup;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}
