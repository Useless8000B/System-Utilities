use std::io;

use crate::ui::menu::Menu;

pub fn show_menu() {
    for option in Menu::all() {
        println!("{}", option.as_string());
    }
}

pub fn get_option() -> u8 {
    loop {
        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Error reading option");

        match option.trim().parse::<u8>() {
            Ok(number) => return number,

            Err(e) => {
                println!("Error parsing value: {e}");
                continue;
            }
        };
    }
}
