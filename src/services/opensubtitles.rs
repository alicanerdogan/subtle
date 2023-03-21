use serde::{Deserialize, Serialize};
use std::vec::Vec;

const ENDPOINT: &str = "https://api.opensubtitles.org:443/xml-rpc";

const ENV_VAR: &str = "OPEN_SUBTITLES_API_KEY";

fn get_api_key() -> String {
    let compile_time_api_key = option_env!("OPEN_SUBTITLES_API_KEY");
    match compile_time_api_key {
        Some(api_key) => api_key.to_string(),
        None => std::env::var(&ENV_VAR).unwrap(),
    }
}

pub fn is_env_var_set() -> bool {
    let compile_time_api_key = get_api_key();
    compile_time_api_key != ""
}

#[derive(Debug)]
pub struct Language {
    pub id: String,
    pub name: String,
    pub iso: String,
}

impl Language {
    fn from_value(value: &xmlrpc::Value) -> Language {
        Language {
            id: value
                .get("SubLanguageID")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            name: value
                .get("LanguageName")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            iso: value.get("ISO639").unwrap().as_str().unwrap().to_string(),
        }
    }
}

pub fn get_languages() -> Result<Vec<Language>, Box<dyn std::error::Error>> {
    let resp = xmlrpc::Request::new("GetSubLanguages")
        .call_url(ENDPOINT)
        .unwrap();

    let data = resp.get("data").unwrap().as_array().unwrap();

    let languages: Vec<Language> = data
        .into_iter()
        .map(|raw_entry| Language::from_value(raw_entry))
        .collect();

    Ok(languages)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    total_pages: i32,
    total_count: i32,
    per_page: i32,
    page: i32,
    data: Vec<SubtitleData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubtitleData {
    pub id: String,
    #[serde(rename = "type")]
    pub subtitle_type: String,
    pub attributes: Attributes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    pub subtitle_id: String,
    pub language: String,
    pub download_count: i32,
    pub new_download_count: i32,
    pub hearing_impaired: bool,
    pub hd: bool,
    pub fps: f32,
    pub votes: i32,
    pub ratings: f32,
    pub from_trusted: bool,
    pub foreign_parts_only: bool,
    pub upload_date: String,
    pub ai_translated: bool,
    pub machine_translated: bool,
    pub release: String,
    pub comments: String,
    pub legacy_subtitle_id: i32,
    pub uploader: Uploader,
    pub feature_details: FeatureDetails,
    pub url: String,
    pub related_links: Vec<RelatedLink>,
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Uploader {
    pub uploader_id: Option<i32>,
    pub name: String,
    pub rank: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureDetails {
    pub feature_id: i32,
    pub feature_type: String,
    pub year: i32,
    pub title: String,
    pub movie_name: String,
    pub imdb_id: i32,
    pub tmdb_id: Option<i32>,
    pub season_number: i32,
    pub episode_number: i32,
    pub parent_imdb_id: i32,
    pub parent_title: String,
    pub parent_tmdb_id: Option<i32>,
    pub parent_feature_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedLink {
    pub label: String,
    pub url: String,
    pub img_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub file_id: i32,
    pub cd_number: i32,
    pub file_name: String,
}

pub fn search_subtitles(subtitle_query: &String, lang: Option<&String>) -> Vec<SubtitleData> {
    let url = format!(
        "https://api.opensubtitles.com/api/v1/subtitles?languages={}&query={}",
        urlencoding::encode(lang.unwrap_or(&"".to_string())),
        urlencoding::encode(subtitle_query)
    );

    let client = reqwest::blocking::Client::new();

    let response = client
        .get(url)
        .header("Api-Key", get_api_key())
        .header("Content-Type", "application/json")
        .send();

    if response.is_err() {
        println!("Error: {:?}", response.err());
        return vec![];
    }

    let response = response.unwrap();

    if reqwest::StatusCode::OK != response.status() {
        println!("Error: {:?}", response.status());

        // print the body
        let body = response.text();
        if body.is_ok() {
            println!("Body: {:?}", body.unwrap());
        }
        return vec![];
    }

    let payload: Payload = match response.json() {
        Ok(payload) => payload,
        Err(e) => {
            println!("Error: {:?}", e);
            return vec![];
        }
    };

    return payload.data;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadPayload {
    pub link: String,
    pub requests: i32,
    pub remaining: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadRequestPayload {
    pub file_id: i32,
}

pub fn get_download_link(file_id: i32) -> Option<String> {
    let url = format!("https://api.opensubtitles.com/api/v1/download");

    let client = reqwest::blocking::Client::new();

    let data = DownloadRequestPayload { file_id: file_id };

    let response = client
        .post(url)
        .json(&data)
        .header("Api-Key", get_api_key())
        .header("Content-Type", "application/json")
        .send();

    if response.is_err() {
        println!("Error: {:?}", response.err());
        return None;
    }

    let response = response.unwrap();

    if reqwest::StatusCode::OK != response.status() {
        println!("Error: {:?}", response.status());

        // print the body
        let body = response.text();
        if body.is_ok() {
            println!("Body: {:?}", body.unwrap());
        }
        return None;
    }

    let payload: DownloadPayload = match response.json() {
        Ok(payload) => payload,
        Err(e) => {
            println!("Error: {:?}", e);
            return None;
        }
    };

    return Some(payload.link);
}
