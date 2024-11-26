use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

use crate::fs::get_app_dir;

const DEFAULT_SYNTAX_THEME: &str = "monokai";
const SYNTAX_THEMES_LIST: &str = include_str!("../themes/themes.txt");

#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    pub syntax_theme: String,
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
