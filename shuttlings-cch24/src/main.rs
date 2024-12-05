use axum::{extract::Query, http::StatusCode, response::Redirect, routing::get, Router};
use serde::Deserialize;

#[derive(Deserialize, Default)]
struct FromKey {
    from: String,
    key: String
}
#[derive(Deserialize, Default)]
struct FromTo {
    from: String,
    to: String
}

async fn hello_world() -> &'static str {
    "Hello, bird!"
}
async fn redirect() -> (StatusCode, Redirect) {
    (
        StatusCode::FOUND, 
        Redirect::to("https://www.youtube.com/watch?v=9Gc4QTqslN4")
    )
}
async fn calcu_ip(Query(params): Query<FromKey>) -> String {
    let from: Vec<u16> = params.from.split('.').map(|x| x.parse::<u16>().unwrap_or(0)).collect();
    let key: Vec<u16> = params.key.split('.').map(|x| x.parse::<u16>().unwrap_or(0)).collect();
    let dest: Vec<u16> = from.into_iter().zip(key.into_iter()).map(|(f, k)| (f+k)%256).collect();
    let return_dest = format!("{}.{}.{}.{}", dest[0], dest[1], dest[2], dest[3]);
    return_dest
}
async fn calcu_key(Query(params): Query<FromTo>) -> String {
    let from: Vec<u16> = params.from.split('.').map(|x| x.parse::<u16>().unwrap_or(0)).collect();
    let to: Vec<u16> = params.to.split('.').map(|x| x.parse::<u16>().unwrap_or(0)).collect();
    let key: Vec<u16> = from.into_iter().zip(to.into_iter()).map(|(f, t)| (t+256-f)%256).collect();
    let return_key = format!("{}.{}.{}.{}", key[0], key[1], key[2], key[3]);
    return_key
}
async fn calcuv6_ip(Query(params): Query<FromKey>) -> String {
    let from: Vec<u16> = params.from.split(':').map(|x| x.parse::<u16>().unwrap_or(0)).collect();
    let to: Vec<u16> = params.to.split(':').map(|x| x.parse::<u16>().unwrap_or(0)).collect();

    from[0]
}
async fn calcuv6_key(Query(params): Query<FromKey>) -> String {

}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/seek", get(redirect))
        .route("/-1/seek2", get(|| async { Redirect::permanent("https://www.youtube.com/watch?v=9Gc4QTqslN4") }))
        .route("/2/dest" ,get(calcu_ip))
        .route("/2/key" ,get(calcu_key))
        .route("/2/v6/dest" ,get(calcuv6_ip))
        .route("/2/v6/key" ,get(calcuv6_key));

    Ok(router.into())
}
