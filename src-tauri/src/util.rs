use anyhow::{anyhow, Result};
use polars::export::chrono::{DateTime, NaiveDateTime, Utc};
use polars::export::regex::Regex;

/**
 * 解析SQL 的文件路径  eg ：SELECT * FROM /work/file.csv  解析出来 /work/file.csv
 * @param  sql
 **/
pub fn parse_sql_to_path_name(sql: &str) -> Result<String> {
    let re = Regex::new(r#"(?i)from\s+([^\s;]+)"#)?;
    if let Some(captures) = re.captures(sql) {
        if let Some(table_name) = captures.get(1) {
            return Ok(table_name.as_str().to_string());
        }
    }
    return Err(anyhow!("您写的sql格式有问题"));
}
pub fn str_to_vec(video_urls: &str) -> Result<Vec<String>> {
    // 正则表达式模式，用于匹配URL
    let re = Regex::new(r"https?://(?:www\.)?douyin\.com/video/\d+")?;

    // 创建一个 Vec<String> 用于存储URL
    let mut urls: Vec<String> = Vec::new();

    // 在输入字符串中搜索匹配的URL
    for url in re.find_iter(&*video_urls) {
        // 获取匹配的URL并存储到Vec<String>中
        urls.push(url.as_str().to_string());
    }
    return Ok(urls);
}

pub fn get_video_id(video_url: &str) -> Result<String> {
    // 使用正则表达式匹配 URL 中的 ID 部分
    let re = Regex::new(r"/video/(\d+)").unwrap();

    if let Some(captures) = re.captures(video_url) {
        if let Some(id) = captures.get(1) {
            return Ok(id.as_str().to_string());
        }
    }
    return Err(anyhow!("解析出错"));
}

pub fn get_date(time: i64) -> String {
    // 假设 create_time 是一个 u32 类型的时间戳

    // 将时间戳转换为 NaiveDateTime 类型
    let naive_datetime = NaiveDateTime::from_timestamp(time, 0);

    // 将 NaiveDateTime 转换为 DateTime<Utc> 类型
    let datetime_utc: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);

    // 格式化日期时间为字符串
    let formatted_datetime = datetime_utc.format("%Y-%m-%d %H:%M:%S").to_string();
    formatted_datetime
}
