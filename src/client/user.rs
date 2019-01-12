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
pub struct GetUserDashboardOptionalParams<'a> {
    limit: Option<&'a str>,
    offset: Option<&'a str>,
    r#type: Option<&'a str>,
    since_id: Option<&'a str>,
    reblog_info: Option<&'a str>,
    notes_info: Option<&'a str>,
}

impl<'a> GetUserDashboardOptionalParams<'a> {
    set_attr!(self, limit);
    set_attr!(self, offset);
    set_attr!(self, r#type);
    set_attr!(self, since_id);
    set_attr!(self, reblog_info);
    set_attr!(self, notes_info);
}

#[derive(Default)]
pub struct GetUserLikesOptionalParams<'a> {
    limit: Option<&'a str>,
    offset: Option<&'a str>,
    before: Option<&'a str>,
    after: Option<&'a str>,
}

impl<'a> GetUserLikesOptionalParams<'a> {
    set_attr!(self, limit);
    set_attr!(self, offset);
    set_attr!(self, before);
    set_attr!(self, after);
}

#[derive(Default)]
pub struct GetUserFollowingOptionalParams<'a> {
    limit: Option<&'a str>,
    offset: Option<&'a str>,
}

impl<'a> GetUserFollowingOptionalParams<'a> {
    set_attr!(self, limit);
    set_attr!(self, offset);
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

    pub fn get_user_dashboard(&self, optional_params: Option<GetUserDashboardOptionalParams>) -> Value {
        let params = if let Some(optional_params) = optional_params {
            set_params![
                ("limit", optional_params.limit),
                ("offset", optional_params.offset),
                ("type", optional_params.r#type),
                ("since_id", optional_params.since_id),
                ("reblog_info", optional_params.reblog_info),
                ("notes_info", optional_params.notes_info)
            ]
        } else { vec![] };
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

    pub fn get_user_likes(&self, optional_params: Option<GetUserLikesOptionalParams>) -> Value {
        let params = if let Some(optional_params) = optional_params {
            set_params![
                ("limit", optional_params.limit),
                ("offset", optional_params.offset),
                ("before", optional_params.before),
                ("after", optional_params.after)
            ]
        } else { vec![] };
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

    pub fn get_user_following(&self, optional_params: Option<GetUserFollowingOptionalParams>) -> Value {
        let params = if let Some(optional_params) = optional_params {
            set_params![
                ("limit", optional_params.limit),
                ("offset", optional_params.offset)
            ]
        } else { vec![] };
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
