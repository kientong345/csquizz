use std::{net::SocketAddr, sync::Arc};

use csquizz::{
    app::{self, AppState},
    config::Configuration,
    database::{non_persistent::SecondaryDatabase, persistent::PrimaryDatabase},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Load configuration
    let config = Arc::new(Configuration::get());

    // Start server
    #[cfg(feature = "local")]
    let address = SocketAddr::from(([127, 0, 0, 1], config.app_config.port));

    #[cfg(not(feature = "local"))]
    let address = SocketAddr::from(([0, 0, 0, 0], config.app_config.port));

    let listener = TcpListener::bind(address)
        .await
        .expect("cannot bind address");

    // Initialize application state
    let primary_db = PrimaryDatabase::init(&config.db_config).await;
    let secondary_db = SecondaryDatabase::init(&config.cache_config).ok();
    let app_state = AppState {
        primary_db,
        secondary_db,
        config,
    };

    // Create app
    let app = app::create_app(app_state).await;

    // Serve app
    axum::serve(listener, app)
        .await
        .expect("cannot serving app");
}
