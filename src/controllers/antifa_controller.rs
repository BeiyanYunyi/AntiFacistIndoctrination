use crate::api;
use crate::types::ApiDoLessonRes;

pub async fn antifa_controller(
  id: u32,
  cookie: &str,
) -> Result<ApiDoLessonRes, Box<dyn std::error::Error>> {
  let res = api::api_do_lesson_post(id, cookie).await?;
  println!("学习结果：{}", serde_json::to_string_pretty(&res)?);
  return Ok(res);
}
