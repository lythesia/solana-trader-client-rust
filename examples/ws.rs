use futures_util::StreamExt;
use solana_trader_client_rust::connections::ws::WS;
use solana_trader_proto::api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ws = WS::new(Some("ws://localhost:8765".into())).await?;
    let mut stream = ws
        .stream_proto::<_, String>("noop", &api::GetBlockStreamRequest {})
        .await?;

    loop {
        match stream.next().await {
            Some(Ok(s)) => println!("got: {}", s),
            Some(Err(e)) => {
                eprintln!("{}", e);
                break;
            }
            None => {
                eprintln!("Stream ended");
                break;
            }
        }
    }
    Ok(())
}
