use crate::{
  types::{ApiCjdListReq, ApiCjdListRes},
  utils,
};

pub async fn api_cjd_list_post(
  page: u32,
  limit: u32,
  cookie: &str,
) -> Result<ApiCjdListRes, Box<dyn std::error::Error>> {
  let req = ApiCjdListReq { page, limit };
  let client = utils::get_client(cookie).await?;
  let res = client
    .post("https://service.jiangsugqt.org/api/cjdList")
    .json(&req)
    .send()
    .await?;
  let new_cookie = match res.headers().get("set-cookie") {
    Some(v) => {
      let header = v.to_str()?.to_owned();
      let mut cookie = String::from(header.split(";").next().unwrap());
      cookie.push_str(";");
      cookie
    }
    None => String::from(""),
  };
  if cookie != new_cookie {
    println!("检测到 Cookie 更新");
  }
  let html = res.text().await?;
  let json_res = serde_json::from_str::<ApiCjdListRes>(&html);
  match json_res {
    Ok(json) => Ok(json),
    Err(e) => {
      println!("{:?},{}", e, html);
      Err(Box::new(e))
    }
  }
}
