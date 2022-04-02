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
    if !result {
        controllers::antifa_controller().await?;
        let result = controllers::check_result_controller().await?;
        if !result {
            panic!("学了却没有学习记录，建议自查");
        }
    }
    return Ok(());
}
