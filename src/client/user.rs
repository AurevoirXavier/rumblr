mod api {
    // GET
    pub const INFO: &'static str = "https://api.tumblr.com/v2/user/info";
    pub const DASHBOARD: &'static str = "https://api.tumblr.com/v2/user/dashboard";
    pub const LIKES: &'static str = "https://api.tumblr.com/v2/user/likes";
    pub const FOLLOWING: &'static str = "https://api.tumblr.com/v2/user/following";
    // POST
    pub const FOLLOW: &'static str = "https://api.tumblr.com/v2/user/follow";
    pub const UNFOLLOW: &'static str = "https://api.tumblr.com/v2/user/unfollow";
    pub const LIKE: &'static str = "https://api.tumblr.com/v2/user/like";
    pub const UNLIKE: &'static str = "https://api.tumblr.com/v2/user/unlike";
}

// --- external ---
use serde_json::Value;
// --- custom ---
use super::{TumblrClient, build_oauth_headers, build_query, build_params};

#[derive(Default)]
pub struct GetUserDashboradRequest<'a> {
    limit: Option<&'a str>,
    offset: Option<&'a str>,
    r#type: Option<&'a str>,
    since_id: Option<&'a str>,
    reblog_info: Option<&'a str>,
    notes_info: Option<&'a str>,
}

impl<'a> GetUserDashboradRequest<'a> {
    set_attr!(self, limit, &'a str);
    set_attr!(self, offset, &'a str);
    set_attr!(self, r#type, &'a str);
    set_attr!(self, r#since_id, &'a str);
    set_attr!(self, r#reblog_info, &'a str);
    set_attr!(self, r#notes_info, &'a str);
}

impl TumblrClient {
    pub fn get_user_info(&self) -> Value {
        let headers = build_oauth_headers(
            "GET",
            api::INFO,
            &self.keys.consumer(),
            Some(&self.keys.token()),
            None,
        );

        self.get(api::INFO, Some(headers))
            .json()
            .unwrap()
    }

    pub fn get_user_dashboard(&self, request: GetUserDashboradRequest) -> Value {
        let params = set_params! {
            ("limit", request.limit),
            ("offset", request.offset),
            ("type", request.r#type),
            ("since_id", request.since_id),
            ("reblog_info", request.reblog_info),
            ("notes_info", request.notes_info)
        };
        let url = build_query(api::DASHBOARD, &params);
        let headers = build_oauth_headers(
            "GET",
            api::DASHBOARD,
            &self.keys.consumer(),
            Some(&self.keys.token()),
            Some(&build_params(params)),
        );

        self.get(&url, Some(headers))
            .json()
            .unwrap()
    }

    pub fn get_user_likes(
        &self,
        limit: Option<&str>,
        offset: Option<&str>,
        before: Option<&str>,
        after: Option<&str>,
    ) -> Value {
        let params = {
            let mut v = vec![];
            if let Some(limit) = limit { v.push(("limit", limit)); }
            if let Some(offset) = offset { v.push(("offset", offset)); }
            if let Some(before) = before { v.push(("before", before)); }
            if let Some(after) = after { v.push(("after", after)); }

            v
        };
        let url = build_query(api::LIKES, &params);
        let headers = build_oauth_headers(
            "GET",
            api::LIKES,
            &self.keys.consumer(),
            Some(&self.keys.token()),
            Some(&build_params(params)),
        );

        self.get(&url, Some(headers))
            .json()
            .unwrap()
    }

    pub fn get_user_following(
        &self,
        limit: Option<&str>,
        offset: Option<&str>,
    ) -> Value {
        let params = {
            let mut v = vec![];
            if let Some(limit) = limit { v.push(("limit", limit)); }
            if let Some(offset) = offset { v.push(("offset", offset)); }

            v
        };
        let url = build_query(api::FOLLOWING, &params);
        let headers = build_oauth_headers(
            "GET",
            api::FOLLOWING,
            &self.keys.consumer(),
            Some(&self.keys.token()),
            Some(&build_params(params)),
        );

        self.get(&url, Some(headers))
            .json()
            .unwrap()
    }

    pub fn follow_blog(&self, url: &str) -> Value {
        let form = vec![("url", url)];
        let headers = build_oauth_headers(
            "POST",
            api::FOLLOW,
            &self.keys.consumer(),
            Some(&self.keys.token()),
            Some(&build_params(form.clone())),
        );

        self.post(api::FOLLOW, headers, &form)
            .json()
            .unwrap()
    }

    pub fn unfollow_blog(&self, url: &str) -> Value {
        let form = vec![("url", url)];
        let headers = build_oauth_headers(
            "POST",
            api::UNFOLLOW,
            &self.keys.consumer(),
            Some(&self.keys.token()),
            Some(&build_params(form.clone())),
        );

        self.post(api::UNFOLLOW, headers, &form)
            .json()
            .unwrap()
    }

    pub fn like_post(&self, id: &str, reblog_key: &str) -> Value {
        let form = vec![("id", id), ("reblog_key", reblog_key)];
        let headers = build_oauth_headers(
            "POST",
            api::LIKE,
            &self.keys.consumer(),
            Some(&self.keys.token()),
            Some(&build_params(form.clone())),
        );

        self.post(api::LIKE, headers, &form)
            .json()
            .unwrap()
    }

    pub fn unlike_post(&self, id: &str, reblog_key: &str) -> Value {
        let form = vec![("id", id), ("reblog_key", reblog_key)];
        let headers = build_oauth_headers(
            "POST",
            api::UNLIKE,
            &self.keys.consumer(),
            Some(&self.keys.token()),
            Some(&build_params(form.clone())),
        );

        self.post(api::UNLIKE, headers, &form)
            .json()
            .unwrap()
    }
}
