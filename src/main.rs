use std::env;
use std::sync::Arc;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::types::Update;
use teloxide::types::UpdateKind;
use teloxide::utils::command::BotCommands;

use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("TELEGRAM自动加税开始，k Starting command bot...");
    // 获取 Telegram Bot 的 token
    // let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not found in environment");
    let token = env::var("TELEGRAM_BOT_TOKEN")
        .unwrap_or_else(|_| "7666999814:AAGamwcDTveGfRFIqOrqP06no-_mWaQk2Gg".to_string());

    // 创建一个 Arc 包裹的 Bot 实例，实现共享
    let bot = Arc::new(Bot::new(token));
    // let bot = Arc::new(Bot::from_env());  //export TELOXIDE_TOKEN=<Your token here>

    // 定义一个处理更新的异步函数
    async fn handle_update(
        bot: Arc<Bot>,
        update: Update,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Command::repl(bot, answer).await;

        if let UpdateKind::Message(message) = update.kind {
            if let Some(text) = message.text() {
                match Command::parse(text, "") {
                    Ok(cmd) => {
                        if let Err(e) = answer(bot, message, cmd).await {
                            log::error!("Error answering message: {:?}", e);
                        }
                    }
                    Err(_) => {
                        // Handle the case where the command is not recognized
                        log::info!("Command not recognized");
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

        Ok(())
    }

    let bot = warp::any().map(move || Arc::clone(&bot)); // 克隆 bot 以在闭包中使用

    //post: https://telegram.payfree.win/telegram
    let handle_webhook = warp::post()
        .and(warp::path("telegram"))
        .and(warp::body::json())
        .and(bot) // 使用克隆的 bot
        .and_then(|update: Update, bot: Arc<Bot>| async move {
            log::info!("start=========");
            let _ = handle_update(bot, update).await;
            log::info!("end=========");

            Ok::<_, warp::Rejection>(warp::http::StatusCode::OK)
        });

    // 启动 warp 服务器
    warp::serve(handle_webhook.or(index()))
        .run(([0, 0, 0, 0], 8080))
        .await;
}

/// GET: /demo/redirect
pub fn index() -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::get()
        .and(warp::path("demo"))
        .and(warp::path("redirect"))
        .and(warp::path("v"))
        .and(warp::path::end())
        .map(|| "这里是V".to_string())
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "支持以下命令：")]
enum Command {
    #[command(description = "帮助命令.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
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
    };

    Ok(())
}
