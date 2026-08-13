pub enum Menu {
    Exit,
    Memory,
    Cpu,
    Gpu,
    Storage,
}

impl Menu {
    pub fn as_string(&self) -> &str {
        match self {
            Menu::Exit => "[0] Exit",
            Menu::Memory => "[1] Check MEMORY stats",
            Menu::Cpu => "[2] Check CPU stats",
            Menu::Gpu => "[3] Check GPU stats",
            Menu::Storage => "[4] Check STORAGE stats",
        }
    }

    pub fn from_input(choice: u8) -> Option<Self> {
        match choice {
            0 => Some(Menu::Exit),
            1 => Some(Menu::Memory),
            2 => Some(Menu::Cpu),
            3 => Some(Menu::Gpu),
            4 => Some(Menu::Storage),
            _ => None,
        }
    }

    pub fn all() -> [Menu; 5] {
        [
            Menu::Exit,
            Menu::Memory,
            Menu::Cpu,
            Menu::Gpu,
            Menu::Storage
        ]
    }
}
