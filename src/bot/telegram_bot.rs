use async_trait::async_trait;
use futures::StreamExt;
use log::error;
use std::error::Error;
use std::sync::Arc;
use telegram_bot::{InlineQuery, Update, UpdatesStream};
use actix_web::{post, HttpServer, HttpResponse, web, App};

#[async_trait]
pub trait TelegramBot: Send + Sync {
    fn updates(&self) -> UpdatesStream;
    async fn on_update(&self, update: &Update) -> Result<(), Box<dyn Error>>;
    async fn on_inline_query(&self, inline_query: InlineQuery) -> Result<(), Box<dyn Error>>;
}

pub async fn run_polling(concurrency: usize, bot: Arc<dyn TelegramBot>) {
    bot.updates()
        .for_each_concurrent(concurrency, |maybe_update| {
            let bot = bot.clone();
            async move {
                match maybe_update {
                    Ok(update) => match bot.on_update(&update).await {
                        Ok(_) => {}
                        Err(err) => error!("error processing update: {:?}, err: {}", update, err),
                    },
                    Err(err) => error!("error getting update: {}", err),
                }
            }
        })
        .await
}

#[post("/updates")]
async fn on_update(update: web::Json<Update>, bot: web::Data<Arc<dyn TelegramBot>>) -> std::io::Result<HttpResponse> {
    match bot.on_update(&update).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => {
            error!("error processing update: {:?}, err: {}", update, err);
            Ok(HttpResponse::InternalServerError().finish())
        },
    }
}

pub async fn run_webhook(port: i64, bot: Arc<dyn TelegramBot>) -> std::io::Result<()> {
    let bot = bot.clone();
    HttpServer::new(move|| {
        App::new()
            .data(bot.clone())
            .service(on_update)
    })
        .bind(format!("127.0.0.1:{}", port).as_str())?
        .run()
        .await
}
