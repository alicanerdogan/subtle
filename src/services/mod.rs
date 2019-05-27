extern crate reqwest;

use std::collections::HashMap;

pub fn call() -> Result<String, Box<std::error::Error>> {
  let mut resp = reqwest::get("https://httpbin.org/ip")?;
  // let json: HashMap<String, String> = resp.json()?;
  let text = resp.text()?;
  Ok(text)
}
