pub mod aliyun;
pub mod cloudflare;
pub mod dnspod;
pub mod namecheap;
pub mod types;

use std::collections::HashMap;

use self::types::ProvidersErrorType;

use super::request::RequestStructure;
use async_trait::async_trait;

#[async_trait]
pub trait DDNSProviderTrait: Send + Sync {
    // fn new() -> Self;
    async fn update(&mut self, ip: &str) -> Result<(), String>;
    async fn get_ip_address(&mut self) -> Result<String, ProvidersErrorType>;
    async fn request_execute(
        &self,
        request: &RequestStructure,
    ) -> Result<(HashMap<String, String>, String), ()> {
        request.execute().await
    }
}
