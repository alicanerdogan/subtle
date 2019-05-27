extern crate clap;
extern crate indicatif;

mod console;
mod io;
mod services;

use clap::{App, Arg};
use console::progress_bar;

use std::{thread, time};

fn sleep1s() {
  let ten_millis = time::Duration::from_secs(1);
  thread::sleep(ten_millis);
}

fn main() {
  let matches = App::new("subfind")
    .version("0.1.0")
    .author("Alican Erdogan <aerdogan07@gmail.com>")
    .about("subtitle finder for files")
    .arg(
      Arg::with_name("FILE")
        .required(true)
        .takes_value(true)
        .index(1)
        .help("url to download"),
    )
    .get_matches();
  let file = matches.value_of("FILE").unwrap();
  println!("{}", file);

  let hash = io::read_file(file).unwrap();
  println!("{}", hash);

  let spinner = progress_bar::create_spinner("Loading...");
  spinner.enable_steady_tick(64);
  let text = services::call().unwrap();
  sleep1s();
  spinner.finish_and_clear();

  println!("{}", text);

  console::select::create_select();
}
