use reqwest::header;
use reqwest::Result;
use serde::Deserialize;
use std::collections::HashMap;
#[derive(Debug, Deserialize)]
struct User {
    uid: String,
    short_id: String,
    nickname: String,
}
#[derive(Debug, Deserialize)]
struct Comment {
    cid: String,
    text: String,
    aweme_id: String,
    digg_count: u32,
    status: u32,
    user: User,
    reply_comment: Option<Vec<ReplyComment>>,
}
#[derive(Debug, Deserialize)]
struct ReplyComment {
    cid: String,
    text: String,
    aweme_id: String,
    digg_count: u32,
    status: u32,
    user: User,
}

#[derive(Debug, Deserialize)]
struct Response {
    status_code: i32,
    comments: Option<Vec<Comment>>,
    cursor: u64,
    has_more: i32,
    total: i32,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 定义请求地址
    let url = "https://www.douyin.com/aweme/v1/web/comment/list/";
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "authority",
        header::HeaderValue::from_static("www.douyin.com"),
    );
    headers.insert("method", header::HeaderValue::from_static("GET"));

    // headers.insert("accept-language", header::HeaderValue::from_static("zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7"));
    // headers.insert("accept-encoding", header::HeaderValue::from_static("gzip, deflate, br"));
    // headers.insert("accept-language", header::HeaderValue::from_static("zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7"));
    headers.insert("cookie",header::HeaderValue::from_static( "ttwid=1%7CTB42b8vTsJC21azGVuP3DtgRfpH24XbeeDwXYC-3Bq8%7C1709282826%7C6d78061c0a5aa49d8d12d3f2eeec786bf9d1e27cc5cc4a93ba70f0f4bcacde7f; dy_swidth=1920; dy_sheight=1080; FORCE_LOGIN=%7B%22videoConsumedRemainSeconds%22%3A180%7D; passport_csrf_token=6c145268d979a7b9ff95f4ad6609cb28; passport_csrf_token_default=6c145268d979a7b9ff95f4ad6609cb28; bd_ticket_guard_client_web_domain=2; ttcid=7624ce32e69b483887ef4144cb59299641; passport_assist_user=CjyaRElJgcLg12iYDt_BKx5ZFlk6EiLSnYzmJHdZjHfunHdB2ks1x5WDVnSLAfahQKOIZDxUk6n92gfWgKcaSgo8Uub2SnU_r0aJwSjOwU7hh_eUdLaOSrmUJrYbGMZJFKiLmBhP0ZHudkhA1UeqfNQkWknvf5k3H24Xj90QEMzhyg0Yia_WVCABIgEDGICoAQ%3D%3D; n_mh=JqkpwmYn68ctwmdqwqgf1yWGH0J9LVG7e4F6IUk46fU; sso_uid_tt=1241f2b3eca56085d9b771c15e0013a8; sso_uid_tt_ss=1241f2b3eca56085d9b771c15e0013a8; toutiao_sso_user=c51fd84559037cfbce4d3ecee4dffb9e; toutiao_sso_user_ss=c51fd84559037cfbce4d3ecee4dffb9e; sid_ucp_sso_v1=1.0.0-KGU0MDE2MjA4MGY3NjFiOTY1Zjc4ZTdkYjEyNWM0ZmYxNGI3YjIzZjkKHQj_u-3MkQIQxqyGrwYY7zEgDDD-g8XPBTgGQPQHGgJscSIgYzUxZmQ4NDU1OTAzN2NmYmNlNGQzZWNlZTRkZmZiOWU; ssid_ucp_sso_v1=1.0.0-KGU0MDE2MjA4MGY3NjFiOTY1Zjc4ZTdkYjEyNWM0ZmYxNGI3YjIzZjkKHQj_u-3MkQIQxqyGrwYY7zEgDDD-g8XPBTgGQPQHGgJscSIgYzUxZmQ4NDU1OTAzN2NmYmNlNGQzZWNlZTRkZmZiOWU; passport_auth_status=3398bb8a6f54b65a7ca5964749aec9fe%2C; passport_auth_status_ss=3398bb8a6f54b65a7ca5964749aec9fe%2C; uid_tt=50442adcc6273041d58cae1f4b08cfed; uid_tt_ss=50442adcc6273041d58cae1f4b08cfed; sid_tt=77368ae50d2854dafcabf599d53b7f43; sessionid=77368ae50d2854dafcabf599d53b7f43; sessionid_ss=77368ae50d2854dafcabf599d53b7f43; LOGIN_STATUS=1; _bd_ticket_crypt_doamin=2; _bd_ticket_crypt_cookie=bdec0e266c8c5e8316aae325998ad1ce; __security_server_data_status=1; my_rd=2; store-region=cn-gd; store-region-src=uid; s_v_web_id=verify_lt8ewjck_Z0vDd0sE_FT2R_43mf_BBjX_dxGHl2f2CcvS; d_ticket=73ffb810fd962e65ecffda4e1db2a87a0cb35; sid_guard=77368ae50d2854dafcabf599d53b7f43%7C1709282918%7C5183972%7CTue%2C+30-Apr-2024+08%3A48%3A10+GMT; sid_ucp_v1=1.0.0-KDc3OWY5NDdhOTdhOGJkYjBkOWJlZjMyZDE3OWNmM2Q2NzA2M2Q2OGIKGQj_u-3MkQIQ5qyGrwYY7zEgDDgGQPQHSAQaAmxxIiA3NzM2OGFlNTBkMjg1NGRhZmNhYmY1OTlkNTNiN2Y0Mw; ssid_ucp_v1=1.0.0-KDc3OWY5NDdhOTdhOGJkYjBkOWJlZjMyZDE3OWNmM2Q2NzA2M2Q2OGIKGQj_u-3MkQIQ5qyGrwYY7zEgDDgGQPQHSAQaAmxxIiA3NzM2OGFlNTBkMjg1NGRhZmNhYmY1OTlkNTNiN2Y0Mw; SEARCH_RESULT_LIST_TYPE=%22single%22; csrf_session_id=560b01cff2fe81fc64bc927ff249cb9b; publish_badge_show_info=%221%2C0%2C0%2C1709465962444%22; download_guide=%221%2F20240303%2F1%22; passport_fe_beating_status=true; pwa2=%220%7C0%7C3%7C1%22; douyin.com; device_web_cpu_core=6; device_web_memory_size=8; __live_version__=%221.1.1.8422%22; webcast_local_quality=null; live_use_vvc=%22false%22; webcast_leading_last_show_time=1709466138682; webcast_leading_total_show_times=1; xgplayer_device_id=37139889661; xgplayer_user_id=132003902605; strategyABtestKey=%221709466311.796%22; live_can_add_dy_2_desktop=%221%22; volume_info=%7B%22isUserMute%22%3Afalse%2C%22isMute%22%3Atrue%2C%22volume%22%3A0.7%7D; stream_player_status_params=%22%7B%5C%22is_auto_play%5C%22%3A0%2C%5C%22is_full_screen%5C%22%3A0%2C%5C%22is_full_webscreen%5C%22%3A0%2C%5C%22is_mute%5C%22%3A1%2C%5C%22is_speed%5C%22%3A1%2C%5C%22is_visible%5C%22%3A1%7D%22; xg_device_score=7.2627859016150556; FOLLOW_NUMBER_YELLOW_POINT_INFO=%22MS4wLjABAAAAG_lG8dtanat7ZYqpPxZBORFCqmCiJ3NAoPEgWL7v0qM%2F1709481600000%2F0%2F1709473438462%2F0%22; stream_recommend_feed_params=%22%7B%5C%22cookie_enabled%5C%22%3Atrue%2C%5C%22screen_width%5C%22%3A1920%2C%5C%22screen_height%5C%22%3A1080%2C%5C%22browser_online%5C%22%3Atrue%2C%5C%22cpu_core_num%5C%22%3A6%2C%5C%22device_memory%5C%22%3A8%2C%5C%22downlink%5C%22%3A10%2C%5C%22effective_type%5C%22%3A%5C%224g%5C%22%2C%5C%22round_trip_time%5C%22%3A50%7D%22; __ac_nonce=065e48f8d00224b5926; __ac_signature=_02B4Z6wo00f01e7S2vwAAIDBLXGl-abhcPXu8t5AAB5SRnj4CgYblYStQIqaWj6PnSwOz7sbZkABL5w6zTUX5gN-CPb7agPzKum0k6T9cUU-cQmRDF14YmZAOTCMh9qR.1zEHmokCXUz8JL0a4; IsDouyinActive=true; FOLLOW_LIVE_POINT_INFO=%22MS4wLjABAAAAG_lG8dtanat7ZYqpPxZBORFCqmCiJ3NAoPEgWL7v0qM%2F1709481600000%2F0%2F1709477776477%2F0%22; bd_ticket_guard_client_data=eyJiZC10aWNrZXQtZ3VhcmQtdmVyc2lvbiI6MiwiYmQtdGlja2V0LWd1YXJkLWl0ZXJhdGlvbi12ZXJzaW9uIjoxLCJiZC10aWNrZXQtZ3VhcmQtcmVlLXB1YmxpYy1rZXkiOiJCQzJGcEU2eG9GTlhEQUpkdjY4SFg0alNseDI4eDZ5T3RXMXJ2TnNlOWZMZGJod25FRnYzMHFSbGQ2RThKSTVhL0xaTzVCN25xbm5IU0pibURCYSs0cnc9IiwiYmQtdGlja2V0LWd1YXJkLXdlYi12ZXJzaW9uIjoxfQ%3D%3D; msToken=vfEFy2wINFYSOQ7Gjk8tbttPb4uk8xAXvszZ8gAT-8V31Q8MdSZAvQ1F4F3lhL7xnfu5xNpeC4uVhL6j1tdiYyTsvzo5t2GoebSR5BFW86MjFnSnVWpk1RA_4euMxeQ=; home_can_add_dy_2_desktop=%221%22; tt_scid=kWhi5QGYJZJIyRlEbSKo9TRbyJr8wnV91RA1MMkc1ls3VMpqX7IPLNpfBOK2gijb3baf; odin_tt=ea896d105474bbdad076385d36883a40961dacaad45af1b3628de09f43126581c78c86ba26ed43509de7eebec38c693c; msToken=4izQKaW9tGOzU2YAuuEOkHBoyApfOD5TbwzayIfK1psnWXbvxZvm68Te3N6KT2efBKjkXWoVNLhcLdWWdgH_C2oaKlxXz8l2LVrxH-4dXqJhX_QuYcRFsFSCUB1W0oI="));
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
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36 SLBrowser/9.0.3.1311 SLBChan/10".parse().unwrap());

    let mut params = HashMap::new();
    params.insert("device_platform", "webapp");
    params.insert("aid", "6383");
    params.insert("channel", "channel_pc_web");
    params.insert("aweme_id", "7341701130524658996");
    params.insert("cursor", "1");
    params.insert("count", "10");
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
    params.insert("webid", "7208902085361960506");
    params.insert("msToken", "VjT914ox94y25sviLBEH1agIm_VfbCOKYwvc3jZjUgGoKdR7NdPAMefyNWXH7d29zI9HpiMG6eo2DK4tRM32Zg3fZByGIDn412Mg3cpF6FqSWhcdsZTvvtJmU8E1GGIF");
    params.insert("X-Bogus", "DFSzswVuGvUANndftbv0TBt/pLwG");
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .query(&params)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    println!("{:?}", resp);

    // 解析 JSON 响应
    let response: Response = serde_json::from_str(&*resp).unwrap();

    // 处理结果
    println!("{:?}", response);
    Ok(())
}
