use crate::telegram::answer;
use crate::telegram::Command;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::types::Update;
use teloxide::types::UpdateKind;
use teloxide::utils::command::BotCommands;
use warp::{Rejection, Reply};

pub async fn webhook(update: Update, bot: Arc<Bot>) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("访问了POST：“/telegram”");
    log::info!("update:{:#?}", update);
    log::info!("bot:{:#?}", bot);

    if let UpdateKind::Message(message) = update.kind {
        if let Some(text) = message.text() {
            log::error!("用户发送的内容：{:#}", text);
            match Command::parse(text, "") {
                Ok(cmd) => {
                    log::warn!("接收到的命令为：{:#?}", cmd);
                    if let Err(e) = answer(bot, message, cmd).await {
                        log::error!("Error answering message: {:?}", e);
                    }
                }
                Err(error) => {
                    // Handle the case where the command is not recognized
                    log::error!("这里接收到的是未定义的命令：{:#?}", error);
                    log::info!("Command not recognized");

                    let title = message.chat.title();
                    log::warn!("聊天标题：{:#?}", title);
                    let chat_id = message.chat.id;
                    log::warn!("聊天ID：{:#?}", chat_id);

                    let _ = bot.send_dice(message.chat.id).await;
                    let _ = bot
                        .send_message(chat_id, "<b>粗体文本</b><span style=\"color: #ff0000;\">这是红色文本</span>")
                        .parse_mode(ParseMode::Html)
                        .await
                        .log_on_error()
                        .await;
                    let _k = bot
                        .send_message(message.chat.id, Command::descriptions().to_string())
                        .await;
                }
            }
            // let kd=message.chat.id;``

            // Command::repl(bot, answer).await;
            // 处理文本消息
            // bot.send_message(message.chat.id, format!("你发送了：{}", text))
            //     .await?;

            // Command::repl(bot, answer).await;
        }
    }

    Ok::<_, warp::Rejection>(warp::http::StatusCode::OK)

    // let mut data = Map::new();
    // let html = to_html_single("reptile_new.html", data);

    // let html = "kkads";
    // Ok(warp::reply::html(html)) //直接返回html
    // Err(warp::reject::not_found())   //错误的返回状态码
}
