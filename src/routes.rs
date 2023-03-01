use crate::handlers::{health::get_health, webhook::telegram_webhook};
use actix_web::web;
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(get_health))
        .service(web::scope("/webhook").route("/telegram", web::post().to(telegram_webhook)));
}
