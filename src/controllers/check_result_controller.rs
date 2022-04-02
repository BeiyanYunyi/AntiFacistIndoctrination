use crate::utils;
use easy_scraper::Pattern;

pub async fn check_result_controller() -> Result<bool, Box<dyn std::error::Error>> {
  let client = utils::get_client().await?;
  let html = client
    .get("https://service.jiangsugqt.org/youth/report")
    .send()
    .await?
    .text()
    .await?;
  let pat = Pattern::new(
    r#"
<table>
  <tr>
    <td>{{name}}</td>
    <td>
      <span>{{state}}</span>
    </td>
  </tr>
</table>
"#,
  )
  .unwrap();
  let ms = pat.matches(html.as_str());
  match ms.get(0) {
    Some(res) => {
      if res["state"] == "未学习" {
        println!("正在完成本周任务");
        return Ok(false);
      }
    }
    None => {
      panic!("无法查询成绩，请检查 Cookie 是否正确");
    }
  };
  println!("本周任务已经被完成了");
  return Ok(true);
}
