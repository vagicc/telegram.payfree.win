use warp::Filter;

pub fn all_routes(
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let favicon = warp::get()
        .and(warp::path("favicon.ico"))
        .and(warp::path::end())
        .and(warp::fs::file("./static/favicon.ico"));

    //.well-known  此目录是申请免费https证书用到
    let well = warp::path(".well-known").and(warp::fs::dir("./static/.well-known"));
    //静态文件目录
    let dir = warp::path("static").and(warp::fs::dir("./static"));
    let hello = warp::path!("hello" / String).map(|name| format!("你好，{}!", name));
    let home = crate::routes::home_route::index();
    let telegram = crate::routes::telegram_r::telegram_webhook();

    let routes = home.or(favicon).or(well).or(dir).or(telegram).or(hello);
    routes
}
