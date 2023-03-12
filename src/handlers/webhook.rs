use crate::{
    helpers::respond_ok,
    models::telegram::TelegramWebhookUpdate,
    utils::{
        ip::{get_public_ip, is_reachable},
        telegram::send_text,
    },
};
use actix_web::{web::Json, HttpResponse, Result};

pub async fn telegram_webhook(payload: Json<TelegramWebhookUpdate>) -> Result<HttpResponse> {
    telegram_handler(
        payload.message.text.clone(),
        payload.message.from.id.to_string(),
    )
    .await?;
    respond_ok()
}

async fn telegram_handler(text: String, chat_id: String) -> Result<(), Box<dyn std::error::Error>> {
    match text.as_str() {
        "/ip" => {
            send_text(
                chat_id,
                &format!("{}{}", "My IP is: ", get_public_ip().await?),
            )
            .await?;
        }
        "/ping" => {
            let is_reachable = is_reachable("192.168.1.118");
            if is_reachable {
                send_text(chat_id, "PC Up!").await?;
            } else {
                send_text(chat_id, "PC down").await?;
            }
        }
        "/wol" => {
            let is_reachable = is_reachable("192.168.1.118");
            if is_reachable {
                send_text(chat_id, "PC Up!").await?;
            } else {
                send_text(chat_id, "PC down").await?;
            }
        }
        _ => {
            send_text(chat_id, "Hello").await?;
        }
    };

    Ok(())
}
