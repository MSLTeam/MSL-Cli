use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoticeData {
    pub notice: String, // 此处对应 content
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TipsData {
    pub tips: Vec<String>, // 此处对应 tips
}

#[derive(Debug, Clone)]
pub struct HomeDisplayData {
    pub notice_html: String,
    pub tips: Vec<String>,
}

pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client:reqwest::Client::builder()
                .user_agent("MSL-Cli/0.2.1-Alpha (By Yuyi-Oak)")
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(),
            base_url: "https://api.mslmc.cn/v3".to_string(),
        }
    }

    // 个人认为此处来点502胶水更合适
    pub async fn fetch_home_display_data(&self) -> Result<HomeDisplayData> {
        let endpoint = format!("{}/query/notice", self.base_url);

        let notice_params = vec![("query".to_string(), "content".to_string())];
        let tips_params = vec![("query".to_string(), "tips".to_string())];

        let notice_resp = self.client.get(&endpoint)
            .query(&notice_params)
            .send()
            .await?
            .json::<ApiResponse<NoticeData>>().await?;

        let tips_resp = self.client.get(&endpoint)
            .query(&tips_params)
            .send()
            .await?
            .json::<ApiResponse<TipsData>>().await?;

        if notice_resp.code != 200 {
            return Err(anyhow::anyhow!("获取公告错误：{}", notice_resp.message));
        }

        Ok(HomeDisplayData {
            notice_html: notice_resp.data.notice,
            tips: tips_resp.data.tips,
        })
    }
}
