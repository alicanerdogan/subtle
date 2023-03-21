use std::path::Path;

pub fn get_basename(filepath: &str) -> std::io::Result<String> {
    let path = Path::new(filepath)
        .parent()
        .unwrap_or_else(|| Path::new("./"));

    let path = path.to_str().unwrap();

    Ok(String::from({
        if path.is_empty() {
            "./"
        } else {
            path
        }
    }))
}
