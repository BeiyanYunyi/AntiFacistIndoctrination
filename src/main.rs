use regex::Regex;
use reqwest::{cookie::Jar, Url};
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args =  utils::get_args();
    let url = "https://service.jiangsugqt.org".parse::<Url>().unwrap();
    let jar = Jar::default();
    jar.add_cookie_str(
        args.cookie.as_str(),
        &url,
    );
    let client_builder = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .cookie_provider(std::sync::Arc::new(jar));
    let client = client_builder.build().unwrap();
    let token = {
        let html = client
            .get("https://service.jiangsugqt.org/youth/lesson/confirm")
            .send()
            .await?
            .text()
            .await?;
        println!("{}", html);
        let match_str = "var token = \"(\\S)*\"";
        let reg = Regex::new(match_str)?;
        let caps = reg.captures(html.as_str()).expect("找不到 Token");
        let token = String::from(caps.get(0).map_or("", |m| m.as_str()));
        token
            .get(13..(token.len() - 1))
            .expect("Token 处理异常")
            .to_string()
    };
    println!("成功获取 Token: {}", token);
    let req = [("token", token.as_str()), ("lesson_id", "122")];
    let res = client
        .post("https://service.jiangsugqt.org/youth/lesson/confirm")
        .form(&req)
        .send()
        .await?
        .text()
        .await?;
    let v: types::SignReq = serde_json::from_str(res.as_str()).unwrap();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
    return Ok(());
}
