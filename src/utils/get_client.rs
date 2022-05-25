use crate::utils::args;
use reqwest::{cookie::Jar, Client, Url};

pub async fn get_client() -> Result<Client, Box<dyn std::error::Error>> {
  let args = args::get_args();
  let url = "https://service.jiangsugqt.org".parse::<Url>().unwrap();
  let jar = Jar::default();
  jar.add_cookie_str(args.cookie.as_str(), &url);
  let client_builder = reqwest::ClientBuilder::new()
    .cookie_store(true)
    .cookie_provider(std::sync::Arc::new(jar));
  let client = client_builder.build().unwrap();
  return Ok(client);
}
