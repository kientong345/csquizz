use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, Pool, Postgres, Transaction};

use crate::database::config;

#[derive(Clone)]
pub struct QuizBankPool {
    connection_pool: Pool<Postgres>,
}

impl QuizBankPool {
    /// Initializes the connection pool and runs any pending migrations
    pub async fn init() -> Self {
        let pool = Self {
            connection_pool: PgPoolOptions::new()
                .max_connections(20)
                .connect(&config::database_url())
                .await
                .expect("Failed to connect to quiz-bank database"),
        };

        pool.run_migrations().await;

        pool
    }

    async fn run_migrations(&self) {
        sqlx::migrate!("./migrations")
            .run(&self.connection_pool)
            .await
            .expect("cannot migrate database");
    }

    /// Gets a connection from the connection pool
    pub async fn get_connection(&self) -> Result<PoolConnection<Postgres>, sqlx::Error> {
        let connection = self.connection_pool.acquire().await?;

        Ok(connection)
    }

    /// Starts a new transaction
    pub async fn start_transaction(&self) -> Result<Transaction<'_, Postgres>, sqlx::Error> {
        let transaction = self.connection_pool.begin().await?;

        Ok(transaction)
    }
}
