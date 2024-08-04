use super::{types::ProvidersErrorType, DDNSProviderTrait};
use aliyun_dns::{AliyunDns, DomainRecord};
use async_trait::async_trait;
use log::{error, info};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize)]
pub struct Aliyun {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub domain: String,
    pub rr: String,
    pub ttl: Option<u64>,
    pub record_type: String,
    #[serde(skip)]
    aliyun_dns: AliyunDns,
    #[serde(skip)]
    has_record: Option<bool>,
    #[serde(skip)]
    record_id: String,
}

// 辅助结构体，用于反序列化
#[derive(Deserialize)]
struct AliyunHelper {
    access_key_id: String,
    access_key_secret: String,
    domain: String,
    rr: String,
    ttl: Option<u64>,
    record_type: String,
}

impl<'de> Deserialize<'de> for Aliyun {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 首先反序列化辅助结构体
        let helper = AliyunHelper::deserialize(deserializer)?;

        // 根据 access_key_id 和 access_key_secret 构造 AliyunDns
        let aliyun_dns = AliyunDns::new(
            helper.access_key_id.clone(),
            helper.access_key_secret.clone(),
        );

        // 构造最终的 Aliyun 结构体
        Ok(Aliyun {
            access_key_id: helper.access_key_id,
            access_key_secret: helper.access_key_secret,
            domain: helper.domain,
            rr: helper.rr,
            ttl: helper.ttl,
            record_type: helper.record_type,
            aliyun_dns,
            has_record: None,
            record_id: String::new(),
        })
    }
}

impl Clone for Aliyun {
    fn clone(&self) -> Self {
        Self {
            access_key_id: self.access_key_id.clone(),
            access_key_secret: self.access_key_secret.clone(),
            domain: self.domain.clone(),
            rr: self.rr.clone(),
            ttl: self.ttl.clone(),
            record_type: self.record_type.clone(),
            aliyun_dns: AliyunDns::new(self.access_key_id.clone(), self.access_key_secret.clone()),
            has_record: self.has_record.clone(),
            record_id: self.record_id.clone(),
        }
    }
}

impl Aliyun {
    pub fn new(
        access_key_id: &str,
        access_key_secret: &str,
        domain: &str,
        rr: &str,
        ttl: Option<u64>,
        record_type: &str,
    ) -> Self {
        let aliyun_dns = AliyunDns::new(access_key_id.to_owned(), access_key_secret.to_owned());
        Self {
            access_key_id: access_key_id.to_string(),
            access_key_secret: access_key_secret.to_string(),
            domain: domain.to_string(),
            rr: rr.to_string(),
            ttl,
            record_type: record_type.to_string(),
            aliyun_dns,
            has_record: Option::None,
            record_id: String::new(),
        }
    }

    pub async fn delete_subdomain_records(&mut self) -> Result<(), String> {
        match self
            .aliyun_dns
            .delete_subdomain_records(&self.domain, &self.rr, Some(&self.record_type))
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to delete subdomain records: {}", e);
                Err("Failed to delete subdomain records".to_string())
            }
        }
    }
}

#[async_trait]
impl DDNSProviderTrait for Aliyun {
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
            let record_id = self.record_id.clone();
            match self
                .aliyun_dns // It's safe to unwrap here
                .update_domain_record(&self.record_id, &self.rr, &self.record_type, ip, self.ttl)
                .await
            {
                Ok(response) => {
                    if response.record_id != record_id {
                        eprintln!("Record ID not match: {}", response.record_id);
                        self.has_record = Option::None;
                        return Err("Record ID not match".to_string());
                    }
                    println!("Updated Record ID: {}", response.record_id);
                }
                Err(e) => {
                    eprintln!("Failed to update domain record: {}", e);
                    self.has_record = Option::None;
                    return Err("Failed to update domain record".to_string());
                }
            }
            // println!("Updated Record ID: {}", response.record_id);
        } else {
            match self
                .aliyun_dns
                .add_domain_record(&self.domain, &self.rr, &self.record_type, ip, self.ttl)
                .await
            {
                Ok(response) => {
                    self.record_id = response.record_id;
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
        let sub_domain = format!("{}.{}", self.rr, self.domain);
        let mut domain_records: Vec<DomainRecord> = Vec::with_capacity(8);
        let mut total_record_count: u32;
        let mut page_number = 1;
        loop {
            let query_response = match self
                .aliyun_dns
                .query_subdomain_records(
                    &self.domain,
                    &sub_domain,
                    &self.record_type,
                    Some(1),
                    Some(20),
                )
                .await
            {
                Ok(query_response) => query_response,
                Err(e) => {
                    error!("Failed to query domain records: {}", e);
                    return Err(ProvidersErrorType::QueryDomainRecordsError);
                }
            };
            domain_records.extend(query_response.domain_records.records);
            total_record_count = query_response.total_count;
            if (20 * page_number) >= total_record_count {
                break;
            }
            page_number += 1;
        }

        // println!("Total Records: {}", total_record_count);
        if total_record_count == 0 {
            self.has_record = Option::Some(false);
            return Err(ProvidersErrorType::NoRecordFound);
        } else if total_record_count > 1 {
            match self.delete_subdomain_records().await {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to delete subdomain records: {}", e);
                    return Err(ProvidersErrorType::DeleteSubdomainRecordsError);
                }
            };
            self.has_record = Option::Some(false);
            return Err(ProvidersErrorType::TooManyRecords);
        }

        let ip = domain_records[0].value.clone();
        self.record_id = domain_records[0].record_id.clone();

        self.has_record = Option::Some(true);
        Ok(ip)
    }
}
