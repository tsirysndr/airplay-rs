use airplay::discovery::{discover, AIR_PLAY_SERVICE_NAME, AIR_TUNES_SERVICE_NAME};
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let devices = discover(AIR_PLAY_SERVICE_NAME);
    tokio::pin!(devices);
    while let Some(device) = devices.next().await {
        println!("Found device: {}", device.get_fullname());
    }
}
