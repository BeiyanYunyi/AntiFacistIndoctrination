use crate::utils::get_client;

/// ## send_message
///
/// 发送 Server 酱消息
pub async fn send_message(
    title: String,
    msg: Option<String>,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://sctapi.ftqq.com/{}.send", token);
    let client = get_client().await?;
    let mut params = Vec::new();
    params.push(("title", title));
    if msg.is_some() {
        params.push(("desp", msg.unwrap()));
    }
    client.post(url).form(&params).send().await?;
    return Ok(());
}
