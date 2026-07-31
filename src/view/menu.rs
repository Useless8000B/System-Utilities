pub enum Menu {
    Exit,
    Zram,
    Ram,
    Vram,
    Cpu,
    Gpu,
    Storage,
}

impl Menu {
    pub fn as_string(&self) -> &str {
        match self {
            Menu::Exit => "[0] Exit",
            Menu::Zram => "[1] Check ZRAM usage",
            Menu::Ram => "[2] Check RAM usage",
            Menu::Vram => "[3] Check VRAM usage",
            Menu::Cpu => "[4] Check CPU stats",
            Menu::Gpu => "[5] Check GPU stats",
            Menu::Storage => "[6] Check STORAGE stats",
        }
    }

    pub fn from_input(choice: u8) -> Option<Self> {
        match choice {
            0 => Some(Menu::Exit),
            1 => Some(Menu::Zram),
            2 => Some(Menu::Ram),
            3 => Some(Menu::Vram),
            4 => Some(Menu::Cpu),
            5 => Some(Menu::Gpu),
            6 => Some(Menu::Storage),
            _ => None,
        }
    }

    pub fn all() -> [Menu; 7] {
        [
            Menu::Exit,
            Menu::Zram,
            Menu::Ram,
            Menu::Vram,
            Menu::Cpu,
            Menu::Gpu,
            Menu::Storage
        ]
    }
}
