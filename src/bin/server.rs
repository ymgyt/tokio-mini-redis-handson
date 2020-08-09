#![allow(dead_code)]

use mini_redis;

#[tokio::main]
pub async fn main() -> mini_redis::Result<()> {
    tracing_subscriber::fmt::try_init()?;

    let port = mini_redis::DEFAULT_PORT;

    let listener = tokio::net::TcpListener::bind(&format!("127.0.0.1:{}",port)).await?;

    mini_redis::server::run(listener, tokio::signal::ctrl_c()).await
}
