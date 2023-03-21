pub mod opensubtitles;

use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

fn get_filename_from_response(response: &reqwest::blocking::Response) -> &str {
    response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("sub.zip")
}

pub fn download(url: &str, source_filename: &str, directory: &PathBuf) {
    let mut response = reqwest::blocking::get(url).unwrap();

    let filename = get_filename_from_response(&response);
    let filename_buffer = PathBuf::from(filename);
    let extension = filename_buffer
        .extension()
        .map(|ext| ext.to_str())
        .unwrap_or(Some("str"))
        .unwrap();

    // if source_filename is empty, use the filename from the response
    let filename = if source_filename.is_empty() {
        String::from(filename)
    } else {
        // otherwise, use the source_filename
        // and remove the extension from the source_filename
        let source_filename_filestem = PathBuf::from(source_filename);
        let filestem = source_filename_filestem
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        String::from(filestem)
    };
    let filepath = directory.join(format!("{}.{}", filename, extension));

    let mut file = File::create(filepath).unwrap();
    copy(&mut response, &mut file).unwrap();
}
