use clap::{arg, command, Command};
use std::path::Path;
use std::time::Duration;
mod io;
use console::Term;
use strsim::normalized_levenshtein;
mod services;
mod terminal;

fn list_languages() {
    let spinner = terminal::progress_bar::create_spinner("Loading available languages...");
    spinner.enable_steady_tick(Duration::from_millis(64));
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

fn download_subtitle(filename: String, lang: Option<&String>, should_sort: bool) {
    enum ActionState {
        ListingSubtitles,
        ConfirmingSelection,
        DownloadingSubtitle,
        Exiting,
    }

    let spinner = terminal::progress_bar::create_spinner("Loading subtitle results...");
    spinner.enable_steady_tick(Duration::from_millis(64));

    let mut entries = services::opensubtitles::search_subtitles(&filename, lang);
    spinner.finish_and_clear();
    if !should_sort {
        entries.sort_by(|b, a| {
            normalized_levenshtein(&a.attributes.release, &filename)
                .partial_cmp(&normalized_levenshtein(&b.attributes.release, &filename))
                .unwrap()
        });
    }
    let select_entries: Vec<String> = entries
        .iter()
        .map(|entry| {
            format!(
                "{} [{}] [Download count: {}]",
                entry.attributes.release,
                entry.attributes.language,
                entry.attributes.download_count
            )
        })
        .collect();

    let mut state = ActionState::ListingSubtitles;
    let mut selected_subtitle_index: Option<usize> = None;
    let mut selected_file_index: Option<usize> = None;

    // infinite loop
    loop {
        match state {
            ActionState::ListingSubtitles => {
                selected_subtitle_index = terminal::select::create_select(
                    &select_entries,
                    "Which subtitle do you want to download?",
                );

                match selected_subtitle_index {
                    Some(_) => state = ActionState::ConfirmingSelection,
                    None => state = ActionState::Exiting,
                }
            }
            ActionState::ConfirmingSelection => {
                if selected_subtitle_index.is_none() {
                    state = ActionState::Exiting;
                    continue;
                }

                let selected_subtitle_index = selected_subtitle_index.unwrap();
                let selected_subtitle_release = &entries[selected_subtitle_index];

                let select_files = selected_subtitle_release
                    .attributes
                    .files
                    .iter()
                    .map(|file| {
                        format!(
                            "{} [id: {}]",
                            file.file_name,
                            file.file_id.to_string().as_str()
                        )
                    })
                    .collect();

                selected_file_index = terminal::select::create_select(
                    &select_files,
                    "Which file do you want to download?",
                );

                match selected_file_index {
                    Some(_) => state = ActionState::DownloadingSubtitle,
                    None => {
                        Term::stderr().clear_last_lines(1).unwrap_or_default();
                        state = ActionState::ListingSubtitles;
                    }
                }
            }
            ActionState::DownloadingSubtitle => {
                if selected_subtitle_index.is_none() {
                    state = ActionState::Exiting;
                    continue;
                }
                if selected_file_index.is_none() {
                    state = ActionState::Exiting;
                    continue;
                }

                let selected_subtitle_index = selected_subtitle_index.unwrap();
                let selected_file_index = selected_file_index.unwrap();
                let selected_subtitle_release = &entries[selected_subtitle_index];
                let selected_subtitle_file_id =
                    selected_subtitle_release.attributes.files[selected_file_index].file_id;

                let spinner = terminal::progress_bar::create_spinner("Downloading...");
                spinner.enable_steady_tick(Duration::from_millis(64));

                let download_link =
                    services::opensubtitles::get_download_link(selected_subtitle_file_id);

                if download_link.is_none() {
                    spinner.finish_and_clear();
                    println!("Failed to download subtitle");
                    state = ActionState::Exiting;
                    continue;
                }

                let directory = io::get_basename(&filename).unwrap();
                let directory = std::fs::canonicalize(&Path::new({
                    if directory.is_empty() {
                        "./"
                    } else {
                        &directory
                    }
                }))
                .unwrap();

                services::download(&download_link.unwrap(), &filename, &directory);

                state = ActionState::Exiting;
            }
            ActionState::Exiting => {
                break;
            }
        }
    }
}

// SUBCOMMANDS
const LIST_LANGUAGES_CMD: &str = "list-languages";

fn main() {
    let matches = command!()
        .bin_name("subtle")
        .name("subtle")
        .version("1.0.0")
        .author("Alican Erdogan <aerdogan07@gmail.com>")
        .about("subtitle finder for movies and tv series")
        .subcommand_negates_reqs(true)
        .subcommand(Command::new(LIST_LANGUAGES_CMD).about("list available languages"))
        .arg(arg!([filename] "Media file name or search parameter").required(true))
        .arg(
            arg!(
                -l --lang <LANG> "Sets the subtitle language"
            )
            .required(false),
        )
        .arg(arg!(-s --sort "Sorts the results by download count"))
        .get_matches();

    match matches.subcommand_name() {
        Some(LIST_LANGUAGES_CMD) => {
            list_languages();
            return;
        }
        _ => (),
    }

    // throw error if env var is not set
    if !services::opensubtitles::is_env_var_set() {
        println!("Please set the environment variable OPENSUBTITLES_API_KEY");
        return;
    }

    let filename = matches.get_one::<String>("filename");

    if filename.is_none() {
        println!("<filename> is a required parameter.");
        return;
    }

    let filename = filename.unwrap().to_owned();
    let lang = matches.get_one::<String>("lang");
    let should_sort = matches.get_one::<bool>("sort").unwrap_or(&false).to_owned();

    download_subtitle(filename, lang, should_sort);
}
