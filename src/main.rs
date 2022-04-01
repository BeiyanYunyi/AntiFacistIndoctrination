use regex::Regex;
use reqwest::{cookie::Jar, Url};
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = utils::get_args();
    let url = "https://service.jiangsugqt.org".parse::<Url>().unwrap();
    let jar = Jar::default();
    let cookie_str = match args.cookie {
        Some(cookie) => cookie.to_string(),
        None => {
            if !envmnt::exists("AFI_COOKIE") {
                panic!("Error: Neither args nor environment variables were set.");
            };
            envmnt::get_or_panic("AFI_COOKIE")
        }
    };
    jar.add_cookie_str(cookie_str.as_str(), &url);
    let client_builder = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .cookie_provider(std::sync::Arc::new(jar));
    let client = client_builder.build().unwrap();
    let lesson_info = {
        let html = client
            .get("https://service.jiangsugqt.org/youth/lesson/confirm")
            .send()
            .await?
            .text()
            .await?;
        if !args.in_action {
            println!("{}", html);
        }
        let lesson_reg = Regex::new(r#"'lesson_id':[0-9]+"#)?;
        let lesson = lesson_reg
            .captures(html.as_str())
            .expect("找不到 lesson id");
        let lesson = String::from(lesson.get(0).map_or("", |m| m.as_str()));
        let lesson = lesson.get(12..).expect("lesson_id 处理异常").to_string();
        let token_reg = Regex::new(r#"var token = "(\S)*""#)?;
        let token = token_reg.captures(html.as_str()).expect("找不到 token");
        let token = String::from(token.get(0).map_or("", |m| m.as_str()));
        let token = token
            .get(13..(token.len() - 1))
            .expect("token 处理异常")
            .to_string();
        (lesson, token)
    };
    if !args.in_action {
        println!(
            "成功获取 lesson_id: {}, token: {}",
            lesson_info.0, lesson_info.1
        );
    };
    let req = [
        ("token", lesson_info.1.as_str()),
        ("lesson_id", lesson_info.0.as_str()),
    ];
    let res = client
        .post("https://service.jiangsugqt.org/youth/lesson/confirm")
        .form(&req)
        .send()
        .await?
        .text()
        .await?;
    let v: types::SignReq = serde_json::from_str(res.as_str()).expect(res.as_str());
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
    return Ok(());
}
