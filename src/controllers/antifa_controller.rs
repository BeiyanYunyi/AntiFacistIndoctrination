use crate::{api, utils};

pub async fn antifa_controller(id: u32) -> Result<(), Box<dyn std::error::Error>> {
  let args = utils::get_args();
  let res = api::api_do_lesson_post(id).await?;
  println!("学习结果：{}", serde_json::to_string_pretty(&res)?);
  return Ok(());
}
