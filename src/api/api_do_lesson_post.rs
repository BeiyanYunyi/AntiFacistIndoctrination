use crate::{
  types::{ApiDoLessonReq, ApiDoLessonRes},
  utils,
};

pub async fn api_do_lesson_post(
  lesson_id: u32,
  cookie: &str,
) -> Result<ApiDoLessonRes, Box<dyn std::error::Error>> {
  let req = ApiDoLessonReq { lesson_id };
  let client = utils::get_client(cookie).await?;
  let html = client
    .post("https://service.jiangsugqt.org/api/doLesson")
    .json(&req)
    .send()
    .await?
    .text()
    .await?;
  let json = serde_json::from_str::<ApiDoLessonRes>(&html)?;
  Ok(json)
}
