use serde::Serialize;
#[derive(Serialize)]
struct Test {
    pub chat_id: String,
    pub text: String,
}

pub async fn send_text(chat_id: String, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let obj = Test { chat_id, text: text.to_string() };
    let client = reqwest::Client::new();

    client
        .post("https://api.telegram.org/bot6270503612:AAFfCs6XeasiFAkbOyfdurd14rFc3S--1Mw/sendMessage")
        .json(&obj)
        .send()
        .await?;
    Ok(())
}
