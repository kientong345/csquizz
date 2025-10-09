use std::net::SocketAddr;

use tokio::net::TcpListener;

use crate::database::pool::QuizBankPool;

mod app;
mod config;
mod database;
mod error;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let port = config::Configuration::get().port;
    let address = SocketAddr::from(([127, 0, 0, 1], port));

    let pool = QuizBankPool::init().await;
    let app = app::create_app(pool).await;
    let listener = TcpListener::bind(address)
        .await
        .expect("cannot bind address");

    axum::serve(listener, app)
        .await
        .expect("cannot serving app");
}
