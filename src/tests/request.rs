use reqwest::header::{HeaderMap, HeaderName};

#[tokio::test]
async fn request() {
    let headername = HeaderName::from_static("authorization");
    // let headers: HeaderMap<HeaderName> = HeaderMap::from_iter(vec![
    //     (
    //         HeaderName::from_static("Authorization"),
    //         format!("Bearer {}", "test").parse().unwrap(),
    //     ),
    //     (
    //         HeaderName::from_static("Content-Type"),
    //         "application/json".parse().unwrap(),
    //     ),
    //     (
    //         HeaderName::from_static("Accept"),
    //         "application/json".parse().unwrap(),
    //     ),
    // ]);
}
