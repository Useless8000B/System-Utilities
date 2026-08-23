use crate::system::reader;
use crate::ui::menu::Menu;
use crate::ui::tui::get_option;
use crate::ui::tui::show_menu;
use crate::utils::format::line;

mod models;
mod system;
mod ui;
mod utils;

fn main() {
    loop {
        line("-=", 18);
        show_menu();
        line("-=", 18);
        let option = get_option();
        line("-=", 18);

        match Menu::from_input(option) {
            Some(Menu::Exit) => break,

            Some(Menu::Memory) => {
                let sources = [
                    ("RAM", reader::read_ram_info().ok()),
                    ("ZRAM", reader::read_zram_info().ok()),
                    ("VRAM", reader::read_vram_info().ok()),
                ];

                for (label, value) in sources {
                    match value {
                        Some(mem) => println!("{}: {:.2}/{:.2} GiB", label, mem.used, mem.total),
                        None => println!("Unknown error displaying info"),
                    }
                }
            }

            Some(Menu::Cpu) => {
                let cpu_info = reader::read_cpu_info()
                    .map_err(|e| format!("Error reading CPU sensor: {e}"))
                    .ok();

                match cpu_info {
                    Some(cpu) => println!("Temperature: {}\nUsage: {}%", cpu.temperature, cpu.usage),
                    None => println!("Unknown error displaying info"),
                }
            }

            Some(Menu::Gpu) => {
                let gpu_info = reader::read_gpu_info()
                    .map_err(|e| format!("Error reading GPU sensor: {e}"))
                    .ok();

                match gpu_info {
                    Some(gpu) => println!("Temperature: {}°C\nUsage: {}%", gpu.temperature, gpu.usage),
                    None => println!("Unknown error displaying info"),
                }
            }

            Some(Menu::Storage) => {
                let storage_info = reader::read_storage_info()
                    .map_err(|e| format!("Error reading STORAGE sensor: {e}"))
                    .ok();

                match storage_info {
                    Some(storage) => println!(
                        "Temperature: {}°C\nUsage: {:.2}/{:.2} GiB",
                        storage.temperature, storage.used, storage.total
                    ),
                    None => println!("Unknown error displaying info"),
                }
            }

            None => println!("Not an option"),
        };
    }
}
