use std::env;

use rocket::serde::Serialize;

pub struct Res;

#[derive(Debug)]
pub enum SendResult {
    Okey(reqwest::Response),
    Error(reqwest::Error),
}

impl Res {
    pub async fn send<T: Serialize>(&self, value: T) -> SendResult {
        let api = "https://graph.facebook.com/v16.0/me/messages?access-token=";
        let page_access_token =
            env::var("PAGE_ACCESS_TOKEN").expect("PAGE_ACCESS_TOKEN not found on .env file");
        let facebook_api = format!("{api}{page_access_token}");
        match reqwest::Client::new()
            .post(&facebook_api)
            .json(&value)
            .send()
            .await
        {
            Ok(response) => SendResult::Okey(response),
            Err(error) => SendResult::Error(error),
        }
    }
}
