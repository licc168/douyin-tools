use crate::model::CommentRes;
use crate::{http_util, util, COMMENT_API_URL};
use anyhow::{anyhow, Result};
use rand::seq::SliceRandom;
use reqwest::header;
use std::collections::HashMap; // Import for random selection

/**
* 组装入参
* video_id：视频 id
* page: 页数
*/
pub fn get_params<'a>(video_id: &'a str, page: &'a str) -> HashMap<&'a str, &'a str> {
    let mut params = HashMap::new();
    params.insert("device_platform", "webapp");
    params.insert("aid", "6383");
    params.insert("channel", "channel_pc_web");
    params.insert("aweme_id", video_id);
    params.insert("count", "20");
    params.insert("item_type", "0");
    params.insert("whale_cut_token", "");
    params.insert("cut_version", "1");
    params.insert("rcFT", "");
    params.insert("pc_client_type", "1");
    params.insert("version_code", "170400");
    params.insert("version_name", "17.4.0");
    params.insert("cookie_enabled", "true");
    params.insert("screen_width", "1920");
    params.insert("screen_height", "1080");
    params.insert("browser_language", "zh-CN");
    params.insert("browser_platform", "MacIntel");
    params.insert("browser_name", "Chrome");
    params.insert("browser_version", "122.0.0.0");
    params.insert("browser_online", "true");
    params.insert("engine_name", "Blink");
    params.insert("engine_version", "122.0.0.0");
    params.insert("os_name", "Mac OS");
    params.insert("os_version", "10.15.7");
    params.insert("cpu_core_num", "6");
    params.insert("device_memory", "8");
    params.insert("platform", "PC");
    params.insert("downlink", "10");
    params.insert("effective_type", "4g");
    params.insert("round_trip_time", "50");
    params.insert("webid", "7341701130524658996");
    params.insert("msToken", "VjT914ox94y25sviLBEH1agIm_VfbCOKYwvc3jZjUgGoKdR7NdPAMefyNWXH7d29zI9HpiMG6eo2DK4tRM32Zg3fZByGIDn412Mg3cpF6FqSWhcdsZTvvtJmU8E1GGIF");
    params.insert("X-Bogus", "DFSzswVuGvUANndftbv0TBt/pLwG");

    // 将page参数转换为字符串并插入HashMap
    params.insert("cursor", page); // 将i32转换为String并取其引用

    params
}
/**
 * 获取请求头
 * cookies：cookies
 */
pub fn get_headers<'a>(cookies: &str) -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    println!("cookie:{}", cookies);
    let cookie_header_value = header::HeaderValue::from_str(cookies).unwrap_or_else(|err| {
        println!("Error parsing cookie header: {}", err);
        header::HeaderValue::from_static("")
    });

    headers.insert("cookie", cookie_header_value);

    headers.insert(
        "referer",
        header::HeaderValue::from_static("https://www.douyin.com/"),
    );
    headers.insert(
        "sec-ch-ua",
        header::HeaderValue::from_static(
            "\"Not_A Brand\";v=\"122\", \"Google Chrome\";v=\"24\", \"Chromium\";v=\"122\"",
        ),
    );
    headers.insert("sec-ch-ua-mobile", header::HeaderValue::from_static("?0"));
    headers.insert(
        "sec-ch-ua-platform",
        header::HeaderValue::from_static("\"macOS\""),
    );
    headers.insert("sec-fetch-dest", header::HeaderValue::from_static("empty"));
    headers.insert("sec-fetch-mode", header::HeaderValue::from_static("cors"));
    headers.insert(
        "sec-fetch-site",
        header::HeaderValue::from_static("same-origin"),
    );

    let user_agents = [
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36 SLBrowser/9.0.3.1311 SLBChan/10",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.131 Safari/537.36 - Chrome 92.0 Win10",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36 - Chrome 92.0 Win10",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:90.0) Gecko/20100101 Firefox/90.0 - Firefox 90.0 Win10",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36 - Chrome 91.0 Win10",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.164 Safari/537.36 - Chrome 91.0 Win10",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36 - Chrome 92.0 macOS",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.131 Safari/537.36 - Chrome 92.0 macOS",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.1 Safari/605.1.15 - Safari Generic macOS",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36 - Chrome 91.0 macOS",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.2 Safari/605.1.15 - Safari Generic macOS",
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:90.0) Gecko/20100101 Firefox/90.0 - Firefox 90.0 Linux",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.164 Safari/537.36 - Chrome 91.0 macOS",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:90.0) Gecko/20100101 Firefox/90.0 - Firefox 90.0 macOS",
        "Mozilla/5.0 (Windows NT 10.0; rv:78.0) Gecko/20100101 Firefox/78.0 - Firefox 78.0 Win10",
        "Mozilla/5.0 (X11; Linux x86_64; rv:90.0) Gecko/20100101 Firefox/90.0 - Firefox 90.0 Linux",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:89.0) Gecko/20100101 Firefox/89.0 - Firefox 89.0 Win10",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/91.0 - Firefox 91.0 Win10",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36 Edg/92.0.902.55 - Edge 92.0 Win10",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/91.0 - Firefox 91.0 Win10",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.131 Safari/537.36 Edg/92.0.902.67 - Edge 92.0 Win10",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36 Edg/92.0.902.62 - Edge 92.0 Win10",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36 - Chrome 92.0 Linux",
        "Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0 - Firefox 78.0 Linux",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.131 Safari/537.36 - Chrome 92.0 Linux",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:78.0) Gecko/20100101 Firefox/78.0 - Firefox 78.0 Win10",
        "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.131 Safari/537.36 - Chrome 92.0 Win7",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:89.0) Gecko/20100101 Firefox/89.0 - Firefox 89.0 macOS",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.164 Safari/537.36 Edg/91.0.864.71 - Edge 91.0 Win10",

         "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.115 Safari/537.36 - Chrome"
       ];
    let random_user_agent = user_agents
        .choose(&mut rand::thread_rng())
        .unwrap_or(&user_agents[0]);
    headers.insert(
        "user-agent",
        header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36"),
    );
    return headers;
}

pub async fn get_comment_data(cookies: &str, url: &str, page: &str) -> Result<CommentRes> {
    let headers = http_util::get_headers(cookies);
    let video_id = util::get_video_id(url);
    let mut video_id = video_id.unwrap();
    let params = http_util::get_params(&video_id, &page).clone();
    let client = reqwest::Client::new();

    let resp = client
        .get(COMMENT_API_URL)
        .query(&params)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let response: Result<CommentRes, _> = serde_json::from_str(&*resp);
    // 根据转换的结果返回相应的Result
    match response {
        Ok(res) => Ok(res),
        Err(e) => Err(anyhow!("cookie 设置错误")),
    }
}
