use dialoguer::{theme::ColorfulTheme, Select};

pub fn create_select<S: Into<String>>(selections: &Vec<String>, prompt: S) -> Option<usize> {
    let interaction = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(&prompt.into())
        .default(0)
        .items(&selections)
        .interact_opt();

    match interaction {
        Ok(selection) => selection,
        Err(_) => None,
    }
}
