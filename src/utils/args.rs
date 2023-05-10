use std::env;

/// 运行参数
pub struct Args {
  /// Server 酱的 SendKey，用于发送反法西斯成功通知
  pub sct_token: Option<String>,

  /// Cookie, 以空格间隔 例: laravel_session=1145141919810889464364364 laravel_session=11451419
  pub cookie: Vec<String>,
}

impl Args {
  pub fn parse() -> Self {
    let token = env::var("AFI_TOKEN").ok();
    // Fallback to AFI_COOKIE to keep compatibility
    let cookie = env::var("AFI_COOKIE").unwrap_or("".into());
    let cookie = match cookie.as_str() {
      "" => Vec::new(),
      _ => cookie.split(" ").map(|s| s.to_owned()).collect(),
    };
    return Args {
      sct_token: token,
      cookie,
    };
  }
}

pub fn get_args() -> Args {
  return Args::parse();
}
