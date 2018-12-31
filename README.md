## Intro

**Tumblr APIs for Rust**

#### Usage

[Create a new photo post](https://www.tumblr.com/docs/en/api/v2#post--create-a-new-blog-post-legacy):

```rust
extern crate rumblr;

// --- external ---
use rumblr::TumblrClient;

fn main() {
    // OAUTH
    const CONSUMER_KEY: &'static str = "YOUR CONSUMER KEY";
    const CONSUMER_SECRET: &'static str = "YOUR CONSUMER SECRET";

    let client = TumblrClient::new()
        .set_consumer(CONSUMER_KEY, CONSUMER_SECRET)
        .proxy("http://127.0.0.1:1087")
        .unwrap()
        .oauth();

    client.save_keys("rumblr.keys").unwrap();

    // Already OAUTHed
    let client = TumblrClient::new()
        .proxy("http://127.0.0.1:1087")
        .unwrap()
        .load_keys("rumblr.keys")
        .unwrap();

    println!(
        "{:?}",
        client.legacy_post(
            "your tumblr domain [e.g. (david.tumblr.com)]",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            rumblr::PostAction::New,
            rumblr::PostType::Photo {
                caption: None,
                link: None,
                source: Some("https://uvwvu.xyz/favicon.png"),
                data: None,
                data64: None,
            },
        )
    );
}
```

Crawl shield photos:

```rust
extern crate rumblr;
extern crate serde_json;

// --- external ---
use rumblr::TumblrClient;
use serde_json::Value;

fn main() {
    let client = TumblrClient::new()
        .proxy("http://127.0.0.1:1087")
        .unwrap()
        .load_keys("rumblr.keys")
        .unwrap();

    let limit = 10;
    for i in 0u32.. {
        let resp: Value = client.get_blog_posts(
            "target tumblr domain [e.g. (david.tumblr.com)]",
            Some("photo"),
            None,
            None,
            Some(&limit.to_string()),
            Some(&(i * limit).to_string()),
            None,
            None,
            None,
            None,
        );

        let path = "export";
        {
            // --- std ---
            use std::{
                fs::create_dir,
                path::Path,
            };

            let path = Path::new(path);
            if !path.is_dir() { create_dir(path).unwrap(); }
        }

        let posts = resp["response"]["posts"].as_array().unwrap();
        for post in posts {
            let mut infos = String::new();
            
            {
                let photos = post["photos"].as_array().unwrap();
                for photo in photos {
                    infos.push_str(photo["original_size"]["url"].as_str().unwrap());
                    infos.push('\n');
                }
            }

            {
                // --- std ---
                use std::{
                    fs::File,
                    io::Write,
                };

                let mut f = File::create(&format!("{}/{}.txt", path, post["id"].as_u64().unwrap())).unwrap();
                f.write_all(infos.as_bytes()).unwrap();
                f.sync_all().unwrap();
            }
        }

        if posts.len() != limit as usize { break; }
    }
}
```

All client methods sync with [Tumblr API](https://www.tumblr.com/docs/en/api/v2).

#### TODO

Support [Neue Post Format](https://www.tumblr.com/docs/npf).
