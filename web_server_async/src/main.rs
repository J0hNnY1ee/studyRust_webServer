use async_std::{net::*, task::spawn};
use futures::StreamExt;
use web_server_async::handle_connection;

#[async_std::main]
async fn main() {
    // 监听本地端口 7878 ，等待 TCP 连接的建立
    let listener = TcpListener::bind("0.0.0.0:7878").await.unwrap();

    listener
        .incoming()
        .for_each_concurrent(None, |tcpstream| async move {
            let tcpstream = tcpstream.unwrap();
            spawn( handle_connection(tcpstream));

        })
        .await;
}
