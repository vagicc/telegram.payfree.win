use serde_json::value::Map;
use std::sync::Arc;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::types::Update;
use teloxide::types::UpdateKind;
use teloxide::utils::command::BotCommands;
use warp::{Rejection, Reply};

/* 响应： /check/link */
pub async fn new_html() -> std::result::Result<impl Reply, Rejection> {
    log::debug!("访问了：“/check/link”");

    let mut data = Map::new();
    // let html = to_html_single("reptile_new.html", data);

    let html = "kkads";
    Ok(warp::reply::html(html)) //直接返回html
                                // Err(warp::reject::not_found())   //错误的返回状态码
}

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "支持以下命令：")]
enum Command {
    #[command(description = "帮助命令.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description = "查询商户配置情")]
    Cmc,
}

async fn answer(bot: Arc<Bot>, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                .await?
        }
        Command::UsernameAndAge { username, age } => {
            let title = msg.chat.title();
            let k = msg.chat_id();

            bot.send_message(
                msg.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
        Command::Cmc => {
            bot.send_message(
                msg.chat.id,
                format!(" 查询商户配置情况  查看商户代收代付的配置情况"),
            )
            .await?
        }
        _ => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
    };

    Ok(())
}

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

    // Ok::<_, warp::Rejection>(warp::http::StatusCode::OK)

    // let mut data = Map::new();
    // let html = to_html_single("reptile_new.html", data);

    let html = "kkads";
    Ok(warp::reply::html(html)) //直接返回html
                                // Err(warp::reject::not_found())   //错误的返回状态码
}
