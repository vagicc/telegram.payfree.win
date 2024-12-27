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
                    //消息只支持这些html：https://core.telegram.org/bots/api#formatting-options
                    let _ = bot
                        .send_message(chat_id, "
                        <b>粗体文本</b> 
                        <a href=\"http://www.example.com/\">inline URL</a>
                        <tg-emoji emoji-id=\"5368324170671202286\">👍</tg-emoji>
<code>inline fixed-width code</code>
<pre>pre-formatted fixed-width code block</pre>
<pre><code class=\"language-python\">pre-formatted fixed-width code block written in the Python programming language</code></pre>
<blockquote>Block quotation started\nBlock quotation continued\nThe last line of the block quotation</blockquote>
<blockquote expandable>Expandable block quotation started\nExpandable block quotation continued\nExpandable block quotation continued\nHidden by default part of the block quotation started\nExpandable block quotation continued\nThe last line of the block quotation</blockquote>

                        ")
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
