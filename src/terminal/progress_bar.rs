use indicatif::{ProgressBar, ProgressStyle};

pub fn create_spinner(msg: &str) -> ProgressBar {
  let spinner = ProgressBar::new_spinner();

  spinner.set_message(msg);
  spinner.set_style(ProgressStyle::default_spinner());

  spinner
}
