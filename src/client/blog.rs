// --- external ---
use serde_json::Value;
use reqwest::multipart::Form;
// --- custom ---
use super::{TumblrClient, build_oauth_headers, build_query, build_params};

const BLOG: &'static str = "https://api.tumblr.com/v2/blog/";

#[derive(Default)]
pub struct GetBlogAvatarOptionalParams<'a> { size: Option<&'a str> }

impl<'a> GetBlogAvatarOptionalParams<'a> {
    pub fn new() -> GetBlogAvatarOptionalParams<'a> { GetBlogAvatarOptionalParams::default() }

    pub fn size(mut self, size: &'a str) -> Self {
        match size {
            "16" | "24" | "30" | "40" | "48" | "64" | "96" | "128" | "512" => {
                self.size = Some(size);
                self
            }
            _ => panic!("The size of the avatar (square, one value for both length and width). Must be one of the values: 16, 24, 30, 40, 48, 64, 96, 128, 512")
        }
    }
}

#[derive(Default)]
pub struct GetBlogLikesOptionalParams<'a> {
    limit: Option<&'a str>,
    offset: Option<&'a str>,
    before: Option<&'a str>,
    after: Option<&'a str>,
}

impl<'a> GetBlogLikesOptionalParams<'a> {
    pub fn new() -> GetBlogLikesOptionalParams<'a> { GetBlogLikesOptionalParams::default() }

    set_attr!(self, limit);
    set_attr!(self, offset);
    set_attr!(self, before);
    set_attr!(self, after);
}

#[derive(Default)]
pub struct GetBlogFollowingRequest<'a> {
    limit: Option<&'a str>,
    offset: Option<&'a str>,
}

impl<'a> GetBlogFollowingRequest<'a> {
    pub fn new() -> GetBlogFollowingRequest<'a> { GetBlogFollowingRequest::default() }

    set_attr!(self, limit);
    set_attr!(self, offset);
}

#[derive(Default)]
pub struct GetBlogFollowersOptionalParams<'a> {
    limit: Option<&'a str>,
    offset: Option<&'a str>,
}

impl<'a> GetBlogFollowersOptionalParams<'a> {
    pub fn new() -> GetBlogFollowersOptionalParams<'a> { GetBlogFollowersOptionalParams::default() }

    set_attr!(self, limit);
    set_attr!(self, offset);
}

#[derive(Default)]
pub struct GetBlogPostsOptionalParams<'a> {
    r#type: Option<&'a str>,
    id: Option<&'a str>,
    tag: Option<&'a str>,
    limit: Option<&'a str>,
    offset: Option<&'a str>,
    reblog_info: Option<&'a str>,
    notes_info: Option<&'a str>,
    filter: Option<&'a str>,
    before: Option<&'a str>,
}

impl<'a> GetBlogPostsOptionalParams<'a> {
    pub fn new() -> GetBlogPostsOptionalParams<'a> { GetBlogPostsOptionalParams::default() }

    set_attr!(self, r#type);
    set_attr!(self, id);
    set_attr!(self, tag);
    set_attr!(self, limit);
    set_attr!(self, offset);
    set_attr!(self, reblog_info);
    set_attr!(self, notes_info);
    set_attr!(self, filter);
    set_attr!(self, before);
}

#[derive(Default)]
pub struct GetBlogPostsQueueOptionalParams<'a> {
    limit: Option<&'a str>,
    offset: Option<&'a str>,
    filter: Option<&'a str>,
}

impl<'a> GetBlogPostsQueueOptionalParams<'a> {
    pub fn new() -> GetBlogPostsQueueOptionalParams<'a> { GetBlogPostsQueueOptionalParams::default() }

    set_attr!(self, limit);
    set_attr!(self, offset);
    set_attr!(self, filter);
}

#[derive(Default)]
pub struct GetBlogPostsDraftOptionalParams<'a> {
    before_id: Option<&'a str>,
    filter: Option<&'a str>,
}

impl<'a> GetBlogPostsDraftOptionalParams<'a> {
    pub fn new() -> GetBlogPostsDraftOptionalParams<'a> { GetBlogPostsDraftOptionalParams::default() }

    set_attr!(self, before_id);
    set_attr!(self, filter);
}

