use crate::{
  types::{ApiCjdListReq, ApiCjdListRes},
  utils,
};

pub async fn api_cjd_list_post(
  page: u32,
  limit: u32,
) -> Result<ApiCjdListRes, Box<dyn std::error::Error>> {
  let req = ApiCjdListReq { page, limit };
  let client = utils::get_client().await?;
  let html = client
    .post("https://service.jiangsugqt.org/api/cjdList")
    .json(&req)
    .send()
    .await?
    .text()
    .await?;
  let json = serde_json::from_str::<ApiCjdListRes>(&html)?;
  Ok(json)
}
