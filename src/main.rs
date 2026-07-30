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

        match Menu::from_input(option) {
            Some(Menu::ZRAM) => {
                let zram_info = reader::read_zram_info()
                    .map_err(|e| format!("Error reading ZRAM sensor: {e}"))
                    .ok();

                if let Some(model) = &zram_info {
                    println!("{:.2}/{:.2}", model.used, model.total)
                }
            }

            Some(Menu::RAM) => {
                let ram_info = reader::read_ram_info()
                    .map_err(|e| format!("Error reading RAM sensor: {e}"))
                    .ok();

                if let Some(model) = &ram_info {
                    println!("{:.2}/{:.2}GB", model.used, model.total)
                }
            }

            Some(Menu::VRAM) => {
                let vram_info = reader::read_vram_info()
                    .map_err(|e| format!("Error reading VRAM sensor: {e}"))
                    .ok();

                if let Some(model) = &vram_info {
                    println!("{:.2}/{:.2}GB", model.used, model.total)
                }
            }

            Some(Menu::CPU) => {
                let cpu_info = reader::read_cpu_info()
                    .map_err(|e| format!("Error reading CPU sensor: {e}"))
                    .ok();

                if let Some(model) = &cpu_info {
                    println!("{:.2}°C", model.average_temperature)
                }
            }

            Option::None => println!("Not an option"),
        };
    }
}
