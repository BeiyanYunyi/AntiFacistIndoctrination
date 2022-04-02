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
    if ms[0]["state"] == "未学习" {
        println!("正在完成本周任务");
        return Ok(false);
    };
    println!("本周任务已经被完成了，无需重复运行");
    return Ok(true);
}
