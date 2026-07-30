pub enum Menu {
	ZRAM,
	RAM,
	VRAM,
	CPU
}

impl Menu {
	pub fn as_string(&self) -> &str {
		match self {
			Menu::ZRAM => "[1] Check ZRAM usage",
			Menu::RAM => "[2] Check RAM usage",
			Menu::VRAM => "[3] Check VRAM usage",
			Menu::CPU => "[4] Check CPU stats"
		}
	}

	pub fn from_input(choice: u8) -> Option<Self> {
		match choice {
			1 => Some(Menu::ZRAM),
			2 => Some(Menu::RAM),
			3 => Some(Menu::VRAM),
			4 => Some(Menu::CPU),
			_ => None			
		}
	}

	pub fn all() -> [Menu; 4] {
		[Menu::ZRAM, Menu::RAM, Menu::VRAM, Menu::CPU]
	}
}