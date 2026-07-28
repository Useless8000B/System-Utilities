use std::fs;

pub fn extract_from_label(path: &str, label: &str) -> Result<u64, String> {
    let content =
        fs::read_to_string(path)
            .map_err(|e| format!("Error reading file {path}: {e}"))?;

    let memory_value = content
        .lines()
        .find_map(|line| {
            if line.starts_with(label) {
                line.split_whitespace()
                    .nth(1)
                    .and_then(|v| v.parse::<u64>().ok())
            } else {
                None
            }
        })
        .ok_or_else(|| format!("Error reading {label}"))?;

    Ok(memory_value)
}