#[derive(Default)]
pub struct GetBlogPostsSubmissionOptionalParams<'a> {
    offset: Option<&'a str>,
    filter: Option<&'a str>,
}

impl<'a> GetBlogPostsSubmissionOptionalParams<'a> {
    pub fn new() -> GetBlogPostsSubmissionOptionalParams<'a> { GetBlogPostsSubmissionOptionalParams::default() }

    set_attr!(self, offset);
    set_attr!(self, filter);
}

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
    Quote {
        quote: &'a str,
        source: Option<&'a str>,
    },
    Link {
        title: Option<&'a str>,
        url: &'a str,
        description: Option<&'a str>,
        thumbnail: Option<&'a str>,
        excerpt: Option<&'a str>,
        author: Option<&'a str>,
    },
    Chat {
        title: Option<&'a str>,
        conversation: &'a str,
    },
    Audio {
        caption: Option<&'a str>,
        external_url: Option<&'a str>,
        data: Option<&'a str>,
    },
    Video {
        caption: Option<&'a str>,
        embed: Option<&'a str>,
        data: Option<&'a str>,
    },
}

#[derive(Default)]
pub struct LegacyPostOptionalParams<'a> {
    state: Option<&'a str>,
    tags: Option<&'a str>,
    tweet: Option<&'a str>,
    date: Option<&'a str>,
    format: Option<&'a str>,
    slug: Option<&'a str>,
    native_inline_images: Option<&'a str>,
}

impl<'a> LegacyPostOptionalParams<'a> {
    pub fn new() -> LegacyPostOptionalParams<'a> { LegacyPostOptionalParams::default() }

    set_attr!(self, state);
    set_attr!(self, tags);
    set_attr!(self, tweet);
    set_attr!(self, date);
    set_attr!(self, format);
    set_attr!(self, slug);
    set_attr!(self, native_inline_images);
}

impl TumblrClient {
    pub fn get_blog_info(&self, blog_identifier: &str) -> Value {
        let url = format!("{}{}/info?api_key={}", BLOG, blog_identifier, self.keys.consumer_key);
        self.get(&url, None)
            .json()
            .unwrap()
    }

    pub fn get_blog_avatar(&self, blog_identifier: &str, optional_params: Option<GetBlogAvatarOptionalParams>) -> Vec<u8> {
        let mut url = format!("{}{}/avatar/", BLOG, blog_identifier);
        if let Some(optional_params) = optional_params {
            if let Some(size) = optional_params.size { url += size; }
        }

        let mut bytes = vec![];
        self.get(&url, None)
            .copy_to(&mut bytes)
            .unwrap();

        bytes
    }

    pub fn get_blog_likes(&self, blog_identifier: &str, optional_params: Option<GetBlogLikesOptionalParams>) -> Value {
        let mut url = format!("{}{}/likes?api_key={}", BLOG, blog_identifier, self.keys.consumer_key);
        if let Some(optional_params) = optional_params {
            build_url!(
                url,
                [
                    ("limit", optional_params.limit),
                    ("offset", optional_params.offset),
                    ("before", optional_params.before),
                    ("after", optional_params.after)
                ]
            );
        }

        self.get(&url, None)
            .json()
            .unwrap()
    }

