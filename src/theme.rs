use anathema::state::{State, Value};
use serde::{Deserialize, Serialize};

use crate::{
    app_themes::{APP_THEME_MAP, DEFAULT_APP_THEME},
    options::get_app_theme_name,
};

#[derive(State)]
pub struct AppTheme {
    /// Theme name
    pub name: Value<String>,
    /// Application background
    pub background: Value<String>,
    /// Main font color
    pub foreground: Value<String>,
    /// Color of borders when not focused
    pub border_unfocused: Value<String>,
    /// Color of borders when focused
    pub border_focused: Value<String>,
    /// Color of top bar background
    pub top_bar_background: Value<String>,
    /// Color of top bar font color
    pub top_bar_foreground: Value<String>,
    /// Color of bottom bar background
    pub bottom_bar_background: Value<String>,
    /// Color of bottom bar font color
    pub bottom_bar_foreground: Value<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppThemePersisted {
    /// Theme name
    pub name: String,
    /// Application background
    pub background: String,
    /// Main font color
    pub foreground: String,
    /// Color of borders when not focused
    pub border_unfocused: String,
    /// Color of borders when focused
    pub border_focused: String,
    /// Color of top bar background
    pub top_bar_background: String,
    /// Color of top bar font color
    pub top_bar_foreground: String,
    /// Color of bottom bar background
    pub bottom_bar_background: String,
    /// Color of bottom bar font color
    pub bottom_bar_foreground: String,
}

impl From<AppTheme> for AppThemePersisted {
    fn from(theme: AppTheme) -> Self {
        AppThemePersisted {
            name: theme.name.to_ref().to_string(),
            background: theme.background.to_ref().to_string(),
            foreground: theme.foreground.to_ref().to_string(),
            border_unfocused: theme.border_unfocused.to_ref().to_string(),
            border_focused: theme.border_focused.to_ref().to_string(),
            top_bar_background: theme.top_bar_background.to_ref().to_string(),
            top_bar_foreground: theme.top_bar_foreground.to_ref().to_string(),
            bottom_bar_background: theme.bottom_bar_background.to_ref().to_string(),
            bottom_bar_foreground: theme.bottom_bar_foreground.to_ref().to_string(),
        }
    }
}

impl From<AppThemePersisted> for AppTheme {
    fn from(theme_persisted: AppThemePersisted) -> Self {
        AppTheme {
            name: theme_persisted.name.into(),
            background: theme_persisted.background.into(),
            foreground: theme_persisted.foreground.into(),
            border_unfocused: theme_persisted.border_unfocused.into(),
            border_focused: theme_persisted.border_focused.into(),
            top_bar_background: theme_persisted.top_bar_background.into(),
            top_bar_foreground: theme_persisted.top_bar_foreground.into(),
            bottom_bar_background: theme_persisted.bottom_bar_background.into(),
            bottom_bar_foreground: theme_persisted.bottom_bar_foreground.into(),
        }
    }
}

pub fn get_app_theme() -> AppTheme {
    let app_theme_name = get_app_theme_name();
    let app_theme_opt = APP_THEME_MAP.get_key_value(&app_theme_name.as_str());

    match app_theme_opt {
        Some((_, app_theme_bytes)) => {
            let app_theme_contents = String::from_utf8_lossy(app_theme_bytes);

            let app_theme = serde_json::from_str::<AppThemePersisted>(&app_theme_contents);
            match app_theme {
                Ok(theme) => theme.into(),
                Err(_) => get_default_app_theme(),
            }
        }

        None => get_default_app_theme(),
    }
}

fn get_default_app_theme() -> AppTheme {
    let default_app_theme_contents = String::from_utf8_lossy(DEFAULT_APP_THEME);
    serde_json::from_str::<AppThemePersisted>(&default_app_theme_contents)
        .unwrap()
        .into()
}
