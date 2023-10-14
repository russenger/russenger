use crate::models::User;
use rocket::tokio::sync::Mutex;

use std::collections::HashMap;
use std::sync::Arc;

#[rocket::async_trait]
pub trait Action: Send + Sync {
    async fn execute(&self, user_id: &str, message: &str, user_conn: &User);
}

type ActionRegistryType = Arc<Mutex<HashMap<&'static str, Box<dyn Action>>>>;

lazy_static::lazy_static! {
    pub static ref ACTION_REGISTRY: ActionRegistryType = Arc::new(Mutex::new(HashMap::new()));
}