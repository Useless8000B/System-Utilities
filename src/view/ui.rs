use std::io;

use crate::view::menu::Menu;

pub fn show_menu() {
    for option in Menu::all() {
        println!("{}", option.as_string());
    }
}

pub fn get_option() -> u8 {
    let mut option = String::new();

	io::stdin().read_line(&mut option).expect("Error reading option");

	let option = option.trim().parse().expect("Not a number!");

	option
}
