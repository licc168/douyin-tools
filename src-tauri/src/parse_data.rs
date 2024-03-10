use crate::model::Comment;
use crate::util;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableData {
    id: i32,
    pub link: String,
    pub main_link: String,
    pub name: String,
    pub date: String,
    pub address: String,
    pub comment_count: u32,
    pub comment_level: String,
    pub comment_content: String,
}

impl TableData {
    pub fn from_comment(comment: Comment, link: String, id: &i32, comment_level: &str) -> Self {
        let main_link = format!("https://www.douyin.com/user/{}", comment.user.sec_uid);
        let name = comment.user.nickname;
        let address = comment.ip_label;
        let comment_count = comment.digg_count;
        let comment_level = comment_level.to_string();
        let comment_content = comment.text;
        let date = util::get_date(comment.create_time);
        let id = *id;
        TableData {
            id,
            link, // You haven't provided a value for link
            main_link,
            name,
            date, // You haven't provided a date field in Comment
            address,
            comment_count,
            comment_level,
            comment_content,
        }
    }
}
