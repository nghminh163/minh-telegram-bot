use std::process::Command;

use reqwest;

pub async fn get_public_ip() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.ipify.org").await?.text().await?;
    Ok(response)
}

pub fn is_reachable(ip: &str) -> bool {
    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(ip)
        .output()
        .unwrap_or_else(|_| panic!("failed to execute ping command"));

    output.status.success()
}
