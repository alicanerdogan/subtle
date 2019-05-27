extern crate crypto;


use crypto::digest::Digest;
use crypto::md5::Md5;

use std::fs::File;
use std::io::Read;

pub fn read_file(filepath: &str) -> std::io::Result<String> {
  let mut file = File::open(filepath)?;

  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;

  let mut md5 = Md5::new();
  md5.input(buffer.as_ref());

  let hash = md5.result_str();

  Ok(hash)
}
