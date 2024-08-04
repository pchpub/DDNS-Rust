use reqwest::header::HeaderMap;
use std::{collections::HashMap, time::Duration};

async fn async_getwebpage(
    url: &str,
    proxy_open: bool,
    proxy_url: &str,
    user_agent: &str,
    cookie: &str,
    headers: Option<HeaderMap>,
) -> Result<(u16, HashMap<String, String>, String), ()> {
    let mut client_builder = reqwest::Client::builder();
    if proxy_open && proxy_url.len() != 0 {
        client_builder = client_builder.proxy(if proxy_url.contains("://") {
            if let Ok(value) = reqwest::Proxy::all(proxy_url) {
                value
            } else {
                return Err(());
            }
        } else {
            if let Ok(value) = reqwest::Proxy::all(format!("socks5://{}", proxy_url)) {
                value
            } else {
                return Err(());
            }
        });
    }
    let mut client = if let Ok(value) = client_builder
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .timeout(Duration::from_secs(20))
        .user_agent(user_agent)
        .build()
    {
        value
    } else {
        return Err(());
    }
    .get(url);
    if let Some(value) = headers {
        client = client
            .headers(value)
            .header("cookie", cookie)
            .header("Accept-Encoding", "gzip, deflate, br");
    }
    let rsp_raw_data = if let Ok(value) = client.send().await {
        value
    } else {
        return Err(());
    };
    // match rsp_raw_data.status().as_u16() {
    //     404 | 429 => return Err(()),
    //     _ => (),
    // }
    let status = rsp_raw_data.status().as_u16();
    let rsp_headers: HashMap<String, String> = rsp_raw_data
        .headers()
        .iter()
        .map(|(k, v)| (k.as_str().to_owned(), v.to_str().unwrap_or("").to_owned()))
        .collect();
    let rsp_body = if let Ok(value) = rsp_raw_data.text().await {
        value
    } else {
        return Err(());
    };
    Ok((status, rsp_headers, rsp_body))
}

async fn async_deletewebpage(
    url: &str,
    proxy_open: bool,
    proxy_url: &str,
    user_agent: &str,
    cookie: &str,
    headers: Option<HeaderMap>,
) -> Result<(u16, HashMap<String, String>, String), ()> {
    let mut client_builder = reqwest::Client::builder();
    if proxy_open && proxy_url.len() != 0 {
        client_builder = client_builder.proxy(if proxy_url.contains("://") {
            if let Ok(value) = reqwest::Proxy::all(proxy_url) {
                value
            } else {
                return Err(());
            }
        } else {
            if let Ok(value) = reqwest::Proxy::all(format!("socks5://{}", proxy_url)) {
                value
            } else {
                return Err(());
            }
        });
    }
    let mut client = if let Ok(value) = client_builder
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .timeout(Duration::from_secs(20))
        .user_agent(user_agent)
        .build()
    {
        value
    } else {
        return Err(());
    }
    .delete(url);
    if let Some(value) = headers {
        client = client
            .headers(value)
            .header("cookie", cookie)
            .header("Accept-Encoding", "gzip, deflate, br");
    }
    let rsp_raw_data = if let Ok(value) = client.send().await {
        value
    } else {
        return Err(());
    };
    // match rsp_raw_data.status().as_u16() {
    //     404 | 429 => return Err(()),
    //     _ => (),
    // }
    let status = rsp_raw_data.status().as_u16();
    let rsp_headers: HashMap<String, String> = rsp_raw_data
        .headers()
        .iter()
        .map(|(k, v)| (k.as_str().to_owned(), v.to_str().unwrap_or("").to_owned()))
        .collect();
    let rsp_body = if let Ok(value) = rsp_raw_data.text().await {
        value
    } else {
        return Err(());
    };
    Ok((status, rsp_headers, rsp_body))
}

async fn async_postwebpage(
    url: &str,
    content: &str,
    proxy_open: bool,
    proxy_url: &str,
    user_agent: &str,
    cookie: &str,
    headers: Option<HeaderMap>,
) -> Result<(u16, HashMap<String, String>, String), ()> {
    let mut client_builder = reqwest::Client::builder();
    if proxy_open && proxy_url.len() != 0 {
        client_builder = client_builder.proxy(if proxy_url.contains("://") {
            if let Ok(value) = reqwest::Proxy::all(proxy_url) {
                value
            } else {
                return Err(());
            }
        } else {
            if let Ok(value) = reqwest::Proxy::all(format!("socks5://{}", proxy_url)) {
                value
            } else {
                return Err(());
            }
        });
    }
    let mut client = if let Ok(value) = client_builder
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .timeout(Duration::from_secs(20))
        .user_agent(user_agent)
        .build()
    {
        value
    } else {
        return Err(());
    }
    .post(url)
    .body(content.to_owned());
    if let Some(value) = headers {
        client = client
            .headers(value)
            .header("cookie", cookie)
            .header("Accept-Encoding", "gzip, deflate, br");
    }
    let rsp_raw_data = if let Ok(value) = client.send().await {
        value
    } else {
        return Err(());
    };
    // match rsp_raw_data.status().as_u16() {
    //     404 | 429 => return Err(()),
    //     _ => (),
    // }
    let status = rsp_raw_data.status().as_u16();

    let rsp_headers: HashMap<String, String> = rsp_raw_data
        .headers()
        .iter()
        .map(|(k, v)| (k.as_str().to_owned(), v.to_str().unwrap_or("").to_owned()))
        .collect();
    let rsp_body = if let Ok(value) = rsp_raw_data.text().await {
        value
    } else {
        return Err(());
    };
    Ok((status, rsp_headers, rsp_body))
}

