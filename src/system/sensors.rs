use std::fs;

pub struct Sensor {
	pub name: String,
	pub path: String
}

impl Sensor {
	pub fn new(name: &str, path: &str) -> Self {
		Sensor {
			name: name.to_string(),
			path: path.to_string()
		}
	}

	pub fn memory_sensors() -> Vec<Sensor> {
		vec![
			Sensor::new("RAM", "/proc/meminfo"),
			Sensor::new("ZRAM", "/proc/meminfo"),
			Sensor::new("VRAM_TOTAL", "/sys/class/drm/card1/device/mem_info_vram_total"),
			Sensor::new("VRAM_USED", "/sys/class/drm/card1/device/mem_info_vram_used")
		]
	}

	pub fn read_sensor(&self) -> Result<u64, String> {
		let raw_content = match fs::read_to_string(&self.path) {
			Ok(c) => c,
			Err(e) => return Err(format!("Error reading {}: {}", self.name, e))
		};

		match raw_content.trim().parse::<u64>() {
			Ok(c) => Ok(c),
			Err(_) => Err(format!("Invalid value from {}", self.name))
		}
	}
}