use std::{collections::HashMap, sync::LazyLock};

pub static CATPPUCCIN_THEME: &[u8; include_bytes!("./app_themes/catppuccin.json").len()] =
    include_bytes!("./app_themes/catppuccin.json");

pub static GRUVBOX_THEME: &[u8; include_bytes!("./app_themes/gruvbox.json").len()] =
    include_bytes!("./app_themes/gruvbox.json");

pub static TOKYONIGHT_THEME: &[u8; include_bytes!("./app_themes/tokyonight.json").len()] =
    include_bytes!("./app_themes/tokyonight.json");

pub static KANAGAWA_THEME: &[u8; include_bytes!("./app_themes/kanagawa.json").len()] =
    include_bytes!("./app_themes/kanagawa.json");

pub static APP_THEME_MAP: LazyLock<HashMap<&str, &[u8]>> =
    LazyLock::<HashMap<&str, &[u8]>>::new(|| {
        let mut theme_map: HashMap<&str, &[u8]> = HashMap::new();

        theme_map.insert("catppuccin", CATPPUCCIN_THEME);
        theme_map.insert("gruvbox", GRUVBOX_THEME);
        theme_map.insert("tokyonight", TOKYONIGHT_THEME);
        theme_map.insert("kanagawa", KANAGAWA_THEME);

        theme_map
    });