    pub fn get_blog_following(&self, blog_identifier: &str, request: GetBlogFollowingRequest) -> Value {
        let api = format!("{}{}/following", BLOG, blog_identifier);
        let params = set_params![
            ("limit", request.limit),
            ("offset", request.offset)
        ];
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

    pub fn get_blog_followers(&self, blog_identifier: &str, optional_params: Option<GetBlogFollowersOptionalParams>) -> Value {
        let api = format!("{}{}/followers", BLOG, blog_identifier);
        let params = if let Some(optional_params) = optional_params {
            set_params![
                ("limit", optional_params.limit),
                ("offset", optional_params.offset)
            ]
        } else { vec![] };
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

    pub fn get_blog_posts(&self, blog_identifier: &str, optional_params: Option<GetBlogPostsOptionalParams>) -> Value {
        let url = if let Some(optional_params) = optional_params {
            let mut url = format!(
                "{}{}/posts{}?api_key={}",
                BLOG,
                blog_identifier,
                if let Some(r#type) = optional_params.r#type { format!("/{}", r#type) } else { String::new() },
                self.keys.consumer_key,
            );
            build_url!(
                url,
                [
                    ("type", optional_params.r#type),
                    ("id", optional_params.id),
                    ("tag", optional_params.tag),
                    ("limit", optional_params.limit),
                    ("offset", optional_params.offset),
                    ("reblog_info", optional_params.reblog_info),
                    ("notes_info", optional_params.notes_info),
                    ("filter", optional_params.filter),
                    ("before", optional_params.before)
                ]
            );

            url
        } else {
            format!(
                "{}{}/posts{}?api_key={}",
                BLOG,
                blog_identifier,
                String::new(),
                self.keys.consumer_key,
            )
        };

        self.get(&url, None)
            .json()
            .unwrap()
    }

    pub fn get_blog_posts_queue(&self, blog_identifier: &str, optional_params: Option<GetBlogPostsQueueOptionalParams>) -> Value {
        let api = format!("{}{}/posts/queue", BLOG, blog_identifier);
        let params = if let Some(optional_params) = optional_params {
            set_params![
                ("limit", optional_params.limit),
                ("offset", optional_params.offset),
                ("filter", optional_params.filter)
            ]
        } else { vec![] };
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

    pub fn get_blog_posts_draft(&self, blog_identifier: &str, optional_params: Option<GetBlogPostsDraftOptionalParams>) -> Value {
        let api = format!("{}{}/posts/draft", BLOG, blog_identifier);
        let params = if let Some(optional_params) = optional_params {
            set_params![
               ("before_id", optional_params.before_id),
               ("filter", optional_params.filter)
          ]
        } else { vec![] };
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

    pub fn get_blog_posts_submission(&self, blog_identifier: &str, optional_params: Option<GetBlogPostsSubmissionOptionalParams>) -> Value {
        let api = format!("{}{}/posts/submission", BLOG, blog_identifier);
        let params = if let Some(optional_params) = optional_params {
            set_params![
                ("offset", optional_params.offset),
                ("filter", optional_params.filter)
            ]
        } else { vec![] };
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

    pub fn legacy_post(&self, blog_identifier: &str, posts_action: PostAction, posts_type: PostType, optional_params: Option<LegacyPostOptionalParams>) -> Value {
        // --- custom ---
        use self::{
            PostAction::*,
            PostType::*,
        };

        let api;
        let params = {
            let mut v = if let Some(optional_params) = optional_params {
                set_params![
                    ("state", optional_params.state),
                    ("tags", optional_params.tags),
                    ("tweet", optional_params.tweet),
                    ("date", optional_params.date),
                    ("format", optional_params.format),
                    ("slug", optional_params.slug),
                    ("native_inline_images", optional_params.native_inline_images)
                ]
            } else { vec![] };

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
                Quote { quote, source } => {
                    v.push(("type", "quote"));
                    v.push(("quote", quote));
                    if let Some(source) = source { v.push(("source", source)); }
                }
                Link { title, url, description, thumbnail, excerpt, author } => {
                    v.push(("type", "link"));
                    v.push(("url", url));
                    if let Some(title) = title { v.push(("title", title)); }
                    if let Some(description) = description { v.push(("description", description)); }
                    if let Some(thumbnail) = thumbnail { v.push(("thumbnail", thumbnail)); }
                    if let Some(excerpt) = excerpt { v.push(("excerpt", excerpt)); }
                    if let Some(author) = author { v.push(("author", author)); }
                }
                Chat { title, conversation } => {
                    v.push(("type", "chat"));
                    v.push(("conversation", conversation));
                    if let Some(title) = title { v.push(("title", title)); }
                }
                Audio { caption, external_url, data } => {
                    v.push(("type", "audio"));
                    if let Some(caption) = caption { v.push(("caption", caption)); }
                    if let Some(external_url) = external_url { v.push(("external_url", external_url)); } else {
                        if let Some(data) = data { v.push(("data", data)); } else {
                            panic!("one of [external_url, data] must be specify")
                        }
                    }
                }
                Video { caption, embed, data } => {
                    v.push(("type", "video"));
                    if let Some(caption) = caption { v.push(("caption", caption)); }
                    if let Some(embed) = embed { v.push(("embed", embed)); } else {
                        if let Some(data) = data { v.push(("data", data)); } else {
                            panic!("one of [embed, data] must be specify")
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
