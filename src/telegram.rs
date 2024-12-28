use std::sync::Arc;
use teloxide::prelude::*;
// use teloxide::types::ParseMode;
// use teloxide::types::Update;
// use teloxide::types::UpdateKind;
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "支持以下命令：")]
pub enum Command {
    #[command(description = "帮助命令.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description = "查询商户配置情")]
    Cmc,
}

pub async fn answer(bot: Arc<Bot>, msg: Message, cmd: Command) -> ResponseResult<()> {
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
            bot.send_message(
                msg.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
        Command::Cmc => {
            let k = bot
                .send_message(
                    msg.chat.id,
                    format!(" 查询商户配置情况  查看商户代收代付的配置情况"),
                )
                .await?;
            k
        }
    };

    Ok(())
}
