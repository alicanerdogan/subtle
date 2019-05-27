extern crate xmlrpc;

use std::collections::BTreeMap;
use std::vec::Vec;

use xmlrpc::{Request, Value};

const ENDPOINT: &str = "https://api.opensubtitles.org:443/xml-rpc";
const USER_AGENT: &str = "subtle-cli v1.0.0";
const USERNAME: &str = "";
const PASSSWORD: &str = "";
const LANG: &str = "en";

pub fn get_token() -> Result<String, Box<std::error::Error>> {
  let resp = Request::new("LogIn")
    .arg(USERNAME)
    .arg(PASSSWORD)
    .arg(LANG)
    .arg(USER_AGENT)
    .call_url(ENDPOINT)
    .unwrap();

  let token = resp.get("token").unwrap().as_str().unwrap();

  Ok(String::from(token))
}

#[derive(Debug)]
pub struct SubtitleEntry {
  pub imdb_id: String,
  pub subtitle_id: String,
  pub subtitle_file_id: String,
  pub lang: String,
  pub movie_size: String,
  pub fps: String,
  pub movie_hash: String,
  pub imdb_rating: String,
  pub movie_title: String,
  pub release_title: String,
  pub duration_in_ms: String,
  pub episode: String,
  pub season: String,
  pub created_at: String,
  pub download_link: String,
  pub download_count: String,
  pub encoding: String,
  pub filename: String,
  pub format: String,
  pub rating: String,
  pub download_link_zip: String,
}

impl SubtitleEntry {
  fn from_value(value: &Value) -> SubtitleEntry {
    SubtitleEntry {
      imdb_id: value
        .get("IDMovieImdb")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      subtitle_id: value
        .get("IDSubtitle")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      subtitle_file_id: value
        .get("IDSubtitleFile")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      lang: value
        .get("LanguageName")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      movie_size: value
        .get("MovieByteSize")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      fps: value.get("MovieFPS").unwrap().as_str().unwrap().to_string(),
      movie_hash: value
        .get("MovieHash")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      imdb_rating: value
        .get("MovieImdbRating")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      movie_title: value
        .get("MovieName")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      release_title: value
        .get("MovieReleaseName")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      duration_in_ms: value
        .get("MovieTimeMS")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      episode: value
        .get("SeriesEpisode")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      season: value
        .get("SeriesSeason")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      created_at: value
        .get("SubAddDate")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      download_link: value
        .get("SubDownloadLink")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      download_count: value
        .get("SubDownloadsCnt")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      encoding: value
        .get("SubEncoding")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      filename: value
        .get("SubFileName")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      format: value
        .get("SubFormat")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      rating: value
        .get("SubRating")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
      download_link_zip: value
        .get("ZipDownloadLink")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(),
    }
  }

  pub fn get_label(&self) -> String {
    let label = format!(
      "{} | Download Count: {}",
      self.lang.to_string(),
      self.download_count.to_string()
    );

    label
  }
}

#[allow(dead_code)]
pub fn search(hash: String, bytesize: u64) {
  let token = get_token().unwrap();
  let mut m = BTreeMap::new();
  m.insert("moviehash".into(), Value::String(hash));
  m.insert("moviebytesize".into(), Value::Int(bytesize as i32));

  let hash_arg = Value::Array(vec![Value::Struct(m)]);

  Request::new("SearchSubtitles")
    .arg(token)
    .arg(hash_arg)
    .call_url(ENDPOINT)
    .unwrap();
}

pub fn search_with_tag(tag: &String) -> Vec<SubtitleEntry> {
  let token = get_token().unwrap();
  let mut m = BTreeMap::new();
  m.insert("query".into(), Value::String(tag.clone()));

  let hash_arg = Value::Array(vec![Value::Struct(m)]);

  let resp = Request::new("SearchSubtitles")
    .arg(token)
    .arg(hash_arg)
    .call_url(ENDPOINT)
    .unwrap();

  let data = resp.get("data").unwrap().as_array().unwrap();

  let entries: Vec<SubtitleEntry> = data
    .into_iter()
    .map(|raw_entry| SubtitleEntry::from_value(raw_entry))
    .collect();

  entries
}

fn get_language_label(entries: &Vec<SubtitleEntry>) -> String {
  let labels: Vec<String> = entries.iter().map(|entry| entry.lang.to_string()).collect();
  let label = labels.join(" | ");

  label
}

pub fn get_subtitle_titles(subtitle_map: &BTreeMap<String, Vec<SubtitleEntry>>) -> Vec<String> {
  let v: Vec<String> = subtitle_map
    .iter()
    .map(|(release_title, entries)| {
      format!(
        "{}  [{}]",
        release_title.to_string().trim(),
        get_language_label(&entries)
      )
    })
    .collect();

  v
}

pub fn get_subtitle_map(entries: Vec<SubtitleEntry>) -> BTreeMap<String, Vec<SubtitleEntry>> {
  let mut entry_map: BTreeMap<String, Vec<SubtitleEntry>> = BTreeMap::new();

  for entry in entries.into_iter() {
    match entry_map.get_mut(&entry.release_title) {
      Some(v) => {
        v.push(entry);
      }
      None => {
        let mut v: Vec<SubtitleEntry> = Vec::new();
        let key = entry.release_title.to_string();
        v.push(entry);
        entry_map.insert(key, v);
      }
    };
  }

  entry_map
}
