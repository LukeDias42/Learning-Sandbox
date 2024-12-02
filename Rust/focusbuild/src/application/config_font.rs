use crate::models::settings::{
    FontSize::{self, Auto, Large, Small, Tiny},
    Language, Settings, Theme,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ConfigFont {
    pub title: Title,
    pub settings_options: SettingsOptions,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Title {
    pub width: u16,
    pub height: u16,
    pub text: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SettingsOptions {
    pub width: u16,
    pub height: u16,
    pub total_height: u16,
    pub longest_possible_config: u16,
    pub title_option_map: Vec<(String, String)>,
    pub vertical_gap: u16,
}

impl Title {
    fn new(logo: String) -> Self {
        let height = logo.lines().count() as u16;
        let width = logo.lines().last().unwrap_or("").chars().count() as u16;
        Self {
            width,
            height,
            text: logo,
        }
    }
}

impl SettingsOptions {
    fn new(
        settings_options: [&str; 4],
        theme_options: [(Theme, &str); 3],
        language_options: [(Language, &str); 2],
        font_size_options: [(FontSize, &str); 4],
        current_settings: Settings,
        gap: u16,
    ) -> Self {
        let height = settings_options[0].lines().count() as u16;
        let width = settings_options[0]
            .to_string()
            .lines()
            .last()
            .unwrap_or("")
            .chars()
            .count() as u16;
        let total_height = (height + gap) * keybinds.len() as u16;
        let longest_description = keybinds
            .map(|(_, desc)| {
                desc.lines()
                    .map(|line| line.chars().count())
                    .max()
                    .unwrap_or(0)
            })
            .iter()
            .max()
            .unwrap_or(&0)
            .to_owned() as u16;
        let key_desc_map = keybinds
            .map(|(key, desc)| (key.to_string(), desc.to_string()))
            .to_vec();

        Self {
            width,
            height,
            longest_description,
            key_desc_map,
            total_height,
            vertical_gap: gap,
        }
    }
}

impl ConfigFont {
    pub fn new(width: u16, height: u16, settings: Settings) -> Self {
        let large = Self::new_large();
        let small = Self::new_small();
        let tiny = Self::new_tiny();
        match settings.font_size {
            Auto => {
                if large.logo.width < width
                    && large.keybinding.longest_description + 2 < width
                    && large.keybinding.total_height + 2 < height
                {
                    return large;
                } else if small.logo.width < width
                    && small.keybinding.longest_description + 2 < width
                    && small.keybinding.total_height + 2 < height
                {
                    return small;
                }
                return tiny;
            }
            Large => large,
            Small => small,
            Tiny => tiny,
        }
    }

    fn new_large() -> Self {
        Self {
            logo: Logo::new(POMO_BUILD_LOGO_LARGE.into()),
            keybinding: KeyBinding::new(KEYBIND_STRINGS_LARGE, GAP_LARGE),
        }
    }

    fn new_small() -> Self {
        Self {
            logo: Logo::new(POMO_BUILD_LOGO_SMALL.into()),
            keybinding: KeyBinding::new(KEYBIND_STRINGS_SMALL, GAP_SMALL),
        }
    }

    fn new_tiny() -> Self {
        Self {
            logo: Logo::new(POMO_BUILD_LOGO_TINY.into()),
            keybinding: KeyBinding::new(KEYBIND_STRINGS_TINY, GAP_TINY),
        }
    }
}

// Big Fig
const CONFIGURATION_OPTION_NAMES: [&str; 4] = [
    "╔╦╗┬ ┬┌─┐┌┬┐┌─┐
 ║ ├─┤├┤ │││├┤ 
 ╩ ┴ ┴└─┘┴ ┴└─┘",
    "╦  ┌─┐┌┐┌┌─┐┬ ┬┌─┐┌─┐┌─┐
║  ├─┤││││ ┬│ │├─┤│ ┬├┤ 
╩═╝┴ ┴┘└┘└─┘└─┘┴ ┴└─┘└─┘",
    "╔═╗┌─┐┌┐┌┌┬┐  ╔═╗┬┌─┐┌─┐
╠╣ │ ││││ │   ╚═╗│┌─┘├┤ 
╚  └─┘┘└┘ ┴   ╚═╝┴└─┘└─┘",
    "╔╗ ┬─┐┌─┐┌─┐┬┌─  ┌─┐┌─┐┬─┐  ╔═╗┌─┐┌─┐┬ ┬┌─┐
╠╩╗├┬┘├┤ ├─┤├┴┐  ├─┘├┤ ├┬┘  ╠╣ │ ││  │ │└─┐
╚═╝┴└─└─┘┴ ┴┴ ┴  ┴  └─┘┴└─  ╚  └─┘└─┘└─┘└─┘",
];

const THEME_OPTIONS: [(Theme, &str); 3] = [
    (
        Theme::BloodRiver,
        "╔╗ ┬  ┌─┐┌─┐┌┬┐  ╦═╗┬┬  ┬┌─┐┬─┐
╠╩╗│  │ ││ │ ││  ╠╦╝│└┐┌┘├┤ ├┬┘
╚═╝┴─┘└─┘└─┘─┴┘  ╩╚═┴ └┘ └─┘┴└─",
    ),
    (
        Theme::GalaxyBlue,
        "╔═╗┌─┐┬  ┌─┐─┐ ┬┬ ┬  ╔╗ ┬  ┬ ┬┌─┐
║ ╦├─┤│  ├─┤┌┴┬┘└┬┘  ╠╩╗│  │ │├┤ 
╚═╝┴ ┴┴─┘┴ ┴┴ └─ ┴   ╚═╝┴─┘└─┘└─┘",
    ),
    (
        Theme::EvergreenForest,
        "╔═╗┬  ┬┌─┐┬─┐┌─┐┬─┐┌─┐┌─┐┌┐┌  ╔═╗┌─┐┬─┐┌─┐┌─┐┌┬┐
║╣ └┐┌┘├┤ ├┬┘│ ┬├┬┘├┤ ├┤ │││  ╠╣ │ │├┬┘├┤ └─┐ │ 
╚═╝ └┘ └─┘┴└─└─┘┴└─└─┘└─┘┘└┘  ╚  └─┘┴└─└─┘└─┘ ┴ ",
    ),
];
const LANGUAGE_OPTIONS: [(Language, &str); 2] = [
    (
        Language::English,
        "╔═╗┌┐┌┌─┐┬  ┬┌─┐┬ ┬
║╣ ││││ ┬│  │└─┐├─┤
╚═╝┘└┘└─┘┴─┘┴└─┘┴ ┴",
    ),
    (
        Language::Portuguese,
        "╔═╗┌─┐┬─┐┌┬┐┬ ┬┌─┐┬ ┬┌─┐┌─┐
╠═╝│ │├┬┘ │ │ ││ ┬│ │├┤ └─┐
╩  └─┘┴└─ ┴ └─┘└─┘└─┘└─┘└─┘",
    ),
];
const FONT_SIZE_OPTIONS: [(FontSize, &str); 4] = [
    (
        FontSize::Auto,
        "╔═╗┬ ┬┌┬┐┌─┐
╠═╣│ │ │ │ │
╩ ╩└─┘ ┴ └─┘",
    ),
    (
        FontSize::Large,
        "╦  ┌─┐┬─┐┌─┐┌─┐
║  ├─┤├┬┘│ ┬├┤ 
╩═╝┴ ┴┴└─└─┘└─┘",
    ),
    (
        FontSize::Small,
        "╔═╗┌┬┐┌─┐┬  ┬  
╚═╗│││├─┤│  │  
╚═╝┴ ┴┴ ┴┴─┘┴─┘",
    ),
    (
        FontSize::Tiny,
        "╔╦╗┬┌┐┌┬ ┬
 ║ ││││└┬┘
 ╩ ┴┘└┘ ┴ ",
    ),
];

// Small Slant
const CONFIG_TITLE: &str = "  _____          ____     
 / ___/__  ___  / _(_)__ _
/ /__/ _ \\/ _ \\/ _/ / _ `/
\\___/\\___/_//_/_//_/\\_, / 
                   /___/  ";

const CONFIGURATION_OPTION_NAMES_TINY: [&str; 4] =
    ["Theme", "Font Size", "Language", "Focus/Break"];
const THEME_OPTIONS_TINY: [(Theme, &str); 3] = [
    (Theme::BloodRiver, "Blood River"),
    (Theme::GalaxyBlue, "Galaxy Blue"),
    (Theme::EvergreenForest, "Evergreen Forest"),
];
const LANGUAGE_OPTIONS_TINY: [(Language, &str); 2] = [
    (Language::English, "English"),
    (Language::Portuguese, "Porutuguês"),
];
const FONT_SIZE_OPTIONS_TINY: [(FontSize, &str); 4] = [
    (FontSize::Auto, "Auto"),
    (FontSize::Large, "Large"),
    (FontSize::Small, "Small"),
    (FontSize::Tiny, "Tiny"),
];
