use clap::Parser;

/// 运行参数
#[derive(Parser, Debug)]
#[clap(name = "Anti Facist Indoctrination")]
#[clap(
    about = "--------------------\n   反对法西斯灌输\n--------------------\n\n人民万岁, 正义永存"
)]
#[clap(version, author = "野兽先辈")]
pub struct Args {
    /// Cookie, 例: laravel_session=1145141919810889464364364
    #[clap(long, short)]
    pub cookie: String,

    /// 在 Action 中运行，不输出隐私信息
    #[clap(long, short)]
    pub in_action: bool,

    /// Server 酱的 SendKey，用于发送反法西斯成功通知
    #[clap(long, short)]
    pub sct_token: Option<String>,
}

pub fn get_args() -> Args {
    return Args::parse();
}
