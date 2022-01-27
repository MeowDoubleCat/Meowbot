mod handler;

use handler::handler;
use std::env;
use teloxide::{prelude::*, types::ParseMode};

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting bot...");
    let bot_token = env::var("bot_token").expect("bot token undefined");
    let bot = Bot::new(bot_token).parse_mode(ParseMode::Html).auto_send();
    teloxide::repl(bot, handler).await;
}

#[tokio::main]
async fn main() {
    run().await;
}
