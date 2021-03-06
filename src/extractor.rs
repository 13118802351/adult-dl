mod fc2;
mod pornhub;
mod xvideos;

use async_trait::async_trait;
use fc2::Fc2;
use pornhub::Pornhub;
use reqwest::Url;
use xvideos::Xvideos;

#[async_trait]
pub trait Extractor {
    async fn extract(&self, url: &str) -> Result<VideoInfo, Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub struct VideoInfo {
    pub url: String,
    pub title: String,
    pub size: usize,
    pub filename: String,
}

pub async fn select_extractor(url: &str) -> Result<Box<dyn Extractor>, Box<dyn std::error::Error>> {
    let parsed = Url::parse(url)?;
    let domain = parsed.domain().unwrap().to_string();

    let e: Box<dyn Extractor> = if domain.contains("xvideos") {
        Box::new(Xvideos {})
    } else if domain.contains("fc2") {
        panic!("Not support site...");
    // Box::new(Fc2 {})
    } else if domain.contains("pornhub") {
        panic!("Not support site...");
    // Box::new(Pornhub {})
    } else {
        panic!("Not support site...");
    };

    Ok(e)
}
