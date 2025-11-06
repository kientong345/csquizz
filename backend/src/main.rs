use std::{net::SocketAddr, sync::Arc};

use csquizz::{
    app::{self, AppState},
    config::Configuration,
    database::pool::QuizBankPool,
};
use tokio::{net::TcpListener, sync::RwLock};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Load configuration
    let config = Configuration::get();

    // Start server
    let address = SocketAddr::from(([127, 0, 0, 1], config.app_config.port));
    let listener = TcpListener::bind(address)
        .await
        .expect("cannot bind address");

    // Initialize application state
    let pool = QuizBankPool::init(&config.db_config).await;
    let app_state = Arc::new(RwLock::new(AppState { pool, config }));

    // Create app
    let app = app::create_app(app_state).await;

    // Serve app
    axum::serve(listener, app)
        .await
        .expect("cannot serving app");
}
