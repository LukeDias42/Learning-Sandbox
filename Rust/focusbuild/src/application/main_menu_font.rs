pub struct MainMenuFont {
    pub logo: Logo,
    pub key_binding: KeyBinding,
}

pub struct Logo {
    pub width: u16,
    pub height: u16,
    pub offset: u16,
    pub text: String,
}

pub struct KeyBinding {
    pub width: u16,
    pub height: u16,
    pub longest_description: u16,
    pub key_desc_map: Vec<(String, String)>,
}

impl MainMenuFont {
    pub fn new(width: u16, height: u16) -> MainMenuFont {
        if width > 88 && height > 36 {
            return MainMenuFont::new_large();
        }
        return MainMenuFont::new_small();
    }
    fn new_large() -> MainMenuFont {
        MainMenuFont {
            logo: Logo {
                width: 78,
                height: 6,
                offset: 0,
                text: POMO_BUILD_LOGO_LARGE.to_string(),
            },
            key_binding: KeyBinding {
                width: 5,
                height: 4,
                longest_description: 34,
                key_desc_map: KEYBIND_STRINGS_LARGE
                    .map(|(key, desc)| (key.to_string(), desc.to_string()))
                    .to_vec(),
            },
        }
    }
    fn new_small() -> MainMenuFont {
        MainMenuFont {
            logo: Logo {
                width: 38,
                height: 4,
                offset: 2,
                text: POMO_BUILD_LOGO_SMALL.to_string(),
            },
            key_binding: KeyBinding {
                width: 3,
                height: 2,
                longest_description: 21,
                key_desc_map: KEYBIND_STRINGS_SMALL
                    .map(|(key, desc)| (key.to_string(), desc.to_string()))
                    .to_vec(),
            },
        }
    }
}

// ANSII SHADOW
const POMO_BUILD_LOGO_LARGE: &str =
    "███████╗ ██████╗  ██████╗██╗   ██╗███████╗██████╗ ██╗   ██╗██╗██╗     ██████╗ 
██╔════╝██╔═══██╗██╔════╝██║   ██║██╔════╝██╔══██╗██║   ██║██║██║     ██╔══██╗
█████╗  ██║   ██║██║     ██║   ██║███████╗██████╔╝██║   ██║██║██║     ██║  ██║
██╔══╝  ██║   ██║██║     ██║   ██║╚════██║██╔══██╗██║   ██║██║██║     ██║  ██║
██║     ╚██████╔╝╚██████╗╚██████╔╝███████║██████╔╝╚██████╔╝██║███████╗██████╔╝
╚═╝      ╚═════╝  ╚═════╝ ╚═════╝ ╚══════╝╚═════╝  ╚═════╝ ╚═╝╚══════╝╚═════╝ 
                                                                              ";

// Adapted
const KEYBIND_STRINGS_LARGE: [(&str, &str); 5] = [
    (
        " ▗▄▄▖
▐▌   
 ▝▀▚▖
▗▄▄▞▘",
        " ▗▄▄▖▗▄▄▄▖▗▄▖ ▗▄▄▖▗▄▄▄▖
▐▌     █ ▐▌ ▐▌▐▌ ▐▌ █  
 ▝▀▚▖  █ ▐▛▀▜▌▐▛▀▚▖ █  
▗▄▄▞▘  █ ▐▌ ▐▌▐▌ ▐▌ █  ",
    ),
    (
        "▗▖ ▗▖
▐▌ ▐▌
▐▛▀▜▌
▐▌ ▐▌",
        "▗▖ ▗▖▗▄▄▄▖ ▗▄▄▖▗▄▄▄▖▗▄▖ ▗▄▄▖▗▖  ▗▖
▐▌ ▐▌  █  ▐▌     █ ▐▌ ▐▌▐▌ ▐▌▝▚▞▘ 
▐▛▀▜▌  █   ▝▀▚▖  █ ▐▌ ▐▌▐▛▀▚▖ ▐▌  
▐▌ ▐▌▗▄█▄▖▗▄▄▞▘  █ ▝▚▄▞▘▐▌ ▐▌ ▐▌  ",
    ),
    (
        "▗▄▄▄ 
▐▌  █
▐▌  █
▐▙▄▄▀",
        "▗▄▄▄  ▗▄▖▗▄▄▄▖▗▄▖ 
▐▌  █▐▌ ▐▌ █ ▐▌ ▐▌
▐▌  █▐▛▀▜▌ █ ▐▛▀▜▌
▐▙▄▄▀▐▌ ▐▌ █ ▐▌ ▐▌",
    ),
    (
        "▗▄▄▄▖
  █  
  █  
  █  ",
        "▗▄▄▄▖▗▄▖ ▗▖ ▗▖▗▖  ▗▖
  █ ▐▌ ▐▌▐▌ ▐▌▐▛▚▖▐▌
  █ ▐▌ ▐▌▐▌ ▐▌▐▌ ▝▜▌
  █ ▝▚▄▞▘▐▙█▟▌▐▌  ▐▌",
    ),
    (
        "▗▄▄▄▖ 
▐▌ ▐▌ 
▐▌ ▐▌ 
▐▙▄▟▙▖",
        "▗▄▄▄▖ ▗▖ ▗▖▗▄▄▄▖▗▄▄▄▖
▐▌ ▐▌ ▐▌ ▐▌  █    █  
▐▌ ▐▌ ▐▌ ▐▌  █    █  
▐▙▄▟▙▖▝▚▄▞▘▗▄█▄▖  █  ",
    ),
];

// Cursive
const POMO_BUILD_LOGO_SMALL: &str = "   _____              __          _    
    /  '             /  )        //   /
 ,-/-,__ _. . . _   /--<  . . o // __/ 
(_/  (_)(__(_/_/_)_/___/_(_/_<_</_(_/_ 
                                       ";
// Big Fig
const KEYBIND_STRINGS_SMALL: [(&str, &str); 5] = [
    (
        " __
(_ 
__)",
        " __            
(_ _|_ _  ___|_
__) |_(_| |  |_",
    ),
    (
        "   
|_|
| |",
        "                     
|_| o  _ _|_ _  __ \\/
| | | _>  |_(_) |  / ",
    ),
    (
        " _ 
| \\
|_/",
        " _          
| \\ _ _|_ _ 
|_/(_| |_(_|",
    ),
    (
        "___
 | 
 | ",
        "___         
 |  _    __ 
 | (_)\\^/| |",
    ),
    (
        " _ 
/ \\
\\_X",
        " _          
/ \\    o _|_
\\_X|_| |  |_",
    ),
];
