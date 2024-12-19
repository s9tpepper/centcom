use std::{
    fs::{self, File},
    io::BufReader,
};

use serde::{Deserialize, Serialize};

use crate::fs::get_app_dir;

const DEFAULT_SYNTAX_THEME: &str = "monokai";
const DEFAULT_APP_THEME: &str = "gruvbox";
const SYNTAX_THEMES_LIST: &str = include_str!("../themes/themes.txt");

#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    pub syntax_theme: String,
    pub app_theme: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct SyntaxTheme {
    pub name: String,
}

pub fn get_syntax_themes() -> Vec<String> {
    let themes: std::str::Lines<'_> = SYNTAX_THEMES_LIST.lines();
    let iterator = themes.into_iter().map(|st| st.to_string());

    iterator.collect()
}

pub fn get_syntax_theme() -> String {
    get_options().syntax_theme
}

pub fn get_app_theme_name() -> String {
    get_options().app_theme
}

pub fn get_default_options() -> Options {
    Options {
        syntax_theme: String::from(DEFAULT_SYNTAX_THEME),
        app_theme: String::from(DEFAULT_APP_THEME),
    }
}

pub fn get_options() -> Options {
    match get_app_dir("options") {
        Ok(mut options_dir) => {
            options_dir.push("options.json");

            match File::open(options_dir) {
                Ok(file) => {
                    let reader = BufReader::new(file);

                    let options = serde_json::from_reader(reader);

                    match options {
                        Ok(opts) => opts,
                        Err(_) => get_default_options(),
                    }
                }
                Err(_) => get_default_options(),
            }
        }

        Err(_) => get_default_options(),
    }
}

pub fn save_options(options: Options) -> anyhow::Result<()> {
    match get_app_dir("options") {
        Ok(mut directory) => {
            directory.push("options.json");

            let contents = serde_json::to_string(&options)?;

            Ok(fs::write(directory, contents)?)
        }
        Err(_) => Err(anyhow::Error::msg("Could not save options")),
    }
}
