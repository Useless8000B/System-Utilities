use crate::errors::reader_error::ReaderError;
use crate::system::reader;
use crate::ui::menu::Menu;
use crate::ui::tui::get_option;
use crate::ui::tui::show_menu;
use crate::utils::format::line;

mod models;
mod system;
mod ui;
mod utils;
mod errors;

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
                    ("RAM", reader::read_ram_info().map_err(|_| ReaderError::NotSupported)),
                    ("ZRAM", reader::read_zram_info().map_err(|_| ReaderError::NotSupported)),
                    ("VRAM", reader::read_vram_info().map_err(|_| ReaderError::NotSupported)),
                ];

                for (label, value) in sources {
                    match value {
                        Ok(mem) => println!("{}: {:.2}/{:.2} GiB", label, mem.used, mem.total),
                        Err(e) => println!("{e}")
                    }
                }
            }

            Some(Menu::Cpu) => {
                let cpu_info = reader::read_cpu_info().map_err(|_| ReaderError::NotSupported);

                match cpu_info {
                    Ok(cpu) => println!("Temperature: {}°C\nUsage: {}%", cpu.temperature, cpu.usage),
                    Err(e) => println!("{e}")
                }
            }

            Some(Menu::Gpu) => {
                let gpu_info = reader::read_gpu_info()
                    .map_err(|_| ReaderError::NotSupported);

                match gpu_info {
                    Ok(gpu) => println!("Temperature: {}°C\nUsage: {}%", gpu.temperature, gpu.usage),
                    Err(e) => println!("{e}"),
                }
            }

            Some(Menu::Storage) => {
                let storage_info = reader::read_storage_info()
                    .map_err(|_| ReaderError::NotSupported);
                match storage_info {
                    Ok(storage) => println!(
                        "Temperature: {}°C\nUsage: {:.2}/{:.2} GiB",
                        storage.temperature, storage.used, storage.total
                    ),
                    Err(e) => println!("{e}"),
                }
            }

            None => println!("Not an option"),
        };
    }
}
