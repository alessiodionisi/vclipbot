extern crate chrono;
extern crate fern;
extern crate actix_web;

mod bot;
mod dal;
mod dao;
mod model;

use crate::bot::clip_bot::TelegramClipBot;
use crate::bot::telegram_bot;
use crate::dal::yarn_api::YarnApiImpl;
use crate::dao::clip_dao::ClipDaoImpl;
use log::info;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    setup_logging();

    info!("loading clip bot configs");
    let token = env::var("TELEGRAM_API_KEY").expect("TELEGRAM_API_KEY not found");
    let yarn_api = Arc::new(YarnApiImpl::new("https://getyarn.io".to_string()));
    let clip_dao = Arc::new(ClipDaoImpl::new(yarn_api));
    let clip_bot = Arc::new(TelegramClipBot::new(token, clip_dao));

    info!("starting clip bot");
    telegram_bot::run_webhook(8888, clip_bot).await;
}

fn setup_logging() {
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("hyper", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}
