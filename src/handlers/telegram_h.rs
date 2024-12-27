use crate::telegram::answer;
use crate::telegram::Command;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::Update;
use teloxide::types::UpdateKind;
use teloxide::utils::command::BotCommands;
use warp::{Rejection, Reply};

pub async fn webhook(update: Update, bot: Arc<Bot>) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("访问了POST：“/telegram”");

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
