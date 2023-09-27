pub mod app_state;

use crate::core::app_state::AppState;
use crate::hooks::messages::FacebookMessage;
use crate::hooks::MessengerWebhookRequest;
use crate::models::User;

use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rocket::{get, post};
use std::collections::HashMap;
use std::sync::Arc;

#[rocket::async_trait]
trait Action: Send + Sync {
    async fn execute(&self, user_id: &str, message: &str, user_conn: &User);
}

type ActionRegisterType = Arc<Mutex<HashMap<String, Box<dyn Action>>>>;

lazy_static::lazy_static! {
    static ref ACTION_REGISTRY: ActionRegisterType = Arc::new(Mutex::new(HashMap::new()));
}

#[get("/webhook")]
pub async fn webhook_verify(request: MessengerWebhookRequest) -> String {
    request.0
}

#[post("/webhook", format = "json", data = "<facebook_message>")]
pub async fn webhook_core(
    facebook_message: Json<FacebookMessage>,
    state: &State<AppState>,
) -> &'static str {
    let message = &facebook_message.get_message();
    let user_id = &facebook_message.get_sender();

    let user_conn = &state.user_conn;
    user_conn.create(user_id).await;

    let action = user_conn
        .get_action(user_id)
        .await
        .expect("failed to get action");

    if let Some(action_fn) = ACTION_REGISTRY.lock().await.get(action.as_str()) {
        action_fn.execute(user_id, message, user_conn).await;
    } else {
        user_conn.reset_action(user_id).await;
    }

    "Ok"
}