use crate::{input, opperations::item_opperations, opperations::portal_opperations};
use std::process::exit;
pub fn main_menu(){
    println!("Enter:\n 1 => portal linking\n 2 => item operations\n 3 => quit");
    let doing = input::menu_input();
    match doing.trim() {
        "1" => portal_input(),
        "2" => item_operations_input(),
        "3" => exit(0),
        _ => {
            println!("Invalid Input");
            main_menu()
        }
    }
}
pub fn portal_input() {
    println!("Please enter to where the portal should be connected?:\n 1 => to nether \n 2 => to overworld \n 3 => menu ");
    let dim = input::menu_input();
    match dim.trim() {
        "1" => portal_opperations::portal_to_nether(),
        "2" => portal_opperations::portal_to_overworld(),
        "3" => main_menu(),
        _ => {
            println!("Please enter a dimension");
            portal_input()
        }
    }
}
pub fn item_operations_input() {
    println!("Enter:\n 1 => stacks to items\n 2 => items to stacks\n 3 => items to shulkers\n 4 => Shulker to items\n 5 => items_to_dchests\n 6 => dchests_to_items\n 7 => stacks_to_dchests\n 8 => dchests_to_stacks\n 9 => dchests_to_fullshulker\n 10 => fullshulker_to_dchests\n 11 => menu");
    let doing = input::menu_input();
    match doing.trim() {
        "1" => item_opperations::multiplication(64.0, "stack".to_string(),"items".to_string() ),
        "2" => item_opperations::division(64.0,"items".to_string(), "stack".to_string() ),
        "3" => item_opperations::division(1728.0,"items".to_string(), "shulkerboxes".to_string() ),
        "4" => item_opperations::multiplication(1728.0,"Shulkerboxes".to_string(), "items".to_string() ),
        "5" => item_opperations::division(3456.0, "items".to_string(),"Double Chests".to_string() ),
        "6" => item_opperations::multiplication(3456.0,"Double Chests".to_string(), "items".to_string() ),
        "7" => item_opperations::division(54.0, "stack".to_string(), "Double Chests".to_string() ),
        "8" => item_opperations::multiplication(54.0,"Double Chests".to_string(), "items".to_string() ),
        "9" => item_opperations::multiplication(93312.0,"Shulkerboxes".to_string(), "Double Chests".to_string() ),
        "10" => item_opperations::division(93312.0, "Double Chests".to_string(),"Shulkerboxes".to_string() ),
        "11" => main_menu(),
        _ => {
            println!("Invalid Input");
            item_operations_input()
        }
    }
}
