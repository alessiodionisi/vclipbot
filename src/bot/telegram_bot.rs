use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;
use telegram_bot::{InlineQuery, Update, UpdatesStream};
use futures::StreamExt;
use log::{error};

#[async_trait]
pub trait TelegramBot: Send {
    fn updates(&self) -> UpdatesStream;
    async fn on_update(&self, update: &Update) -> Result<(), Box<dyn Error>>;
    async fn on_inline_query(&self, inline_query: InlineQuery) -> Result<(), Box<dyn Error>>;
}

pub async fn run(concurrency: usize, bot: Arc<dyn TelegramBot>) {
    bot.updates().for_each_concurrent(concurrency, |maybe_update| {
        let bot = bot.clone();
        async move {
            match maybe_update {
                Ok(update) =>  match bot.on_update(&update).await {
                    Ok(_) => {}
                    Err(err) => error!("error processing update: {:?}, err: {}", update, err)
                }
                Err(err) => error!("error getting update: {}", err)
            }
        }
    }).await
}
