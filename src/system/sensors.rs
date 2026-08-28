use std::{fs, path::Path};
use crate::errors::reader_error::ReaderError;

pub struct Sensor {
    pub name: &'static str,
    pub path: &'static str,
}

impl Sensor {
    pub const fn new(name: &'static str, path: &'static str) -> Self {
        Sensor { name, path }
    }

    pub fn memory_sensors() -> &'static [Sensor] {
        const MEMORY_SENSORS: &[Sensor] = &[
            Sensor::new("RAM", "/proc/meminfo"),
            Sensor::new("ZRAM", "/proc/meminfo"),
            Sensor::new(
                "VRAM_TOTAL",
                "/sys/class/drm/card1/device/mem_info_vram_total",
            ),
            Sensor::new(
                "VRAM_USED",
                "/sys/class/drm/card1/device/mem_info_vram_used",
            ),
        ];

        MEMORY_SENSORS
    }

    pub fn cpu_sensors() -> &'static [Sensor] {
        const CPU_SENSORS: &[Sensor] = &[Sensor::new(
            "INTEL_AVERAGE_TEMPERATURE",
            "/sys/class/hwmon/hwmon2/temp1_input",
        )];

        CPU_SENSORS
    }

    pub fn gpu_sensors() -> &'static [Sensor] {
        const GPU_SENSORS: &[Sensor] = &[
            Sensor::new("AMD_GPU_TEMPERATURE", "/sys/class/hwmon/hwmon1/temp1_input"),
            Sensor::new(
                "AMD_GPU_USAGE",
                "/sys/class/drm/card1/device/gpu_busy_percent",
            ),
        ];

        GPU_SENSORS
    }

    pub fn storage_sensors() -> &'static [Sensor] {
        const STORAGE_SENSORS: &[Sensor] = &[
            Sensor::new("NVME_TEMPERATURE", "/sys/class/hwmon/hwmon0/temp1_input"),
            Sensor::new("NVME_SIZE", "/sys/block/nvme0n1/size"),
        ];

        STORAGE_SENSORS
    }

    fn exists(&self) -> bool {
        Path::new(self.path).exists()
    }

    pub fn read_sensor(&self) -> Result<u64, ReaderError> {
        let raw_content = match fs::read_to_string(self.path) {
            Ok(c) => c,
            Err(e) => return Err(ReaderError::ReadingError(format!("Error reading {}: {e}", self.path))),
        };

        match raw_content.trim().parse::<u64>() {
            Ok(c) => Ok(c),
            Err(e) => Err(ReaderError::ParseError(format!("Couldn't parse: {raw_content}: {e}"))),
        }
    }

    pub fn find_sensor<'a>(sensors: &'a [Sensor], name: &str) -> Result<&'a Sensor, ReaderError> {
        sensors
            .iter()
            .find(|v| v.name == name && v.exists())
            .ok_or_else(|| ReaderError::SensorNotFound(format!("{name} sensor not found")))
    }
}
