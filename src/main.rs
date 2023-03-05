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
  let result = controllers::check_result_controller().await?;
  match result {
    Studied => {
      controllers::send_message_controller("运行结果：重复", Some("本周已经学过了")).await?;
      return Ok(());
    }
    NotStudied(id) => {
      controllers::antifa_controller(id).await?;
      let result = controllers::check_result_controller().await?;
      match result {
        Studied => {
          controllers::send_message_controller("运行结果：成功", Some("学习成功")).await?;
          return Ok(());
        }
        NotStudied(_) => {
          const ERROR_MSG: &str = "请求发送了，查询时却没有学习记录，建议自行查询学习状态";
          controllers::send_message_controller("运行结果：错误", Some(ERROR_MSG)).await?;
          panic!("{}", ERROR_MSG);
        }
      }
    }
  }
}
