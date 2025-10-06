use crate::database::pool::QuizBankPool;

mod database;
mod models;

#[tokio::main]
async fn main() {
    let pool = QuizBankPool::init().await;
    println!("Hello, world!");
}
