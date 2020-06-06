extern crate clap;
extern crate indicatif;

mod io;
mod services;
mod terminal;

use clap::{App, Arg, SubCommand};
use terminal::progress_bar;

// SUBCOMMANDS
const LIST_LANGUAGES_CMD: &str = "list-languages";

// OPTION ARGS
const LANG_OPT: &str = "lang";

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
        .arg(
            Arg::with_name(LANG_OPT)
                .help("subtitle language")
                .takes_value(true)
                .short("l")
                .long("lang")
                .multiple(true)
                .required(false)
                .requires("FILE"),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some(LIST_LANGUAGES_CMD) => {
            list_languages();
            return;
        }
        _ => (),
    }

    let desired_languages: Option<Vec<String>> = {
        if !matches.is_present(LANG_OPT) {
            None
        } else {
            let languages = services::opensubtitles::get_languages().unwrap();
            let language_map = services::opensubtitles::get_languages_map(&languages).unwrap();
            let lang_opts: Vec<String> = matches
                .values_of(LANG_OPT)
                .unwrap()
                .map(|opt| language_map.get(opt))
                .filter(|&language_name| language_name != None)
                .map(|language_name| language_name.unwrap())
                .map(|language_name| language_name.to_string())
                .collect();

            if lang_opts.len() == 0 {
                None
            } else {
                Some(lang_opts)
            }
        }
    };

    let file = matches.value_of("FILE").unwrap();
    println!("{}", file);

    let filename = io::get_filename(file).unwrap();
    println!("Filename: {}", filename);

    let spinner = progress_bar::create_spinner("Loading...");
    spinner.enable_steady_tick(64);
    let entries = services::opensubtitles::search_with_tag(&filename);
    let entries: Vec<services::opensubtitles::SubtitleEntry> = match desired_languages {
        Some(desired_languages) => entries
            .into_iter()
            .filter(|entry| desired_languages.iter().any(|opt| opt == &entry.lang))
            .collect(),
        None => entries,
    };
    let entries_map = services::opensubtitles::get_subtitle_map(entries);
    let entry_titles = services::opensubtitles::get_subtitle_titles(&entries_map);
    spinner.finish_and_clear();

    if entry_titles.len() == 0 {
        println!("No subtitles were found.");
        return;
    }

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
