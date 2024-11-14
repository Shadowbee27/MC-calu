use crate::lib::input;
use std::process::exit;
pub mod lib{
    pub mod input;
}
pub mod opperations {
    pub mod item_opperations;
    pub mod portal_opperations;
}
mod menus;

fn main() {
    println!("Enter:\n 1 => portal linking\n 2 => item operations\n 3 => quit");
    let doing = input::menu_input();
    match doing.trim() {
        "1" => menus::portal_input(),
        "2" => menus::item_operations_input(),
        "3" => exit(0),
        _ => {
            println!("Invalid Input");
            main()
        }
    }
}
