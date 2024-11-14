use crate::{input, main, opperations::item_opperations, opperations::portal_opperations};

pub fn portal_input() {
    println!("Please enter to where the portal should be connected?:\n 1 => to nether \n 2 => to overworld \n 3 => menu ");
    let dim = input::menu_input();
    match dim.trim() {
        "1" => portal_opperations::portal_to_nether(),
        "2" => portal_opperations::portal_to_overworld(),
        "3" => main(),
        _ => {
            println!("Please enter a dimension");
            portal_input()
        }
    }
}
pub fn item_operations_input() {
    println!("Enter:\n 1 => stacks to items\n 2 => items to stacks \n 3 => menu");
    let doing = input::menu_input();
    match doing.trim() {
        "1" => item_opperations::stack_to_items(),
        "2" => item_opperations::items_to_stack(),
        "3" => main(),
        _ => {
            println!("Invalid Input");
            item_operations_input()
        }
    }
}