async fn async_patchwebpage(
    url: &str,
    content: &str,
    proxy_open: bool,
    proxy_url: &str,
    user_agent: &str,
    cookie: &str,
    headers: Option<HeaderMap>,
) -> Result<(u16, HashMap<String, String>, String), ()> {
    let mut client_builder = reqwest::Client::builder();
    if proxy_open && proxy_url.len() != 0 {
        client_builder = client_builder.proxy(if proxy_url.contains("://") {
            if let Ok(value) = reqwest::Proxy::all(proxy_url) {
                value
            } else {
                return Err(());
            }
        } else {
            if let Ok(value) = reqwest::Proxy::all(format!("socks5://{}", proxy_url)) {
                value
            } else {
                return Err(());
            }
        });
    }
    let mut client = if let Ok(value) = client_builder
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .timeout(Duration::from_secs(20))
        .user_agent(user_agent)
        .build()
    {
        value
    } else {
        return Err(());
    }
    .patch(url)
    .body(content.to_owned());
    if let Some(value) = headers {
        client = client
            .headers(value)
            .header("cookie", cookie)
            .header("Accept-Encoding", "gzip, deflate, br");
    }
    let rsp_raw_data = if let Ok(value) = client.send().await {
        value
    } else {
        return Err(());
    };
    // match rsp_raw_data.status().as_u16() {
    //     404 | 429 => return Err(()),
    //     _ => (),
    // }
    let status = rsp_raw_data.status().as_u16();

    let rsp_headers: HashMap<String, String> = rsp_raw_data
        .headers()
        .iter()
        .map(|(k, v)| (k.as_str().to_owned(), v.to_str().unwrap_or("").to_owned()))
        .collect();
    let rsp_body = if let Ok(value) = rsp_raw_data.text().await {
        value
    } else {
        return Err(());
    };
    Ok((status, rsp_headers, rsp_body))
}

pub struct RequestStructure {
    pub mathod: RequestMethod,
    pub url: String,
    pub content: String,
    pub headers: Option<HeaderMap>,
    pub proxy: Option<String>,
    pub user_agent: Option<String>,
    pub cookie: Option<String>,
}

impl Clone for RequestStructure {
    fn clone(&self) -> Self {
        Self {
            mathod: self.mathod.clone(),
            url: self.url.clone(),
            content: self.content.clone(),
            headers: self.headers.clone(),
            proxy: self.proxy.clone(),
            user_agent: self.user_agent.clone(),
            cookie: self.cookie.clone(),
        }
    }
}

impl RequestStructure {
    pub fn new(
        mathod: RequestMethod,
        url: String,
        content: String,
        headers: Option<HeaderMap>,
        proxy: Option<String>,
        user_agent: Option<String>,
        cookie: Option<String>,
    ) -> Self {
        Self {
            mathod,
            url,
            content,
            headers,
            proxy,
            user_agent,
            cookie,
        }
    }

    pub fn new_default(mathod: RequestMethod, url: String, content: String) -> Self {
        Self {
            mathod,
            url,
            content,
            headers: None,
            proxy: None,
            user_agent: None,
            cookie: None,
        }
    }

    pub async fn execute(&self) -> Result<(u16, HashMap<String, String>, String), ()> {
        match self.mathod {
            RequestMethod::GET => {
                async_getwebpage(
                    &self.url,
                    self.proxy.is_some(),
                    &self.proxy.clone().unwrap_or_default(),
                    &self.user_agent.clone().unwrap_or_default(),
                    &self.cookie.clone().unwrap_or_default(),
                    self.headers.clone(),
                )
                .await
            }
            RequestMethod::POST => {
                async_postwebpage(
                    &self.url,
                    &self.content,
                    self.proxy.is_some(),
                    &self.proxy.clone().unwrap_or_default(),
                    &self.user_agent.clone().unwrap_or_default(),
                    &self.cookie.clone().unwrap_or_default(),
                    self.headers.clone(),
                )
                .await
            }
            RequestMethod::PATCH => {
                async_patchwebpage(
                    &self.url,
                    &self.content,
                    self.proxy.is_some(),
                    &self.proxy.clone().unwrap_or_default(),
                    &self.user_agent.clone().unwrap_or_default(),
                    &self.cookie.clone().unwrap_or_default(),
                    self.headers.clone(),
                )
                .await
            }
            RequestMethod::DELETE => {
                async_deletewebpage(
                    &self.url,
                    self.proxy.is_some(),
                    &self.proxy.clone().unwrap_or_default(),
                    &self.user_agent.clone().unwrap_or_default(),
                    &self.cookie.clone().unwrap_or_default(),
                    self.headers.clone(),
                )
                .await
            }
        }
    }
}

pub enum RequestMethod {
    GET,
    POST,
    PATCH,
    DELETE,
}

impl Clone for RequestMethod {
    fn clone(&self) -> Self {
        match self {
            RequestMethod::GET => RequestMethod::GET,
            RequestMethod::POST => RequestMethod::POST,
            RequestMethod::PATCH => RequestMethod::PATCH,
            RequestMethod::DELETE => RequestMethod::DELETE,
        }
    }
}
