mod api {
    pub const REQUEST_TOKEN: &'static str = "https://www.tumblr.com/oauth/request_token";
    pub const AUTHORIZE: &'static str = "https://www.tumblr.com/oauth/authorize";
    pub const ACCESS_TOKEN: &'static str = "https://www.tumblr.com/oauth/access_token";
}


// --- std ---
use std::{
    borrow::Cow,
    collections::HashMap,
    fs::File,
};
// --- external ---
use oauth_client::Token;
// --- custom ---
use super::{Keys, TumblrClient, build_oauth_headers};

impl Keys {
    pub fn consumer(&self) -> Token { Token::new(&self.consumer_key, &self.consumer_secret) }

    pub fn token(&self) -> Token { Token::new(&self.access_key, &self.access_secret) }
}

impl TumblrClient {
    pub fn set_consumer(mut self, key: &str, secret: &str) -> Self {
        self.keys.consumer_key = key.to_owned();
        self.keys.consumer_secret = secret.to_owned();

        self
    }

    fn request_token(&self) -> Token {
        let resp = self.get(
            api::REQUEST_TOKEN,
            Some(build_oauth_headers(
                "GET",
                api::REQUEST_TOKEN,
                &self.keys.consumer(),
                None,
                None,
            )),
        ).text().unwrap();

        let params = split_params(&resp);
        Token::new(
            params.get("oauth_token").unwrap().to_string(),
            params.get("oauth_token_secret").unwrap().to_string(),
        )
    }

    fn authorize(token: &str) { println!("OAuth: {}?oauth_token={}", api::AUTHORIZE, token); }

    fn access_token(&self, token: &Token) -> (String, String) {
        let oauth_verifier = read_line("Url: ");
        let params = {
            let oauth_verifier = oauth_verifier.split("oauth_verifier=")
                .last()
                .unwrap();

            let mut params = HashMap::new();
            params.insert("oauth_verifier".into(), oauth_verifier.into());

            params
        };

        let resp = self.get(
            api::ACCESS_TOKEN,
            Some(build_oauth_headers(
                "GET",
                api::ACCESS_TOKEN,
                &self.keys.consumer(),
                Some(token),
                Some(&params),
            )),
        ).text().unwrap();

        let param = split_params(&resp);
        (
            param.get("oauth_token")
                .unwrap()
                .to_string(),
            param.get("oauth_token_secret")
                .unwrap()
                .to_string()
        )
    }

    pub fn oauth(mut self) -> Self {
        let token = self.request_token();
        TumblrClient::authorize(&token.key);
        let (access_key, access_secret) = self.access_token(&token);

        self.keys.access_key = access_key;
        self.keys.access_secret = access_secret;

        self
    }

    pub fn save_keys(&self, path: &str) -> Result<(), std::io::Error> {
        let file = File::create(path)?;
        serde_json::to_writer(file, &self.keys)?;

        Ok(())
    }

    pub fn load_keys(mut self, path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        self.keys = serde_json::from_reader(file)?;

        Ok(self)
    }
}

fn read_line(tips: &str) -> String {
    // --- std ---
    use std::io::{Write, stdin, stdout};

    let mut s = String::new();
    print!("{}", tips);
    stdout().flush().unwrap();
    stdin().read_line(&mut s).unwrap();

    s.trim().to_owned()
}

fn split_params(query: &str) -> HashMap<Cow<str>, Cow<str>> {
    let mut params = HashMap::new();

    for param in query.split('&') {
        let mut s = param.splitn(2, '=');
        let k = s.next().unwrap();
        let v = s.next().unwrap();

        params.insert(k.into(), v.into());
    }

    params
}
