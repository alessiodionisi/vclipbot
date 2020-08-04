use crate::bot::telegram_bot::TelegramBot;
use crate::dao::clip_dao::ClipDao;
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;
use telegram_bot::{
    Api, CanAnswerInlineQuery, InlineQuery, InlineQueryResult, InlineQueryResultMpeg4Gif,
    ParseMode, Update, UpdateKind, UpdatesStream,
};
use log::info;

pub struct TelegramClipBot {
    api: Api,
    clip_dao: Arc<dyn ClipDao>,
}

#[async_trait]
impl TelegramBot for TelegramClipBot {
    fn updates(&self) -> UpdatesStream {
        self.api.stream()
    }

    async fn on_update(&self, update: &Update) -> Result<(), Box<dyn Error>> {
        if let Update {
            kind: UpdateKind::InlineQuery(inline_query),
            ..
        } = update
        {
            self.on_inline_query(inline_query.clone()).await
        } else {
            Ok(())
        }
    }

    async fn on_inline_query(&self, inline_query: InlineQuery) -> Result<(), Box<dyn Error>> {
        let clips = self.clip_dao.query(inline_query.query.as_str()).await?;
        info!("loaded {:?} clips for query: {:?}", clips, inline_query.query.as_str());

        let mut query_results: Vec<InlineQueryResult> = Vec::new();

        for clip in clips {
            let title_and_caption = format!("{} - {}", clip.caption.trim(), clip.title.trim());
            let query_result = InlineQueryResultMpeg4Gif {
                id: clip.id.to_string(),
                mpeg4_url: clip.mp4_link,
                mpeg4_width: None,
                mpeg4_height: None,
                mpeg4_duration: None,
                thumb_url: clip.gif_link,
                title: Some(title_and_caption.to_string()),
                caption: Some(title_and_caption.to_string()),
                parse_mode: Some(ParseMode::Markdown),
                reply_markup: None,
                input_message_content: None,
            };

            query_results.push(InlineQueryResult::from(query_result));
        }

        self.api.send(inline_query.answer(query_results)).await?;
        Ok(())
    }
}

impl TelegramClipBot {
    pub fn new(token: String, clip_dao: Arc<dyn ClipDao>) -> TelegramClipBot {
        let api = Api::new(token);
        TelegramClipBot { api, clip_dao }
    }
}
