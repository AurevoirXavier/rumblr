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
    blog::{GetBlogAvatarRequest, GetBlogLikesRequest, GetBlogFollowingRequest, GetBlogFollowersRequest, GetBlogPostsRequest, GetBlogPostsQueueRequest, GetBlogPostsDraftRequest, GetBlogPostsSubmissionRequest, PostAction, PostType},
    user::{GetUserDashboardRequest, GetUserLikesRequest, GetUserFollowingRequest},
};

mod client;
