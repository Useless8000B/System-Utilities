use crate::models::memory_model::MemoryModel;
use crate::system::sensors::Sensor;
use crate::utils::extract_from_label::extract_from_label;

pub fn read_ram_info() -> Result<MemoryModel, String> {
    let sensors = Sensor::memory_sensors();

    let raw_ram = sensors
        .iter()
        .find(|v| v.name == "RAM")
        .ok_or("RAM sensor not found!")?;

    let raw_total_ram = extract_from_label(&raw_ram.path, "MemTotal")?;
    let raw_available_ram = extract_from_label(&raw_ram.path, "MemAvailable")?;

    Ok(MemoryModel {
        total: raw_total_ram as f64 / (1024.0 * 1024.0),
        used: (raw_total_ram as f64 - raw_available_ram as f64) / (1024.0 * 1024.0),
    })
}

pub fn read_zram_info() -> Result<MemoryModel, String> {
    let sensors = Sensor::memory_sensors();

    let raw_zram = sensors
        .iter()
        .find(|v| v.name == "ZRAM")
        .ok_or("ZRAM sensor not found")?;

    let raw_total_zram = extract_from_label(&raw_zram.path, "SwapTotal")?;
    let raw_free_zram = extract_from_label(&raw_zram.path, "SwapFree")?;

    Ok(MemoryModel {
        total: raw_total_zram as f64 / (1024.0 * 1024.0),
        used: (raw_total_zram - raw_free_zram) as f64 / (1024.0 * 1024.0),
    })
}
