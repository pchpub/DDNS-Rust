// use async_trait::async_trait;
// use reqwest::header::{HeaderMap, HeaderValue};
// use serde::Serialize;

// use crate::mods::request::{RequestMethod, RequestStructure};

// use super::{types::ProvidersErrorType, DDNSProviderTrait};

// #[derive(Serialize)]
// pub struct CloudFlare {
//     pub zone_id: String,
//     pub api_token: String,
//     pub domain: String,
//     pub rr: String,
//     pub ttl: Option<u64>,
//     pub record_type: String,
//     #[serde(skip)]
//     has_record: Option<bool>,
//     #[serde(skip)]
//     record_id: String,
// }

// impl CloudFlare {
//     pub fn new(
//         zone_id: String,
//         api_token: String,
//         domain: String,
//         rr: String,
//         ttl: Option<u64>,
//         record_type: String,
//     ) -> Self {
//         CloudFlare {
//             zone_id,
//             api_token,
//             domain,
//             rr,
//             ttl,
//             record_type,
//             has_record: None,
//             record_id: String::new(),
//         }
//     }
// }

// #[async_trait]
// impl DDNSProviderTrait for CloudFlare {
//     async fn update(&mut self, ip: &str) -> Result<(), String> {
//         todo!("update")
//     }

//     async fn get_ip_address(&mut self) -> Result<String, ProvidersErrorType> {
//         // 使用 Cloudflare API 获取特定子域名的 A 记录

//         // 构造请求
//         let mut headers = HeaderMap::new();
//         headers.insert(
//             "Authorization",
//             HeaderValue::from_str(&format!("Bearer {}", self.api_token))
//                 .unwrap_or(HeaderValue::from_static("")),
//         );
//         headers.insert("Content-Type", HeaderValue::from_static("application/json"));

//         let url = format!(
//             "https://api.cloudflare.com/client/v4/zones/{}/dns_records?type={}&name={}",
//             self.zone_id, self.record_type, self.rr
//         );

//         let request = RequestStructure::new(
//             RequestMethod::GET,
//             url,
//             "".to_string(),
//             Some(headers),
//             None,
//             None,
//             None,
//         );
//         let (headers, body) = match self.request_execute(&request).await {
//             Ok((headers, body)) => (headers, body),
//             Err(_) => {
//                 return Err(ProvidersErrorType::NetworkError);
//             }
//         };

//     }
// }
