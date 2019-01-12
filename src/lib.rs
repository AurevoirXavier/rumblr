#[macro_use]
extern crate failure;
extern crate oauth_client;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub use self::client::{
    TumblrClient,
    blog::{
        GetBlogAvatarOptionalParams,
        GetBlogLikesOptionalParams,
        GetBlogFollowingOptionalParams,
        GetBlogFollowersOptionalParams,
        GetBlogPostsOptionalParams,
        GetBlogPostsQueueOptionalParams,
        GetBlogPostsDraftOptionalParams,
        GetBlogPostsSubmissionOptionalParams,
        LegacyPostOptionalParams,
        PostAction,
        PostType
    },
    user::{GetUserDashboardOptionalParams, GetUserLikesOptionalParams, GetUserFollowingOptionalParams},
};

mod client;
