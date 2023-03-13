use clap::Parser;

/// 运行参数
#[derive(Parser, Debug)]
#[clap(name = "Anti Facist Indoctrination")]
#[clap(
  about = "--------------------\n   反对法西斯灌输\n--------------------\n\n人民万岁, 正义永存"
)]
#[clap(version, author = "野兽先辈")]
pub struct Args {
  /// Server 酱的 SendKey，用于发送反法西斯成功通知
  #[clap(long, short)]
  pub sct_token: Option<String>,

  /// Cookie, 以空格间隔 例: laravel_session=1145141919810889464364364 laravel_session=11451419
  pub cookie: Vec<String>,
}

pub fn get_args() -> Args {
  return Args::parse();
}
