/// ## send_message
///
/// 发送 Server 酱消息
pub async fn send_message(
  title: String,
  msg: Option<&str>,
  token: String,
) -> Result<(), Box<dyn std::error::Error>> {
  let url = format!("https://sctapi.ftqq.com/{}.send", token);
  let client = reqwest::ClientBuilder::new().build()?;
  let mut params = Vec::new();
  params.push(("title", title));
  if msg.is_some() {
    let msg = format!(
      "这是一条来自 AFI 的推送消息，以下是正文：\n\n{}",
      msg.unwrap()
    );
    params.push(("desp", msg));
  }
  client.post(url).form(&params).send().await?;
  return Ok(());
}
