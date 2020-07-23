mod dal;
mod dao;
mod model;

use crate::dal::yarn_api::YarnApiImpl;
use crate::dao::clip_dao::{ClipDao, ClipDaoImpl};
use scraper::{Html, Selector};
use std::env;
use std::sync::Arc;
use telegram_bot::{
    Api, CanAnswerInlineQuery, CanSendMessage, InlineQueryResult, InlineQueryResultMpeg4Gif,
    MessageChat, ParseMode, Update, UpdateKind,
};
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() {
    let api = Api::new(env::var("TELEGRAM_API_KEY").unwrap());
    let yarn_api = Arc::new(YarnApiImpl::new("https://getyarn.io".to_string()));
    let clip_dao = Arc::new(ClipDaoImpl::new(yarn_api));

    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        match update {
            // Ok(Update {
            //     kind: UpdateKind::Message(message),
            //     ..
            // }) => {
            //     let mut text_message = message.chat.text(
            //         "This bot can help you find video clips. \
            //                 It works automatically, no need to add it anywhere. \
            //                 Simply open any of your chats and type `@vclipbot` something in the message field. \
            //                 Then tap on a result to send.\
            //                 \n\nFor example, try typing `@vclipbot goat` here."
            //     );
            //     text_message.parse_mode(ParseMode::Markdown);
            //
            //     let res = api.send(text_message).await;
            //     if let Err(err) = res {
            //         println!("error: {:?}", err);
            //     }
            // },
            Ok(Update {
                kind: UpdateKind::InlineQuery(inline_query),
                ..
            }) => {
                let clips = match clip_dao.get_clips(inline_query.query.clone()).await {
                    Ok(response) => response,
                    Err(err) => {
                        println!("error: {:?}", err);
                        return;
                    }
                };

                let mut query_results: Vec<InlineQueryResult> = Vec::new();

                for clip in clips {
                    let title_and_caption =
                        format!("{} - {}", clip.caption.trim(), clip.title.trim());
                    let query_result = InlineQueryResultMpeg4Gif {
                        id: clip.id.to_string(),
                        mpeg4_url: format!("https://y.yarn.co/{}_text.mp4", clip.id),
                        mpeg4_width: None,
                        mpeg4_height: None,
                        mpeg4_duration: None,
                        thumb_url: format!("https://y.yarn.co/{}_text.gif", clip.id),
                        title: Some(title_and_caption.to_string()),
                        caption: Some(title_and_caption.to_string()),
                        parse_mode: Some(ParseMode::Markdown),
                        reply_markup: None,
                        input_message_content: None,
                    };

                    query_results.push(InlineQueryResult::from(query_result));
                }

                let res = api.send(inline_query.answer(query_results)).await;
                if let Err(err) = res {
                    println!("error: {:?}", err);
                }
            }
            Err(err) => {
                println!("error: {:?}", err);
            }
            _ => {}
        }
    }
}
