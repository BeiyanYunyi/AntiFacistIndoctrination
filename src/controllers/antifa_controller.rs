use crate::{types, utils};
use regex::Regex;

pub async fn antifa_controller() -> Result<(), Box<dyn std::error::Error>> {
  let args = utils::get_args();
  let client = utils::get_client().await?;
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
