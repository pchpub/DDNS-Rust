use super::{types::ProvidersErrorType, DDNSProviderTrait};
use crate::mods::request::{RequestMethod, RequestStructure};
use async_trait::async_trait;
use log::{error, info};
use reqwest::header::{HeaderMap, HeaderName};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::json;
use std::{collections::HashMap, vec};

#[derive(Serialize)]
pub struct Dynv6 {
    pub zone_id: String, //you can get it from the url
    pub token: String,   //Bearer Token
    pub rr: String,
    pub record_type: String,
    #[serde(skip)]
    client: Dynv6Client,
    #[serde(skip)]
    has_record: Option<bool>,
    #[serde(skip)]
    record_id: u64,
}

// 辅助结构体，用于反序列化
#[derive(Deserialize)]
struct ADynv6Helper {
    pub zone_id: String,
    pub token: String, //Bearer Token
    pub rr: String,
    pub record_type: String,
}

impl<'de> Deserialize<'de> for Dynv6 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 首先反序列化辅助结构体
        let helper = ADynv6Helper::deserialize(deserializer)?;

        // check if the token is valid
        if helper.token.is_empty() || contains_invalid_chars(&helper.token) {
            return Err(serde::de::Error::custom("Token is invalid"));
        }

        // 根据 zone_id 和 token 构造 Dynv6Client
        let client = Dynv6Client::new(&helper.token, &helper.zone_id);

        // 构造最终的 Dynv6 结构体
        Ok(Dynv6 {
            zone_id: helper.zone_id,
            token: helper.token,
            rr: helper.rr,
            record_type: helper.record_type,
            client,
            has_record: None,
            record_id: 0,
        })
    }
}

impl Clone for Dynv6 {
    fn clone(&self) -> Self {
        Self {
            zone_id: self.zone_id.clone(),
            token: self.token.clone(),
            rr: self.rr.clone(),
            client: Dynv6Client::new(&self.token, &self.zone_id),
            record_type: self.record_type.clone(),
            has_record: self.has_record,
            record_id: self.record_id.clone(),
        }
    }
}

impl Dynv6 {
    pub fn new(zone_id: &str, token: &str, rr: &str, record_type: &str) -> Self {
        Self {
            zone_id: zone_id.to_string(),
            token: token.to_string(),
            rr: rr.to_string(),
            client: Dynv6Client::new(token, zone_id),
            record_type: record_type.to_string(),
            has_record: Option::None,
            record_id: 0,
        }
    }

    pub async fn delete_subdomain_records(&mut self, record_id: &str) -> Result<(), String> {
        self.client.delete_domain_record(record_id).await
    }
}

#[async_trait]
impl DDNSProviderTrait for Dynv6 {
    async fn update(&mut self, ip: &str) -> Result<(), String> {
        let has_record = match self.has_record {
            Some(has_record) => {
                if has_record {
                    true
                } else {
                    false
                }
            }
            None => match self.get_ip_address().await {
                Ok(_) => true,
                Err(_) => false,
            },
        };

        if has_record {
            match self
                .client
                .update_domain_record(self.record_id, &self.rr, &self.record_type, ip)
                .await
            {
                Ok(_) => {
                    info!("Updated Record ID: {}", self.record_id);
                }
                Err(e) => {
                    error!("Failed to update domain record: {}", e);
                    return Err("Failed to update domain record".to_string());
                }
            }
        } else {
            match self
                .client
                .add_domain_record(&self.zone_id, &self.rr, &self.record_type, ip)
                .await
            {
                Ok(record_id) => {
                    self.record_id = record_id;
                    self.has_record = Some(true);
                    info!("Added Record ID: {}", self.record_id);
                }
                Err(e) => {
                    error!("Failed to add domain record: {}", e);
                    return Err("Failed to add domain record".to_string());
                }
            }
        }

        Ok(())
    }

    async fn get_ip_address(&mut self) -> Result<String, ProvidersErrorType> {
        let query_response = self
            .client
            .query_subdomain_records(&self.rr, &self.record_type)
            .await;
        match query_response {
            Ok(query_response) => {
                if query_response.is_empty() {
                    return Err(ProvidersErrorType::NoRecordFound);
                } else if query_response.len() > 1 {
                    // delete mutiple records (retain the first one)
                    let record_ids = query_response
                        .iter()
                        .skip(1)
                        .map(|record| record.id.to_string())
                        .collect::<Vec<String>>();
                    for record_id in record_ids.iter() {
                        match self.delete_subdomain_records(record_id).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Failed to delete subdomain records: {}", e);
                                return Err(ProvidersErrorType::DeleteDomainRecordsError);
                            }
                        }
                    }
                }

                let ip = query_response[0].data.clone();
                let record_id = query_response[0].id;
                self.record_id = record_id;
                Ok(ip)
            }
            Err(e) => {
                error!("Failed to get ip address: {}", e);
                Err(ProvidersErrorType::QueryDomainRecordsError)
            }
        }
    }
}

struct Dynv6Client {
    zone_id: String,
    token: String,
}

