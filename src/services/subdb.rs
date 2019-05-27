extern crate reqwest;

#[allow(dead_code)]
const ENDPOINT: &str = "https://api.thesubdb.com";

#[allow(dead_code)]
pub fn search(hash: &str) -> Result<String, Box<std::error::Error>> {
  let url = format!("{0}?action=search&hash={1}", ENDPOINT, hash);

  let mut resp = reqwest::Client::new()
    .get(&url)
    .header("User-Agent", "subtle")
    .send()?;

  let text = resp.text()?;
  Ok(text)
}
