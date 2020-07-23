use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;
use telegram_bot::{InlineQuery, Update, UpdatesStream};
use tokio::stream::StreamExt;

#[async_trait]
pub trait TelegramBot: Send {
    fn updates(&self) -> UpdatesStream;
    async fn on_update(&self, update: Update) -> Result<(), Box<dyn Error>>;
    async fn on_inline_query(&self, inline_query: InlineQuery) -> Result<(), Box<dyn Error>>;
}

pub async fn run(bot: Arc<dyn TelegramBot>) {
    let mut stream = bot.updates();
    while let Some(update) = stream.next().await {
        match bot.on_update(update.unwrap()).await {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
