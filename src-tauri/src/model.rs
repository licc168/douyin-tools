use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub sec_uid: String,

    pub uid: String,
    pub short_id: String,
    pub nickname: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Comment {
    pub cid: String,
    pub text: String,
    pub aweme_id: String,
    pub digg_count: u32,
    pub status: u32,
    pub ip_label: String,
    pub create_time: i64,
    pub user: User,
    pub reply_comment: Option<Vec<Comment>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CommentRes {
    pub status_code: i32,
    pub comments: Option<Vec<Comment>>,
    pub cursor: u64,
    pub has_more: i32,
    pub total: i32,
}
