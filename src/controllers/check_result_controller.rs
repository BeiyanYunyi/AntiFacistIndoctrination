use crate::api;

pub enum CheckResultRes {
  NotStudied(u32),
  Studied,
}

pub async fn check_result_controller(
  cookie: &str,
) -> Result<CheckResultRes, Box<dyn std::error::Error>> {
  let cjd_res = api::api_cjd_list_post(1, 20, cookie).await;
  match cjd_res {
    Ok(res) => {
      if let Some(bb) = res.data.get(0) {
        if bb.has_learn == "0" {
          println!("正在完成本周任务");
          return Ok(CheckResultRes::NotStudied(bb.id));
        }
      }
    }
    _ => {
      panic!("无法查询成绩，请检查 Cookie 是否正确");
    }
  };
  println!("本周任务已经被完成了");
  return Ok(CheckResultRes::Studied);
}
