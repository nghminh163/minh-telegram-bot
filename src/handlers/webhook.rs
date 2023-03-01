use crate::{helpers::respond_ok, models::telegram::TelegramWebhookUpdate};
use actix_web::{web::Json, HttpResponse, Result};

use serde::Serialize;
#[derive(Serialize)]
pub struct Test {
    pub chat_id: String,
    pub text: String,
}

async fn test() -> Result<(), Box<dyn std::error::Error>> {
    let obj = Test {
        chat_id: "805525292".to_string(),
        text: "abc".to_string(),
    };
    let client = reqwest::Client::new();

    let res = client
        .post("https://api.telegram.org/bot6270503612:AAFS3tWamqBq2sSP-kSQnvEnx5Vkpp6_qnc/sendMessage")
        .json(&obj)
        .send()
        .await?;
    println!("test {}", res.text().await?);
    Ok(())
}

pub async fn telegram_webhook(payload: Json<TelegramWebhookUpdate>) -> Result<HttpResponse> {
    test().await?;
    respond_ok()
}
