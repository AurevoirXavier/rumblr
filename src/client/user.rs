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

    pub fn get_user_dashboard(
        &self,
        limit: Option<&str>,
        offset: Option<&str>,
        r#type: Option<&str>,
        since_id: Option<&str>,
        reblog_info: Option<&str>,
        notes_info: Option<&str>,
    ) -> Value {
        let params = {
            let mut v = vec![];
            if let Some(limit) = limit { v.push(("limit", limit)); }
            if let Some(offset) = offset { v.push(("offset", offset)); }
            if let Some(r#type) = r#type { v.push(("type", r#type)); }
            if let Some(since_id) = since_id { v.push(("since_id", since_id)); }
            if let Some(reblog_info) = reblog_info { v.push(("reblog_info", reblog_info)); }
            if let Some(notes_info) = notes_info { v.push(("notes_info", notes_info)); }

            v
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
