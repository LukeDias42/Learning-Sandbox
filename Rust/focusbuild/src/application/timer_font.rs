pub struct TimerFont {
    pub title: Title,
    pub digits: Digits,
}

pub struct Title {
    pub offset: i16,
    pub width: u16,
    pub height: u16,
    pub text: String,
}

pub struct Digits {
    pub digit_width_map: Vec<(String, u16)>,
    pub total_width: u16,
    pub height: u16,
    pub stopwatchs_width: u16,
    pub balance_width: u16,
    pub gap: u16,
}

impl TimerFont {
    pub fn new(width: u16, height: u16) -> TimerFont {
        if width > 88 && height > 36 {
            return TimerFont::new_large();
        }
        return TimerFont::new_small();
    }
    fn new_large() -> TimerFont {
        TimerFont {
            title: Title {
                offset: -1,
                width: 45,
                height: 6,
                text: FOCUS_TITLE.to_string(),
            },
            digits: Digits {
                gap: 1,
                height: 5,
                total_width: 71,
                stopwatchs_width: 26,
                balance_width: 29,
                digit_width_map: DIGIT_WIDTH_MAP_LARGE
                    .map(|(digit, width)| (digit.to_string(), width))
                    .to_vec(),
            },
        }
    }
    fn new_small() -> TimerFont {
        TimerFont {
            title: Title {
                offset: -1,
                width: 45,
                height: 6,
                text: FOCUS_TITLE.to_string(),
            },
            digits: Digits {
                gap: 0,
                height: 3,
                total_width: 33,
                stopwatchs_width: 10,
                balance_width: 12,
                digit_width_map: DIGIT_WIDTH_MAP_SMALL
                    .map(|(digit, width)| (digit.to_string(), width))
                    .to_vec(),
            },
        }
    }
}

const FOCUS_TITLE: &str = "███████╗ ██████╗  ██████╗██╗   ██╗███████╗██╗
██╔════╝██╔═══██╗██╔════╝██║   ██║██╔════╝██║
█████╗  ██║   ██║██║     ██║   ██║███████╗██║
██╔══╝  ██║   ██║██║     ██║   ██║╚════██║╚═╝
██║     ╚██████╔╝╚██████╗╚██████╔╝███████║██╗
╚═╝      ╚═════╝  ╚═════╝ ╚═════╝ ╚══════╝╚═╝";
const DIGIT_WIDTH_MAP_LARGE: [(&str, u16); 12] = [
    (
        " ████ 
██  ██
██  ██
██  ██
 ████ ",
        6,
    ),
    (
        "████  
  ██  
  ██  
  ██  
██████",
        6,
    ),
    (
        " ████ 
██  ██
   ██ 
 ██   
██████",
        6,
    ),
    (
        " ████ 
██  ██
   ██
██  ██
 ████ ",
        6,
    ),
    (
        "██  ██
██  ██
██████
    ██
    ██",
        6,
    ),
    (
        "██████
██    
█████ 
    ██
█████ ",
        6,
    ),
    (
        " ████ 
██    
█████ 
██  ██
 ████ ",
        6,
    ),
    (
        "██████
   ██ 
  ██  
 ██   
██    ",
        6,
    ),
    (
        " ████ 
██  ██
 ████ 
██  ██
 ████ ",
        6,
    ),
    (
        " ████ 
██  ██
 █████
    ██
 ████ ",
        6,
    ),
    (
        "  
██
  
██
    ",
        2,
    ),
    (
        "   
   
███
   
      ",
        3,
    ),
];

const DIGIT_WIDTH_MAP_SMALL: [(&str, u16); 12] = [
    (
        "┏┓
┃┫
┗┛",
        2,
    ),
    (
        "┓ 
┃ 
┻ ",
        2,
    ),
    (
        "┏┓
┏┛
┗━",
        2,
    ),
    (
        "┏┓
 ┫
┗┛",
        2,
    ),
    (
        "┏┓
┃┃
┗╋",
        2,
    ),
    (
        "┏━
┗┓
┗┛",
        2,
    ),
    (
        "┏┓
┣┓
┗┛",
        2,
    ),
    (
        "━┓
 ┃
 ╹",
        2,
    ),
    (
        "┏┓
┣┫
┗┛",
        2,
    ),
    (
        "┏┓
┗┫
┗┛",
        2,
    ),
    (
        " 
•
•",
        1,
    ),
    (
        "  
━━
  ",
        2,
    ),
];
