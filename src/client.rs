use serde::Serialize;
use std::time::Duration;
use surf::{Config, Error, Url};

#[derive(Serialize)]
struct RateQuery {
    value: f64,
}

#[derive(Serialize)]
struct ScrubQuery {
    position: f64,
}

#[derive(Serialize)]
struct VolumeQuery {
    value: f64,
}

pub struct Params {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
}

pub struct Client {
    http_client: surf::Client,
}

impl Client {
    pub fn new(params: Params) -> Self {
        let url = Url::parse(&format!("http://{}:{}", params.host, params.port)).unwrap();
        Self {
            http_client: Config::new()
                .set_base_url(url)
                .set_timeout(Some(Duration::from_secs(5)))
                .try_into()
                .unwrap(),
        }
    }

    pub async fn play(&self, url: &str) -> Result<(), Error> {
        self.play_at(url, 0.0).await?;
        Ok(())
    }

    pub async fn play_at(&self, url: &str, position: f64) -> Result<(), Error> {
        self.http_client
            .post("/play")
            .body(format!(
                "Content-Location: {}\nStart-Position: {}\n",
                url, position
            ))
            .send()
            .await?;
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), Error> {
        self.http_client.post("/stop").send().await?;
        Ok(())
    }

    pub async fn scrub(&self, position: f64) -> Result<(), Error> {
        let q = ScrubQuery { position };
        self.http_client.post("/scrub").query(&q)?.send().await?;
        Ok(())
    }

    pub async fn rate(&self, value: f64) -> Result<(), Error> {
        let q = RateQuery { value };
        self.http_client.post("/rate").query(&q)?.send().await?;
        Ok(())
    }

    pub async fn volume(&self, value: f64) -> Result<(), Error> {
        let q = VolumeQuery { value };
        self.http_client.post("/volume").query(&q)?.send().await?;
        Ok(())
    }

    pub async fn photo(&self) {
        todo!()
    }

    pub async fn playback_info(&self) -> Result<(), Error> {
        let _resp = self.http_client.get("/playback-info").send().await?;
        todo!()
    }

    pub async fn authorize(&self) -> Result<(), Error> {
        todo!()
    }
}
