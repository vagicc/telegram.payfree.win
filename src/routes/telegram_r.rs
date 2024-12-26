use crate::handlers::telegram_h;
use warp::Filter;
use teloxide::prelude::*;
use std::sync::Arc;

// POST: /
pub fn telegram_webhook() -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    // 获取 Telegram Bot 的 token
    let token = std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not found in environment");
    // let token = env::var("TELEGRAM_BOT_TOKEN")
    //     .unwrap_or_else(|_| "7666999814:AAGamwcDTveGfRFIqOrqP06no-_mWaQk2Gg".to_string());

    // 创建一个 Arc 包裹的 Bot 实例，实现共享
    let bot = std::sync::Arc::new(Bot::new(token));
    let bot = warp::any().map(move || Arc::clone(&bot)); // 克隆 bot 以在闭包中使用

    let handle_webhook = warp::post()
        .and(warp::path("telegram"))
        .and(warp::body::json())
        .and(bot) // 使用克隆的 bot
        .and_then(telegram_h::webhook);
    handle_webhook
}
