use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TelegramUser {
    pub id: u32,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub language_code: String,
}

#[derive(Deserialize, Debug)]
pub struct TelegramChat {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct TelegramMessage {
    pub message_id: u32,
    pub from: TelegramUser,
    pub chat: TelegramChat,
    pub date: u32,
    pub text: String,
}
#[derive(Deserialize, Debug)]
pub struct TelegramWebhookUpdate {
    pub update_id: u32,
    pub message: TelegramMessage,
}
