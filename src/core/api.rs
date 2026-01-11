use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnnouncementResponse {
    pub code: i32,
    pub data: AnnouncementData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnnouncementData {
    pub title: String,
    pub content: String,
    pub image_url: Option<String>,
    pub link: Option<String>,
}

pub struct ApiClient {
    client: reqwest::Client,
}
