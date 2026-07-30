pub enum Menu {
    Zram,
    Ram,
    Vram,
    Cpu,
    VideoCard,
}

impl Menu {
    pub fn as_string(&self) -> &str {
        match self {
            Menu::Zram => "[1] Check ZRAM usage",
            Menu::Ram => "[2] Check RAM usage",
            Menu::Vram => "[3] Check VRAM usage",
            Menu::Cpu => "[4] Check CPU stats",
            Menu::VideoCard => "[5] Check VIDEO CARD stats",
        }
    }

    pub fn from_input(choice: u8) -> Option<Self> {
        match choice {
            1 => Some(Menu::Zram),
            2 => Some(Menu::Ram),
            3 => Some(Menu::Vram),
            4 => Some(Menu::Cpu),
            5 => Some(Menu::VideoCard),
            _ => None,
        }
    }

    pub fn all() -> [Menu; 5] {
        [
            Menu::Zram,
            Menu::Ram,
            Menu::Vram,
            Menu::Cpu,
            Menu::VideoCard,
        ]
    }
}
