use crate::telegram::answer;
use crate::telegram::Command;
use teloxide::payloads::SendMessage;
use teloxide::{
    prelude::*,
    types::{Message, ParseMode, ReplyParameters},
};

use std::sync::Arc;
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

                    // 创建一个 ReplyParameters，用于引用原消息
                    let reply_parameters = ReplyParameters::new(message.id);

                    bot.send_message(message.chat.id, "111这是回复！")
                        .reply_parameters(reply_parameters)
                        .parse_mode(ParseMode::Html)
                        .await
                        .log_on_error()
                        .await;

                    let reply_params = ReplyParameters::new(message.id);

                    // 创建一个 SendMessage 请求，手动初始化所有字段
                    let send_message = SendMessage {
                        // chat_id: message.chat.id,                  // 必填字段：聊天 ID
                        chat_id: teloxide::types::Recipient::Id(message.chat.id),

                        text: "这是一个回复消息".to_string(), // 必填字段：消息文本
                        parse_mode: Some(ParseMode::Markdown), // 可选字段：Markdown 格式
                        reply_parameters: Some(reply_params), // 可选字段：引用原消息
                        message_thread_id: None,              // 可选字段：消息线程 ID
                        entities: None, // 可选字段：消息实体（如加粗、链接等）
                        link_preview_options: None, // 可选字段：链接预览选项
                        disable_notification: None, // 可选字段：禁用通知
                        protect_content: None, // 可选字段：保护内容
                        reply_markup: None, // 可选字段：自定义键盘或强制回复
                    };

                    // 发送消息
                    bot.send_message(
                        send_message.chat_id, // 必填字段：聊天 ID
                        send_message.text,    // 必填字段：消息文本
                    )
                    .parse_mode(send_message.parse_mode.unwrap_or(ParseMode::Markdown)) // 可选字段：Markdown 格式
                    .await
                    .log_on_error()
                    .await;

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
                    // bot.send_message(message.chat.id, "这是回复！")
                    //     .reply_to_message_id(message.id) // 正确用法
                    //     .send()
                    //     .await;

                    // 创建一个 SendMessage 请求

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
