use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use std::net::TcpListener;

pub mod configuration;
pub mod routes;
pub mod startup;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}
