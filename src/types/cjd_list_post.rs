use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ApiCjdListReq {
  pub page: u32,
  pub limit: u32,
}

#[derive(Deserialize, Serialize)]
pub struct ApiCjdListResData {
  pub id: u32,
  pub title: String,
  pub has_learn: String,
}

#[derive(Deserialize, Serialize)]
pub struct ApiCjdListRes {
  message: String,
  status: u32,
  redirect: String,
  pub data: Vec<ApiCjdListResData>,
}
