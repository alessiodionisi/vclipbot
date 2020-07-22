use scraper::{Html, Selector};
use std::env;
use telegram_bot::{
    Api, CanAnswerInlineQuery, CanSendMessage, InlineQueryResult, InlineQueryResultMpeg4Gif,
    MessageChat, ParseMode, Update, UpdateKind,
};
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() {
    let api = Api::new(env::var("TELEGRAM_API_KEY").unwrap());
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
                let getyarn_url =
                    format!("https://getyarn.io/yarn-find?text={}", inline_query.query);

                let getyarn_response = match reqwest::get(&getyarn_url).await {
                    Ok(response) => response,
                    Err(err) => {
                        println!("error: {:?}", err);
                        return;
                    }
                };

                let getyarn_html = match getyarn_response.text().await {
                    Ok(html) => html,
                    Err(err) => {
                        println!("error: {:?}", err);
                        return;
                    }
                };

                let getyarn_document = Html::parse_document(&getyarn_html);

                let clip_wrap_selector = Selector::parse(".clip-wrap").unwrap();
                let title_selector = Selector::parse(".title").unwrap();
                let a_selector = Selector::parse("a").unwrap();

                let mut query_results: Vec<InlineQueryResult> = Vec::new();

                for clip_wrap in getyarn_document.select(&clip_wrap_selector) {
                    let mut clip_wrap_links = clip_wrap.select(&a_selector);
                    let first_link = clip_wrap_links.next().unwrap();
                    let first_link_href = first_link.value().attr("href").unwrap();
                    let second_link = clip_wrap_links.next().unwrap();

                    let clip_title = first_link
                        .select(&title_selector)
                        .next()
                        .unwrap()
                        .text()
                        .collect::<String>();
                    let clip_id = first_link_href.replace("/yarn-clip/", "");
                    let clip_text = second_link.text().collect::<String>();

                    let title_and_caption = format!("{} - {}", clip_text.trim(), clip_title.trim());

                    let query_result = InlineQueryResultMpeg4Gif {
                        id: clip_id.to_string(),
                        mpeg4_url: format!("https://y.yarn.co/{}.mp4", clip_id),
                        mpeg4_width: None,
                        mpeg4_height: None,
                        mpeg4_duration: None,
                        thumb_url: format!("https://y.yarn.co/{}_text_hi.gif", clip_id),
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
