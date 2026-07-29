pub enum Menu {
	ZRAM,
	RAM,
	VRAM,
}

impl Menu {
	pub fn as_string(&self) -> &str {
		match self {
			Menu::ZRAM => "[1] Check ZRAM usage",
			Menu::RAM => "[2] Check RAM usage",
			Menu::VRAM => "[3] Check VRAM usage"
		}
	}

	pub fn from_input(choice: u8) -> Option<Self> {
		match choice {
			1 => Some(Menu::ZRAM),
			2 => Some(Menu::RAM),
			3 => Some(Menu::VRAM),
			_ => None			
		}
	}

	pub fn all() -> [Menu; 3] {
		[Menu::ZRAM, Menu::RAM, Menu::VRAM]
	}
}