impl Dynv6Client {
    fn new(token: &str, zone_id: &str) -> Self {
        Self {
            zone_id: zone_id.to_string(),
            token: token.to_string(),
        }
    }

    async fn send_records_request(
        &self,
        request: &RequestStructure,
    ) -> Result<Vec<QueryResponse>, String> {
        let mut request = request.clone();
        let headers = HeaderMap::from_iter(vec![
            (
                HeaderName::from_static("Authorization"),
                format!("Bearer {}", self.token).parse().unwrap(),
            ),
            (
                HeaderName::from_static("Content-Type"),
                "application/json".parse().unwrap(),
            ),
            (
                HeaderName::from_static("Accept"),
                "application/json".parse().unwrap(),
            ),
        ]);
        request.headers = Some(headers);
        let rsp: Result<(u16, HashMap<String, String>, String), ()> = request.execute().await;

        let (status, mut rsp_body) = match rsp {
            Ok(value) => (value.0, value.2),
            Err(_) => return Err("Failed to send request".to_string()),
        };

        if status != 200 {
            return Err(format!("Failed to send request, status code: {}", status));
        }

        if rsp_body.is_empty() {
            return Err("Empty response".to_string());
        } else if !rsp_body.starts_with("[") {
            rsp_body = format!("[{}]", rsp_body);
        }

        let query_response: Vec<QueryResponse> = match serde_json::from_str(&rsp_body) {
            Ok(value) => value,
            Err(_e) => {
                return Err("Failed to parse response".to_string());
            }
        };
        Ok(query_response)
    }

    async fn query_subdomain_records(
        &self,
        rr: &str,
        record_type: &str,
    ) -> Result<Vec<QueryResponse>, String> {
        match rr {
            "@" | "" => {
                // shit api
                // fuck dynv6
                let url = format!("https://dynv6.com/api/v2/zones/{}", self.zone_id);

                let request = RequestStructure::new(
                    RequestMethod::GET,
                    url,
                    String::new(),
                    None,
                    None,
                    None,
                    None,
                );

                let query_response = request.execute().await;

                let (status, rsp_body) = match query_response {
                    Ok(value) => (value.0, value.2),
                    Err(_) => {
                        return Err("Failed to send request".to_string());
                    }
                };

                if status != 200 {
                    return Err(format!("Failed to send request, status code: {}", status));
                }

                if rsp_body.is_empty() {
                    return Ok(Vec::new());
                }

                let json_data: serde_json::Value = match serde_json::from_str(&rsp_body) {
                    Ok(value) => value,
                    Err(_e) => {
                        return Err("Failed to parse response".to_string());
                    }
                };

                let custom_data = match record_type {
                    "A" => {
                        let ipv4addr = match json_data["ipv4address"].as_str() {
                            Some(value) => value,
                            None => return Ok(Vec::new()),
                        };

                        let zone_id = match json_data["id"].as_u64() {
                            Some(value) => value,
                            None => return Ok(Vec::new()),
                        };

                        QueryResponse {
                            name: "@".to_string(),
                            data: ipv4addr.to_string(),
                            record_type: "A".to_string(),
                            id: 0,
                            zone_id,
                            ..Default::default()
                        }
                    }
                    "AAAA" => {
                        let ipv6addr = match json_data["ipv6prefix"].as_str() {
                            Some(value) => value,
                            None => return Ok(Vec::new()),
                        };

                        let zone_id = match json_data["id"].as_u64() {
                            Some(value) => value,
                            None => return Ok(Vec::new()),
                        };

                        QueryResponse {
                            name: "@".to_string(),
                            data: ipv6addr.to_string(),
                            record_type: "AAAA".to_string(),
                            id: 0,
                            zone_id,
                            ..Default::default()
                        }
                    }
                    _ => {
                        return Err(
                            "Invalid record type for dynv6, only support A and AAAA".to_string()
                        );
                    }
                };

                Ok(vec![custom_data])
            }
            _ => {
                let url = format!("https://dynv6.com/api/v2/zones/{}/records", self.zone_id);

                let request = RequestStructure::new(
                    RequestMethod::GET,
                    url,
                    String::new(),
                    None,
                    None,
                    None,
                    None,
                );

                let query_response = self.send_records_request(&request).await;

                let query_response = match query_response {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                };

                let query_response: Vec<QueryResponse> = query_response
                    .into_iter()
                    .filter(|record| record.name == rr && record.record_type == record_type)
                    .collect();

                Ok(query_response)
            }
        }
    }

