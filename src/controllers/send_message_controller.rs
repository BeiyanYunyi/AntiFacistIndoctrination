use crate::utils::{get_args, send_message};

/// ## send_message_controller
///
/// 控制 Server 酱的通知推送
pub async fn send_message_controller(
    title: &str,
    msg: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let args = get_args();
    let mut title = format!("[AFI] {}", title);
    title.truncate(32); // 标题长度限制为 32
    match args.sct_token {
        Some(token) => {
            send_message(title, msg, token.as_str()).await?;
            return Ok(());
        }
        None => return Ok(()),
    };
}
