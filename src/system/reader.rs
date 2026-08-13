use std::path::Path;
use std::thread;
use std::time::Duration;

use sysinfo::System;
use sysinfo::Disks;

use crate::models::memory::Memory;
use crate::models::pu::Pu;
use crate::models::storage::Storage;
use crate::system::sensors::Sensor;
use crate::utils::extract_from_label::extract_from_label;

pub fn read_ram_info() -> Result<Memory, String> {
    let sensors = Sensor::memory_sensors();

    let raw_ram = sensors
        .iter()
        .find(|v| v.name == "RAM")
        .ok_or("RAM sensor not found!")?;

    let raw_total_ram = extract_from_label(&raw_ram.path, "MemTotal")?;
    let raw_available_ram = extract_from_label(&raw_ram.path, "MemAvailable")?;

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

    let raw_total_zram = extract_from_label(&raw_zram.path, "SwapTotal")?;
    let raw_free_zram = extract_from_label(&raw_zram.path, "SwapFree")?;

    Ok(Memory {
        total: raw_total_zram as f64 / (1024.0 * 1024.0),
        used: (raw_total_zram - raw_free_zram) as f64 / (1024.0 * 1024.0),
    })
}

pub fn read_vram_info() -> Result<Memory, String> {
    let sensors = Sensor::memory_sensors();

    let raw_total_vram = sensors
        .iter()
        .find(|v| v.name == "VRAM_TOTAL")
        .ok_or("VRAM_TOTAL sensor not found!")?
        .read_sensor()?;

    let raw_used_vram = sensors
        .iter()
        .find(|v| v.name == "VRAM_USED")
        .ok_or("VRAM_USED sensor not found!")?
        .read_sensor()?;

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
        temperature: raw_intel_average_temperature as f32 / 1000.0,
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
        temperature: raw_amd_gpu_temperature as f32 / 1000.0,
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

    let raw_storage_temperature = sensors
        .iter()
        .find(|v| v.name == "NVME_TEMPERATURE")
        .ok_or("NVME_TEMPERATURE sensor not found!")?
        .read_sensor()?;

    let raw_storage_size = sensors
        .iter()
        .find(|v| v.name == "NVME_SIZE")
        .ok_or("NVME_SIZE sensor not found!")?
        .read_sensor()?;

    Ok(Storage {
        temperature: raw_storage_temperature as f32 / 1000.0,
        used: disk.available_space() as f64 / (1024.0 * 1024.0 * 1024.0),
        total: (raw_storage_size * 512) as f64 / (1024.0 * 1024.0 * 1024.0),
    })
}
