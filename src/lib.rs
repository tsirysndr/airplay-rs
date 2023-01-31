pub mod client;
pub mod connection;
pub mod device;
pub mod discovery;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackInfo {
    pub ready_to_play: bool,
    pub duration: f64,
    pub position: f64,
}

pub enum SlideTransition {
    None,
    Dissolve,
    SlideLeft,
    SlideRight,
}
