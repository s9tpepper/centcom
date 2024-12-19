use std::{collections::HashMap, sync::LazyLock};

pub static DEFAULT_APP_THEME: &[u8; include_bytes!("./app_themes/default theme.json").len()] =
    include_bytes!("./app_themes/default theme.json");

pub static GRUVBOX_THEME: &[u8; include_bytes!("./app_themes/gruvbox.json").len()] =
    include_bytes!("./app_themes/gruvbox.json");

pub static APP_THEME_MAP: LazyLock<HashMap<&str, &[u8]>> =
    LazyLock::<HashMap<&str, &[u8]>>::new(|| {
        let mut theme_map: HashMap<&str, &[u8]> = HashMap::new();

        theme_map.insert("default theme", DEFAULT_APP_THEME);
        theme_map.insert("gruvbox", GRUVBOX_THEME);

        theme_map
    });
