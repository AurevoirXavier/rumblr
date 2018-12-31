// --- external ---
use serde_json::Value;
use reqwest::multipart::Form;
// --- custom ---
use super::{TumblrClient, build_oauth_headers, build_query, build_params};

const BLOG: &'static str = "https://api.tumblr.com/v2/blog/";

pub enum PostAction<'a> {
    New,
    Edit(&'a str),
    Reblog {
        id: &'a str,
        reblog_key: &'a str,
        comment: &'a str,
        native_inline_images: &'a str,
    },
}

pub enum PostType<'a> {
    Text {
        title: Option<&'a str>,
        body: &'a str,
    },
    Photo {
        caption: Option<&'a str>,
        link: Option<&'a str>,
        source: Option<&'a str>,
        data: Option<Vec<&'a str>>,
        data64: Option<&'a str>,
    },
}

impl TumblrClient {
    pub fn get_blog_info(&self, blog_identifier: &str) -> Value {
        let url = format!("{}{}/info?api_key={}", BLOG, blog_identifier, self.keys.consumer_key);
        self.get(&url, None)
            .json()
            .unwrap()
    }

    pub fn get_blog_avatar(&self, blog_identifier: &str, size: Option<&str>) -> Vec<u8> {
        let mut url = format!("{}{}/avatar/", BLOG, blog_identifier);
        if let Some(size) = size { url += size; }

        let mut bytes = vec![];
        self.get(&url, None)
            .copy_to(&mut bytes)
            .unwrap();

        bytes
    }

    pub fn get_blog_likes(
        &self,
        blog_identifier: &str,
        limit: Option<&str>,
        offset: Option<&str>,
        before: Option<&str>,
        after: Option<&str>,
    ) -> Value {
        let mut url = format!("{}{}/likes?api_key={}", BLOG, blog_identifier, self.keys.consumer_key);
        if let Some(limit) = limit { url += &format!("&limit={}", limit); }
        if let Some(offset) = offset { url += &format!("&offset={}", offset); }
        if let Some(before) = before { url += &format!("&before={}", before); }
        if let Some(after) = after { url += &format!("&after={}", after); }

        self.get(&url, None)
            .json()
            .unwrap()
    }

    pub fn get_blog_following(
        &self,
        blog_identifier: &str,
        limit: Option<&str>,
        offset: Option<&str>,
    ) -> Value {
        let api = format!("{}{}/following", BLOG, blog_identifier);
        let params = {
            let mut v = vec![];
            if let Some(limit) = limit { v.push(("limit", limit)); }
            if let Some(offset) = offset { v.push(("offset", offset)); }

            v
        };
        let url = build_query(&api, &params);
        let headers = build_oauth_headers(
            "GET",
            &api,
            &self.keys.consumer(),
            Some(&self.keys.token()),
            Some(&build_params(params)),
        );

        self.get(&url, Some(headers))
            .json()
            .unwrap()
    }

    pub fn get_blog_followers(
        &self,
        blog_identifier: &str,
        limit: Option<&str>,
        offset: Option<&str>,
    ) -> Value {
        let api = format!("{}{}/followers", BLOG, blog_identifier);
        let params = {
            let mut v = vec![("blog-identifier", blog_identifier)];
            if let Some(limit) = limit { v.push(("limit", limit)); }
            if let Some(offset) = offset { v.push(("offset", offset)); }

            v
        };
        let url = build_query(&api, &params);
        let headers = build_oauth_headers(
            "GET",
            &api,
            &self.keys.consumer(),
            Some(&self.keys.token()),
            Some(&build_params(params)),
        );

        self.get(&url, Some(headers))
            .json()
            .unwrap()
    }

