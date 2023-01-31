use async_stream::stream;
use futures_util::Stream;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};

pub const AIR_PLAY_SERVICE_NAME: &str = "_airplay._tcp.local.";
pub const AIR_TUNES_SERVICE_NAME: &str = "_raop._tcp.local.";

pub fn discover(service_name: &str) -> impl Stream<Item = ServiceInfo> {
    let mdns = ServiceDaemon::new().unwrap();
    let receiver = mdns.browse(service_name).expect("Failed to browse");
    stream! {
        while let Ok(event) = receiver.recv() {
          match event {
              ServiceEvent::ServiceResolved(info) => {
                  yield info;
              }
              _ => {}
          }
      }
    }
}