    async fn update_domain_record(
        &self,
        record_id: u64,
        rr: &str,
        record_type: &str,
        value: &str,
    ) -> Result<(), String> {
        match rr {
            "@" | "" => {
                // shit api
                // fuck dynv6
                let url = format!("https://dynv6.com/api/v2/zones/{}", self.zone_id);

                let context = match record_type {
                    "A" => {
                        json!({
                            "ipv4address": value
                        })
                    }
                    "AAAA" => {
                        json!({
                            "ipv6prefix": value
                        })
                    }
                    _ => {
                        return Err(
                            "Invalid record type for dynv6, only support A and AAAA".to_string()
                        );
                    }
                }
                .to_string();

                let request = RequestStructure::new(
                    RequestMethod::PATCH,
                    url,
                    context,
                    None,
                    None,
                    None,
                    None,
                );

                let query_response = request.execute().await;

                let status = match query_response {
                    Ok(value) => value.0,
                    Err(_) => {
                        return Err("Failed to send request".to_string());
                    }
                };

                if status != 200 {
                    return Err(format!("Failed to send request, status code: {}", status));
                }

                Ok(())
            }
            _ => {
                let url = format!(
                    "https://dynv6.com/api/v2/zones/{}/records/{}",
                    self.zone_id, record_id
                );

                let context = match record_type {
                    "A" => {
                        json!({
                            "name": rr,
                            "type": "A",
                            "data": value
                        })
                    }
                    "AAAA" => {
                        json!({
                            "name": rr,
                            "type": "AAAA",
                            "data": value
                        })
                    }
                    _ => {
                        return Err(
                            "Invalid record type for dynv6, only support A and AAAA".to_string()
                        );
                    }
                }
                .to_string();

                let request = RequestStructure::new(
                    RequestMethod::PATCH,
                    url,
                    context,
                    None,
                    None,
                    None,
                    None,
                );

                let query_response = request.execute().await;

                let status = match query_response {
                    Ok(value) => value.0,
                    Err(_) => {
                        return Err("Failed to send request".to_string());
                    }
                };

                if status != 200 {
                    return Err(format!("Failed to send request, status code: {}", status));
                }

                Ok(())
            }
        }
    }

    async fn delete_domain_record(&self, record_id: &str) -> Result<(), String> {
        let url = format!(
            "https://dynv6.com/api/v2/zones/{}/records/{}",
            self.zone_id, record_id
        );

        let request = RequestStructure::new(
            RequestMethod::DELETE,
            url,
            String::new(),
            None,
            None,
            None,
            None,
        );

        let query_response = request.execute().await;

        let status = match query_response {
            Ok(value) => value.0,
            Err(_) => {
                return Err("Failed to send request".to_string());
            }
        };

        if status != 200 {
            return Err(format!("Failed to send request, status code: {}", status));
        }

        Ok(())
    }

    async fn add_domain_record(
        &self,
        domain: &str,
        rr: &str,
        record_type: &str,
        value: &str,
    ) -> Result<u64, String> {
        match domain {
            "@" | "" => {
                return Err("Please add zone on website".to_string());
            }
            _ => {
                let url = format!("https://dynv6.com/api/v2/zones/{}/records", self.zone_id);

                let context = match record_type {
                    "A" | "AAAA" => {
                        json!({
                            "name": rr,
                            "type": record_type,
                            "data": value
                        })
                    }
                    _ => {
                        return Err(
                            "Invalid record type for dynv6, only support A and AAAA".to_string()
                        );
                    }
                }
                .to_string();

                let request = RequestStructure::new(
                    RequestMethod::POST,
                    url,
                    context,
                    None,
                    None,
                    None,
                    None,
                );

                let query_response = self.send_records_request(&request).await;

                let query_response = match query_response {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                };

                let query_response: Vec<QueryResponse> = query_response
                    .into_iter()
                    .filter(|record| record.name == rr && record.record_type == record_type)
                    .collect();

                Ok(query_response[0].id)
            }
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
struct QueryResponse {
    #[serde(default)]
    name: String,
    #[serde(default)]
    priority: Option<u64>,
    #[serde(default)]
    port: Option<u64>,
    #[serde(default)]
    weight: Option<u64>,
    #[serde(default)]
    flags: Option<u8>,
    #[serde(default)]
    tag: Option<Tag>,
    #[serde(default)]
    data: String,
    #[serde(default, rename = "expandedData")]
    expanded_data: Option<String>,
    #[serde(default)]
    id: u64,
    #[serde(default, rename = "zoneID")]
    zone_id: u64,
    #[serde(default, rename = "type")]
    record_type: String,
}

impl Default for QueryResponse {
    fn default() -> Self {
        QueryResponse {
            name: String::new(),
            priority: Option::None,
            port: Option::None,
            weight: Option::None,
            flags: Option::None,
            tag: Option::None,
            data: String::new(),
            expanded_data: Option::None,
            id: 0,
            zone_id: 0,
            record_type: String::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
enum Tag {
    #[serde(rename = "issue")]
    Issue,
    #[serde(rename = "issuewild")]
    Issuewild,
    #[serde(rename = "iodef")]
    Iodef,
}

impl Default for Tag {
    fn default() -> Self {
        Tag::Issue
    }
}

fn contains_invalid_chars(s: &str) -> bool {
    let invalid_chars = "\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\
                         \x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\x7F\
                         :";

    for c in s.chars() {
        if invalid_chars.contains(c) {
            return true;
        }
    }

    false
}
