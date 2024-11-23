use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

use crate::fs::get_app_dir;

const DEFAULT_SYNTAX_THEME: &str = "themes/monokai.tmTheme";

#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    pub syntax_theme: String,
}

pub fn get_syntax_theme() -> String {
    get_options().syntax_theme
}

pub fn get_default_options() -> Options {
    Options {
        syntax_theme: String::from(DEFAULT_SYNTAX_THEME),
    }
}

pub fn get_options() -> Options {
    match get_app_dir("options.json") {
        Ok(options_dir) => match File::open(options_dir) {
            Ok(file) => {
                let reader = BufReader::new(file);

                let options = serde_json::from_reader(reader);

                match options {
                    Ok(opts) => opts,
                    Err(_) => get_default_options(),
                }
            }
            Err(_) => get_default_options(),
        },

        Err(_) => get_default_options(),
    }
}
