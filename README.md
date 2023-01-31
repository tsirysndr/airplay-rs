# airplay-rs

A Rust library for discovering and controlling AirPlay devices.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
airplay = "0.1"
```

## Example

```rust
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
```

## License

MIT

## Contributing

1. Fork it
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create new Pull Request
  