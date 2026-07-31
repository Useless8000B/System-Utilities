use crate::system::reader;
use crate::utils::format::line;
use crate::view::menu::Menu;
use crate::view::ui::get_option;
use crate::view::ui::show_menu;

mod models;
mod system;
mod utils;
mod view;

fn main() {
    loop {
        line("-=", 18);
        show_menu();
        line("-=", 18);
        let option = get_option();
        line("-=", 18);

        match Menu::from_input(option) {
            Some(Menu::Exit) => break,

            Some(Menu::Zram) => {
                let zram_info = reader::read_zram_info()
                    .map_err(|e| format!("Error reading ZRAM sensor: {e}"))
                    .ok();

                if let Some(model) = &zram_info {
                    println!("{:.2}/{:.2}GiB", model.used, model.total)
                }
            }

            Some(Menu::Ram) => {
                let ram_info = reader::read_ram_info()
                    .map_err(|e| format!("Error reading RAM sensor: {e}"))
                    .ok();

                if let Some(model) = &ram_info {
                    println!("Usage: {:.2}/{:.2} GiB", model.used, model.total)
                }
            }

            Some(Menu::Vram) => {
                let vram_info = reader::read_vram_info()
                    .map_err(|e| format!("Error reading VRAM sensor: {e}"))
                    .ok();

                if let Some(model) = &vram_info {
                    println!("Usage: {:.2}/{:.2} GiB", model.used, model.total)
                }
            }

            Some(Menu::Cpu) => {
                let cpu_info = reader::read_cpu_info()
                    .map_err(|e| format!("Error reading CPU sensor: {e}"))
                    .ok();

                if let Some(model) = &cpu_info {
                    println!(
                        "Temperature: {:.2}°C\nUsage: {}%",
                        model.temperature, model.usage
                    )
                }
            }

            Some(Menu::Gpu) => {
                let gpu_info = reader::read_gpu_info()
                    .map_err(|e| format!("Error reading GPU sensor: {e}"))
                    .ok();

                if let Some(model) = &gpu_info {
                    println!("{:.2}°C\nUsage: {}%", model.temperature, model.usage)
                }
            }

            Some(Menu::Storage) => {
                let storage_info = reader::read_storage_info()
                    .map_err(|e| format!("Error reading STORAGE sensor: {e}"))
                    .ok();

                if let Some(model) = &storage_info {
                    println!(
                        "Temperature: {:.2}°C\nUsage: {:.2}/{:.2}",
                        model.temperature, model.used, model.total
                    )
                }
            }

            Option::None => println!("Not an option"),
        };
    }
}
