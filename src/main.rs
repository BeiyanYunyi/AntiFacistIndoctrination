use crate::controllers::CheckResultRes::{NotStudied, Studied};

mod api;
mod controllers;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!(
    r#"
资产阶级国家的形式虽然多种多样，但本质是一样的：所有这些国家，不管怎样，归根到底一定都是资产阶级专政。
——《国家与革命》，列宁
        "#
  );
  let args = utils::get_args();
  if args.cookie.is_empty() {
    println!("请在命令行中输入cookie");
    return Ok(());
  }
  let mut res_ary = Vec::new();
  let len = args.cookie.len();
  for ck in args.cookie {
    let result = controllers::check_result_controller(ck.as_str()).await?;
    match result {
      Studied => {
        res_ary.push("重复");
      }
      NotStudied(id) => {
        controllers::antifa_controller(id, ck.as_str()).await?;
        let result = controllers::check_result_controller(ck.as_str()).await?;
        match result {
          Studied => {
            res_ary.push("成功");
          }
          NotStudied(_) => {
            res_ary.push("请求发送了，查询时却没有学习记录，建议自行查询学习状态");
          }
        }
      }
    }
    if len > 1 && res_ary.len() < len {
      println!("等待 3 秒后继续运行下一个任务");
      tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
  }
  println!("运行结果：{:?}", res_ary);
  controllers::send_message_controller(
    format!("运行结果：{:?}", res_ary).as_str(),
    Some(format!("{:?}", res_ary).as_str()),
  )
  .await?;
  Ok(())
}
