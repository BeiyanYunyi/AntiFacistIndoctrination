use clap::Parser;

/// 运行参数
#[derive(Parser, Debug)]
#[clap(name = "Anti Facist Indoctrination")]
#[clap(about = "--------------------\n   反对法西斯灌输\n--------------------\n\n人民万岁, 正义永存")]
pub struct Args {
    /// Cookie, 例: laravel_session=1145141919810889464364364
    #[clap(long, short)]
    pub cookie: String,
}

pub fn get_args() -> Args {
    return Args::parse();
}
