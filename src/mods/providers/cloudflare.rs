// use serde::Serialize;

// #[derive(Serialize)]
// pub struct CloudFlare {
//     pub zone_id: String,
//     pub access_key: String,
//     pub domain: String,
//     pub rr: String,
//     pub ttl: Option<u64>,
//     pub record_type: String,
//     #[serde(skip)]
//     client: reqwest::Client,
//     #[serde(skip)]
//     has_record: Option<bool>,
//     #[serde(skip)]
//     record_id: String,
// }
