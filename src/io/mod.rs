extern crate zip;

use std::fs::{set_permissions, File, Permissions};
use std::io::{copy, BufReader, Read, Seek, SeekFrom};
use std::mem;
use std::path::Path;

#[allow(dead_code)]
const HASH_BLK_SIZE: u64 = 65536;

#[allow(dead_code)]
pub fn read_file(filepath: &str) -> Result<String, std::io::Error> {
  let mut file = File::open(filepath)?;

  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;

  let filesize = file.metadata().unwrap().len();
  let hash = calculate_hash(file, filesize).unwrap();

  Ok(hash)
}

#[allow(dead_code)]
pub fn get_bytesize(filepath: &str) -> std::io::Result<u64> {
  let file = File::open(filepath)?;

  Ok(file.metadata().unwrap().len())
}

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

pub fn get_filename(filepath: &str) -> std::io::Result<String> {
  let path = Path::new(filepath);

  Ok(String::from(path.file_name().unwrap().to_str().unwrap()))
}

fn calculate_hash(file: File, fsize: u64) -> Result<String, std::io::Error> {
  let mut buf = [0u8; 8];
  let mut word: u64;
  let mut hash_val: u64 = fsize; // seed hash with file size

  let iterations = HASH_BLK_SIZE / 8;

  let mut reader = BufReader::with_capacity(HASH_BLK_SIZE as usize, file);

  for _ in 0..iterations {
    reader.read_exact(&mut buf).unwrap();
    unsafe {
      word = mem::transmute(buf);
    };
    hash_val = hash_val.wrapping_add(word);
  }

  reader.seek(SeekFrom::Start(fsize - HASH_BLK_SIZE)).unwrap();

  for _ in 0..iterations {
    reader.read_exact(&mut buf).unwrap();
    unsafe {
      word = mem::transmute(buf);
    };
    hash_val = hash_val.wrapping_add(word);
  }

  let hash_string = format!("{:01$x}", hash_val, 16);

  Ok(hash_string)
}

pub fn remove_file(filepath: &str) {
  std::fs::remove_file(filepath).unwrap();
}

pub fn extract_zip_file(zip_filepath: &str, extracted_file: &str, target_filename: &str) {
  let fname = std::path::Path::new(zip_filepath);
  let file = File::open(&fname).unwrap();

  let mut archive = zip::ZipArchive::new(file).unwrap();

  for i in 0..archive.len() {
    let mut file = archive.by_index(i).unwrap();

    let sanitized_name = file.sanitized_name();
    let fullname = sanitized_name.to_str().unwrap();

    if fullname == extracted_file {
      let outpath = Path::new(zip_filepath)
        .parent()
        .unwrap()
        .join(target_filename)
        .with_extension(sanitized_name.extension().unwrap().to_str().unwrap());

      let mut outfile = File::create(&outpath).unwrap();
      copy(&mut file, &mut outfile).unwrap();

      #[cfg(unix)]
      {
        use std::os::unix::fs::PermissionsExt;

        if let Some(mode) = file.unix_mode() {
          set_permissions(&outpath, Permissions::from_mode(mode)).unwrap();
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn basename() {
    let basename = super::get_basename("").unwrap();
    assert_eq!(basename, "./");
    let basename = super::get_basename("test").unwrap();
    assert_eq!(basename, "./");
    let basename = super::get_basename("test/123").unwrap();
    assert_eq!(basename, "test");
    let basename = super::get_basename("test/123.txt").unwrap();
    assert_eq!(basename, "test");
    let basename = super::get_basename("test/abc/123.txt").unwrap();
    assert_eq!(basename, "test/abc");
    let basename = super::get_basename("/test").unwrap();
    assert_eq!(basename, "/");
    let basename = super::get_basename("/test/123").unwrap();
    assert_eq!(basename, "/test");
    let basename = super::get_basename("/test/123.txt").unwrap();
    assert_eq!(basename, "/test");
    let basename = super::get_basename("/test/abc/123.txt").unwrap();
    assert_eq!(basename, "/test/abc");
  }
}
