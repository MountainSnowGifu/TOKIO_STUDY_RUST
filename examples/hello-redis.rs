use bytes::Bytes;
use mini_redis::{client, Result};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tracing::info;
use tracing_subscriber;

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Vec<u8>,
        resp: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // let number_of_yaks = 3;
    // info!(number_of_yaks, "preparing to shave yaks");

    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    // タスク1は "get" を、タスク2は "set" を担当する
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };

        tx.send(cmd).await.unwrap();
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "hello".to_string(),
            val: b"world".to_vec(),
            resp: resp_tx,
        };

        tx2.send(cmd).await.unwrap();
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let manager = tokio::spawn(async move {
        // サーバーへのコネクションを確立する
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // メッセージの受信を開始
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // エラーは無視する
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val.into()).await;
                    // エラーは無視する
                    let _ = resp.send(res);
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