    pub fn get_blog_posts(
        &self,
        blog_identifier: &str,
        r#type: Option<&str>,
        id: Option<&str>,
        tag: Option<&str>,
        limit: Option<&str>,
        offset: Option<&str>,
        reblog_info: Option<&str>,
        notes_info: Option<&str>,
        filter: Option<&str>,
        before: Option<&str>,
    ) -> Value {
        let mut url = format!(
            "{}{}/posts{}?api_key={}",
            BLOG,
            blog_identifier,
            if let Some(r#type) = r#type { format!("/{}", r#type) } else { String::new() },
            self.keys.consumer_key,
        );
        if let Some(id) = id { url += &format!("&id={}", id); }
        if let Some(tag) = tag { url += &format!("&tag={}", tag); }
        if let Some(limit) = limit { url += &format!("&limit={}", limit); }
        if let Some(offset) = offset { url += &format!("&offset={}", offset); }
        if let Some(reblog_info) = reblog_info { url += &format!("&reblog_info={}", reblog_info); }
        if let Some(notes_info) = notes_info { url += &format!("&notes_info={}", notes_info); }
        if let Some(filter) = filter { url += &format!("&filter={}", filter); }
        if let Some(before) = before { url += &format!("&before={}", before); }

        self.get(&url, None)
            .json()
            .unwrap()
    }

    pub fn get_blog_posts_queue(
        &self,
        blog_identifier: &str,
        limit: Option<&str>,
        offset: Option<&str>,
        filter: Option<&str>,
    ) -> Value {
        let api = format!("{}{}/posts/queue", BLOG, blog_identifier);
        let params = {
            let mut v = vec![];
            if let Some(limit) = limit { v.push(("limit", limit)); }
            if let Some(offset) = offset { v.push(("offset", offset)); }
            if let Some(filter) = filter { v.push(("filter", filter)); }

            v
        };
        let url = build_query(&api, &params);
        let headers = build_oauth_headers(
            "GET",
            &api,
            &self.keys.consumer(),
            Some(&self.keys.token()),
            Some(&build_params(params)),
        );

        self.get(&url, Some(headers))
            .json()
            .unwrap()
    }

    pub fn get_blog_posts_draft(
        &self,
        blog_identifier: &str,
        before_id: Option<&str>,
        filter: Option<&str>,
    ) -> Value {
        let api = format!("{}{}/posts/draft", BLOG, blog_identifier);
        let params = {
            let mut v = vec![];
            if let Some(before_id) = before_id { v.push(("before_id", before_id)); }
            if let Some(filter) = filter { v.push(("filter", filter)); }

            v
        };
        let url = build_query(&api, &params);
        let headers = build_oauth_headers(
            "GET",
            &api,
            &self.keys.consumer(),
            Some(&self.keys.token()),
            Some(&build_params(params)),
        );

        self.get(&url, Some(headers))
            .json()
            .unwrap()
    }

    pub fn get_blog_posts_submission(
        &self,
        blog_identifier: &str,
        offset: Option<&str>,
        filter: Option<&str>,
    ) -> Value {
        let api = format!("{}{}/posts/submission", BLOG, blog_identifier);
        let params = {
            let mut v = vec![];
            if let Some(offset) = offset { v.push(("offset", offset)); }
            if let Some(filter) = filter { v.push(("filter", filter)); }

            v
        };
        let url = build_query(&api, &params);
        let headers = build_oauth_headers(
            "GET",
            &api,
            &self.keys.consumer(),
            Some(&self.keys.token()),
            Some(&build_params(params)),
        );

        self.get(&url, Some(headers))
            .json()
            .unwrap()
    }

    pub fn legacy_post(
        &self,
        blog_identifier: &str,
        state: Option<&str>,
        tags: Option<&str>,
        tweet: Option<&str>,
        date: Option<&str>,
        format: Option<&str>,
        slug: Option<&str>,
        native_inline_images: Option<&str>,
        posts_action: PostAction,
        posts_type: PostType,
    ) -> Value {
        // --- custom ---
        use self::{
            PostAction::*,
            PostType::*,
        };

        let api;
        let params = {
            let mut v = vec![];
            if let Some(state) = state { v.push(("state", state)); }
            if let Some(tags) = tags { v.push(("tags", tags)); }
            if let Some(tweet) = tweet { v.push(("tweet", tweet)); }
            if let Some(date) = date { v.push(("date", date)); }
            if let Some(format) = format { v.push(("format", format)); }
            if let Some(slug) = slug { v.push(("slug", slug)); }
            if let Some(native_inline_images) = native_inline_images { v.push(("native_inline_images", native_inline_images)); }

            match posts_action {
                New => {
                    api = format!("{}{}/post", BLOG, blog_identifier);
                }
                Edit(id) => {
                    api = format!("{}{}/post/edit", BLOG, blog_identifier);
                    v.push(("id", id));
                }
                Reblog {
                    id,
                    reblog_key,
                    comment,
                    native_inline_images,
                } => {
                    api = format!("{}{}/post/reblog", BLOG, blog_identifier);
                    v.push(("id", id));
                    v.push(("reblog_key", reblog_key));
                    v.push(("comment", comment));
                    v.push(("native_inline_images", native_inline_images));
                }
            }

            match posts_type {
                Text { title, body } => {
                    v.push(("type", "text"));
                    v.push(("body", body));
                    if let Some(title) = title { v.push(("title", title)); }
                }
                Photo { caption, link, source, data, data64 } => {
                    v.push(("type", "photo"));
                    if let Some(caption) = caption { v.push(("caption", caption)); }
                    if let Some(link) = link { v.push(("link", link)); }
                    if let Some(source) = source { v.push(("source", source)); } else {
                        if let Some(data64) = data64 { v.push(("data64", data64)); } else {
                            if let Some(data) = data {
                                loop {
                                    let headers = build_oauth_headers(
                                        "POST",
                                        &api,
                                        &self.keys.consumer(),
                                        Some(&self.keys.token()),
                                        Some(&build_params(v.clone())),
                                    );

                                    let mut form = Form::new();
                                    for (k, v) in v.iter().cloned() { form = form.text(k.to_owned(), v.to_owned()); }
                                    for (i, f) in data.iter().enumerate() { form = form.file(format!("data[{}]", i), f.to_owned()).unwrap(); }

                                    match self.session.post(&api)
                                        .headers(headers)
                                        .multipart(form)
                                        .send() {
                                        Ok(mut resp) => return resp.json().unwrap(),
                                        Err(e) => {
                                            println!("{:?}", e);
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                panic!("one of [source, data, data64] must be specify")
                            }
                        }
                    }
                }
            }

            v
        };

        let headers = build_oauth_headers(
            "POST",
            &api,
            &self.keys.consumer(),
            Some(&self.keys.token()),
            Some(&build_params(params.clone())),
        );

        self.post(&api, headers, &params)
            .json()
            .unwrap()
    }

//    pub fn neue_post(
//        &self,
//        blog_identifier: &str,
//    ) -> Value {
//        unimplemented!()
//    }

    pub fn delete_post(&self, blog_identifier: &str, id: &str) -> Value {
        let api = format!("{}{}/post/delete", BLOG, blog_identifier);
        let form = vec![("id", id)];
        let headers = build_oauth_headers(
            "POST",
            &api,
            &self.keys.consumer(),
            Some(&self.keys.token()),
            Some(&build_params(form.clone())),
        );

        self.post(&api, headers, &form)
            .json()
            .unwrap()
    }
}
