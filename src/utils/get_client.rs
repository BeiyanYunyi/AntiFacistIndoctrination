use reqwest::{cookie::Jar, Client, Url};

pub async fn get_client(cookie: &str) -> Result<Client, Box<dyn std::error::Error>> {
  let url = "https://service.jiangsugqt.org".parse::<Url>().unwrap();
  let jar = Jar::default();
  jar.add_cookie_str(cookie, &url);
  let client_builder = reqwest::ClientBuilder::new()
    .cookie_store(true)
    .cookie_provider(std::sync::Arc::new(jar));
  let client = client_builder.build().unwrap();
  return Ok(client);
}
