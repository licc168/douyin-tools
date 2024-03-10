#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::model::{Comment, CommentRes};
use crate::parse_data::TableData;
use crate::resp::Resp;
use anyhow::{anyhow, Result};
use csv::Writer;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use tokio::time::sleep;

const COMMENT_API_URL: &str = "https://www.douyin.com/aweme/v1/web/comment/list/";
static mut PARSED_COMMENTS: Vec<Comment> = Vec::new();

#[tauri::command]
async fn collect_comment(
    file_path: String,
    cookie: String,
    video_urls: String,
    window: tauri::Window,
) -> Resp<Vec<TableData>> {
    //解析成数组
    let urls = util::str_to_vec(&*video_urls);
    let mut datas: Vec<TableData> = Vec::new();
    let mut id = 1;
    window.emit("event_log", "采集开始").unwrap();
    // 打印提取到的URL
    for url in &urls.unwrap() {
        let comment_res: Result<CommentRes> = http_util::get_comment_data(&cookie, url, "1").await;
        match comment_res {
            Ok(response) => {
                let total_pages = (response.total + 19) / 20; // 计算总页数
                for page in 1..=total_pages {
                    sleep(std::time::Duration::from_secs(1)).await;
                    let page_response: CommentRes =
                        http_util::get_comment_data(&*cookie, url, &*page.to_string())
                            .await
                            .unwrap();

                    window
                        .emit(
                            "event_log",
                            format!("执行{},共{}页，执行到第{:?}页，", url, total_pages, page)
                                .as_str(),
                        )
                        .unwrap();
                    for comment in page_response.comments.unwrap() {
                        unsafe { PARSED_COMMENTS.clear() };
                        window
                            .emit(
                                "event_data",
                                TableData::from_comment(comment.clone(), url.to_string(), &id, "1"),
                            )
                            .unwrap();
                        datas.push(TableData::from_comment(
                            comment.clone(),
                            url.to_string(),
                            &id,
                            "1",
                        ));
                        id = id + 1;
                        parse_comments(comment.reply_comment);
                        unsafe {
                            for comment in PARSED_COMMENTS.iter() {
                                window
                                    .emit(
                                        "event_data",
                                        TableData::from_comment(
                                            comment.clone(),
                                            url.to_string(),
                                            &id,
                                            "2",
                                        ),
                                    )
                                    .unwrap();
                                datas.push(TableData::from_comment(
                                    comment.clone(),
                                    url.to_string(),
                                    &id,
                                    "2",
                                ));
                                id = id + 1;
                            }
                        };
                    }
                }
            }
            Err(err) => {
                window
                    .emit("event_log", &format!(" {}请检查", err))
                    .unwrap();
                return Resp::fail(1, "cookie设置错误请检查");
            }
        }
    }
    //创建 csv 文件
    create_csv(datas.clone(), file_path.clone());
    window
        .emit(
            "event_log",
            format!(
                "采集完成:文件路径{}/{}",
                file_path.clone(),
                "douyin_comment.csv"
            ),
        )
        .unwrap();
    Resp::success(datas)
}
// 递归解析评论
fn parse_comments(comments: Option<Vec<Comment>>) {
    if let Some(comments_vec) = comments {
        for comment in comments_vec {
            println!("{:?}", comment);
            // 将解析后的评论添加到全局 Vec<Comment> 中
            unsafe { PARSED_COMMENTS.push(comment.clone()) };
            // 递归解析 reply_comment
            if let Some(reply_comments) = comment.reply_comment {
                parse_comments(Some(reply_comments));
            }
        }
    }
}

fn create_csv(datas: Vec<TableData>, file_path: String) {
    let path_name = format!("{}/{}.csv", file_path, "douyin_comment");
    let file = File::create(path_name).unwrap();
    let mut writer = Writer::from_writer(file);
    writer
        .write_record(&[
            "目标链接",
            "评论者",
            "主页链接",
            "评论时间",
            "p属地",
            "评论点赞数",
            "评论级别",
            "评论内容",
        ])
        .unwrap();

    println!("{:?}", datas);
    for row in datas.clone() {
        writer
            .write_record(&[
                row.link,                      // 目标链接
                row.name,                      // 评论者
                row.main_link,                 // 主页链接
                row.date,                      // 评论时间
                row.address,                   // p属地
                row.comment_count.to_string(), // 评论点赞数
                row.comment_level,             // 评论级别
                row.comment_content,           // 评论内容
            ])
            .unwrap();
    }

    // Flush the writer to save the changes to file
    writer.flush().unwrap();
}
mod http_util;
mod menu;
mod model;
mod parse_data;
mod resp;
mod util;

fn main() {
    tauri::Builder::default()
        .setup(|_| Ok(()))
        .invoke_handler(tauri::generate_handler![collect_comment])
        .menu(menu::init()) // ✅ 将菜单添加到所有窗口
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
