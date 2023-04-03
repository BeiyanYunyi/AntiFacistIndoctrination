use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ApiDoLessonReq {
  pub lesson_id: u32,
}

#[derive(Deserialize, Serialize)]
pub struct ApiDoLessonResData {
  title: String,
  pub(crate) url: String,
}

#[derive(Deserialize, Serialize)]
pub struct ApiDoLessonRes {
  message: String,
  status: u32,
  redirect: String,
  pub(crate) data: ApiDoLessonResData,
}
