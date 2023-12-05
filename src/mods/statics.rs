use crate::mods::types::Config;
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::new()));
}
