use std::net::SocketAddr;

use quiz_bank::{app, config, database::pool::QuizBankPool};
use tokio::net::TcpListener;

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
