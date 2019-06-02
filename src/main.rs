extern crate clap;
extern crate indicatif;

mod io;
mod services;
mod terminal;

use clap::{App, Arg, SubCommand};
use terminal::progress_bar;

// SUBCOMMANDS
const LIST_LANGUAGES_CMD: &str = "list-languages";

fn main() {
  let matches = App::new("subtle")
    .version("0.1.0")
    .author("Alican Erdogan <aerdogan07@gmail.com>")
    .about("subtitle finder for movies and tv series")
    .subcommand(SubCommand::with_name(LIST_LANGUAGES_CMD).about("list available languages"))
    .arg(
      Arg::with_name("FILE")
        .required(false)
        .takes_value(true)
        .index(1)
        .help("media file"),
    )
    .get_matches();

  match matches.subcommand_name() {
    Some(LIST_LANGUAGES_CMD) => {
      list_languages();
      return;
    }
    _ => (),
  }

  let file = matches.value_of("FILE").unwrap();
  println!("{}", file);

  let filename = io::get_filename(file).unwrap();
  println!("Filename: {}", filename);

  let spinner = progress_bar::create_spinner("Loading...");
  spinner.enable_steady_tick(64);
  let entries = services::opensubtitles::search_with_tag(&filename);
  let entries_map = services::opensubtitles::get_subtitle_map(entries);
  let entry_titles = services::opensubtitles::get_subtitle_titles(&entries_map);
  spinner.finish_and_clear();

  let selected_subtitle_index =
    terminal::select::create_select(&entry_titles, "Which subtitle do you want to download?")
      .unwrap();

  let (_, subtitle_entries) = entries_map.iter().nth(selected_subtitle_index).unwrap();

  let subtitle_entries_labels = subtitle_entries
    .iter()
    .map(|entry| entry.get_label())
    .collect();
  let selected_subtitle_entry_index = terminal::select::create_select(
    &subtitle_entries_labels,
    "Which language do you want to download?",
  )
  .unwrap();
  let selected_subtitle_entry = subtitle_entries
    .iter()
    .nth(selected_subtitle_entry_index)
    .unwrap();

  let spinner = progress_bar::create_spinner("Downloading...");
  spinner.enable_steady_tick(64);
  let zip_path = services::download(
    &selected_subtitle_entry.download_link_zip,
    &io::get_basename(&file).unwrap(),
  );
  spinner.finish_and_clear();

  io::extract_zip_file(&zip_path, &selected_subtitle_entry.filename, &filename);
  io::remove_file(&zip_path);
  println!("Done: {}", &selected_subtitle_entry.filename);
}

fn list_languages() {
  let spinner = progress_bar::create_spinner("Loading available languages...");
  spinner.enable_steady_tick(64);
  let languages = services::opensubtitles::get_languages().unwrap();
  spinner.finish_and_clear();
  for lang in languages {
    println!(
      "{} | {} {}",
      &lang.name,
      &lang.iso,
      console::Emoji(
        &terminal::emojis::get_flag_emoji(&lang.iso).unwrap_or(String::new()),
        ""
      )
    );
  }
}
