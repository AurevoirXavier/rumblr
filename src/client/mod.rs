mod oauth;

macro_rules! set_attr {
    ($self_:ident, $attr:ident, $type:ty) => {
        pub fn $attr(mut $self_, $attr: $type) -> Self {
            $self_.$attr = Some($attr);
            $self_
        }
    };
}

macro_rules! set_params {
    [$(($name:expr, $value:expr)),*] => {{
        let mut v = Vec::new();
        $(if let Some(value) = $value { v.push(($name, value)); })*

        v
    }}
}

pub mod user;
pub mod blog;

// --- std ---
use std::{
    borrow::Cow,
    collections::HashMap,
};
// --- external ---
use oauth_client::Token;
use reqwest::{
    Client, ClientBuilder, Proxy, Response,
    header::{AUTHORIZATION, HeaderMap},
};
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Keys {
    consumer_key: String,
    consumer_secret: String,
    access_key: String,
    access_secret: String,
}

impl Keys {
    fn new() -> Keys {
        Keys {
            consumer_key: String::new(),
            consumer_secret: String::new(),
            access_key: String::new(),
            access_secret: String::new(),
        }
    }
}

#[derive(Debug, Fail)]
pub enum TumblrClientError {
    #[fail(display = "Invalid proxy address: {}", address)]
    InvalidProxyAddress { address: String }
}

#[derive(Debug)]
pub struct TumblrClient {
    pub session: Client,
    keys: Keys,
}

impl TumblrClient {
    pub fn new() -> TumblrClient {
        TumblrClient {
            session: Client::new(),
            keys: Keys::new(),
        }
    }

    pub fn proxy(mut self, address: &str) -> Result<Self, TumblrClientError> {
        if let Ok(address) = Proxy::https(address) {
            self.session = ClientBuilder::new()
                .proxy(address)
                .build()
                .unwrap();

            Ok(self)
        } else { Err(TumblrClientError::InvalidProxyAddress { address: address.to_owned() }) }
    }

    fn get(&self, url: &str, headers: Option<HeaderMap>) -> Response {
        let headers = if let Some(headers) = headers { headers } else { HeaderMap::new() };
        loop {
            let headers = headers.clone();
            match self.session.get(url)
                .headers(headers)
                .send() {
                Ok(resp) => return resp,
                Err(e) => {
                    println!("{:?}", e);
                    continue;
                }
            }
        }
    }

    fn post<T: Serialize>(&self, url: &str, headers: HeaderMap, form: &T) -> Response {
        loop {
            let headers = headers.clone();
            match self.session.post(url)
                .headers(headers)
                .form(form)
                .send() {
                Ok(resp) => return resp,
                Err(e) => {
                    println!("{:?}", e);
                    continue;
                }
            }
        }
    }

//    fn post_json<T: Serialize>(&self, url: &str, json: &T) -> Response {
//        loop {
//            match self.session.post(url)
//                .json(json)
//                .send() {
//                Ok(resp) => return resp,
//                Err(e) => {
//                    println!("{:?}", e);
//                    continue;
//                }
//            }
//        }
//    }
}

fn build_oauth_headers(
    method: &str,
    uri: &str,
    consumer: &Token,
    token: Option<&Token>,
    other_param: Option<&HashMap<Cow<str>, Cow<str>>>,
) -> HeaderMap {
    let (header, _) = oauth_client::authorization_header(method, uri, consumer, token, other_param);

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, header.parse().unwrap());

    headers
}

fn build_query(api: &str, params: &[(&str, &str)]) -> String {
    if params.is_empty() { api.to_owned() } else {
        format!(
            "{}?{}",
            api,
            params.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("&")
        )
    }
}

fn build_params<'a>(params: Vec<(&'a str, &'a str)>) -> HashMap<Cow<'a, str>, Cow<'a, str>> {
    params.into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect()
}
