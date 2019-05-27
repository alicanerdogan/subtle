extern crate reqwest;

pub mod opensubtitles;
pub mod subdb;

#[allow(dead_code)]
pub fn call() -> Result<String, Box<std::error::Error>> {
  let mut resp = reqwest::get("https://httpbin.org/ip")?;
  // let json: HashMap<String, String> = resp.json()?;
  let text = resp.text()?;
  Ok(text)
}

use std::fs::File;
use std::io::copy;
use std::path::Path;

fn get_filename_from_response(response: &reqwest::Response) -> &str {
  response
    .url()
    .path_segments()
    .and_then(|segments| segments.last())
    .and_then(|name| if name.is_empty() { None } else { Some(name) })
    .unwrap_or("sub.zip")
}

pub fn download(url: &str, directory: &str) -> String {
  let mut response = reqwest::get(url).unwrap();

  let mut tuple = {
    let filename = { get_filename_from_response(&response) };
    let filepath = std::fs::canonicalize(&Path::new(directory))
      .unwrap()
      .join(Path::new(filename));
    (
      String::from(filepath.to_str().unwrap()),
      File::create(filepath).unwrap(),
    )
  };

  copy(&mut response, &mut tuple.1).unwrap();

  tuple.0
}
