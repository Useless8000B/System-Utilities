use std::path::Path;
use std::thread;
use std::time::Duration;

use sysinfo::Disks;
use sysinfo::System;

use crate::models::memory::Memory;
use crate::models::pu::Pu;
use crate::models::storage::Storage;
use crate::system::sensors::Sensor;
use crate::utils::extract_from_label::extract_from_label;

const MILLIDEGREES_C_TO_CELSIUS: f64 = 1000.0;

pub fn read_ram_info() -> Result<Memory, String> {
    let sensors = Sensor::memory_sensors();

    let raw_ram = sensors
        .iter()
        .find(|v| v.name == "RAM")
        .ok_or("RAM sensor not found!")?;

    let raw_total_ram = extract_from_label(raw_ram.path, "MemTotal")?;
    let raw_available_ram = extract_from_label(raw_ram.path, "MemAvailable")?;

    Ok(Memory {
        total: raw_total_ram as f64 / (1024.0 * 1024.0),
        used: (raw_total_ram as f64 - raw_available_ram as f64) / (1024.0 * 1024.0),
    })
}

pub fn read_zram_info() -> Result<Memory, String> {
    let sensors = Sensor::memory_sensors();

    let raw_zram = sensors
        .iter()
        .find(|v| v.name == "ZRAM")
        .ok_or("ZRAM sensor not found!")?;

    let raw_total_zram = extract_from_label(raw_zram.path, "SwapTotal")?;
    let raw_free_zram = extract_from_label(raw_zram.path, "SwapFree")?;

    Ok(Memory {
        total: raw_total_zram as f64 / (1024.0 * 1024.0),
        used: (raw_total_zram - raw_free_zram) as f64 / (1024.0 * 1024.0),
    })
}

pub fn read_vram_info() -> Result<Memory, String> {
    let sensors = Sensor::memory_sensors();
    let raw_total_vram = Sensor::find_sensor(sensors, "VRAM_TOTAL")?.read_sensor()?;
    let raw_used_vram = Sensor::find_sensor(sensors, "VRAM_USED")?.read_sensor()?;

    Ok(Memory {
        total: raw_total_vram as f64 / (1024.0 * 1024.0),
        used: raw_used_vram as f64 / (1024.0 * 1024.0),
    })
}

pub fn read_cpu_info() -> Result<Pu, String> {
    let sensors = Sensor::cpu_sensors();
    let mut sys = System::new_all();

    sys.refresh_cpu_usage();
    thread::sleep(Duration::from_millis(200));
    sys.refresh_cpu_usage();

    let cpu_usage: u8 = sys.global_cpu_usage() as u8;
    let raw_intel_average_temperature = sensors
        .iter()
        .find(|v| v.name == "INTEL_AVERAGE_TEMPERATURE")
        .ok_or("INTEL_AVERAGE_TEMPERATURE sensor not found!")?
        .read_sensor()?;

    Ok(Pu {
        temperature: raw_intel_average_temperature as f64 / MILLIDEGREES_C_TO_CELSIUS,
        usage: cpu_usage,
    })
}

pub fn read_gpu_info() -> Result<Pu, String> {
    let sensors = Sensor::gpu_sensors();

    let raw_amd_gpu_temperature = sensors
        .iter()
        .find(|v| v.name == "AMD_GPU_TEMPERATURE")
        .ok_or("AMD_GPU_TEMPERATURE sensor not found!")?
        .read_sensor()?;

    let raw_amd_gpu_usage = sensors
        .iter()
        .find(|v| v.name == "AMD_GPU_USAGE")
        .ok_or("AMD_GPU_USAGE sensor not found!")?
        .read_sensor()?;

    Ok(Pu {
        temperature: raw_amd_gpu_temperature as f64 / MILLIDEGREES_C_TO_CELSIUS,
        usage: raw_amd_gpu_usage as u8,
    })
}

pub fn read_storage_info() -> Result<Storage, String> {
    let sensors = Sensor::storage_sensors();
    let disks = Disks::new_with_refreshed_list();
    let target = Path::new("/");
    let disk = disks
        .iter()
        .find(|v| target.starts_with(v.mount_point()))
        .ok_or("NVME_AVAILABLE_SIZE sensor not found!")?;

    let raw_storage_temperature = Sensor::find_sensor(sensors, "NVME_TEMPERATURE")?.read_sensor()?;

    let raw_storage_size = Sensor::find_sensor(sensors, "NVME_SIZE")?.read_sensor()?;

    Ok(Storage {
        temperature: raw_storage_temperature as f64 / MILLIDEGREES_C_TO_CELSIUS,
        used: disk.available_space() as f64 / (1024.0 * 1024.0 * 1024.0),
        total: (raw_storage_size * 512) as f64 / (1024.0 * 1024.0 * 1024.0),
    })
}
