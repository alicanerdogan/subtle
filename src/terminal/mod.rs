pub mod progress_bar;
pub mod select;

use console::style;

#[allow(dead_code)]
pub fn get_blue_styled_text(text: &str) -> String {
  format!("{}", style(text).blue())
}

#[allow(dead_code)]
pub fn get_bold_text(text: &str) -> String {
  format!("{}", style(text).bold())
}
