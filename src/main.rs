mod services;
mod config;

use teloxide::prelude::*;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    services::commands::Command::repl(bot, services::commands::answer).await;
}