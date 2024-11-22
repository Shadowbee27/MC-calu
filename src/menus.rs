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
    println!("Enter:\n 1 => stacks to items\n 2 => items to stacks\n 3 => items to shulkers\n 4 => Shulker to items\n 5 => items_to_dchests\n 6 => dchests_to_items\n 7 => stacks_to_dchests\n 8 => dchests_to_stacks\n 9 => dchests_to_fullshulker\n 10 => fullshulker_to_dchests\n 11 => menu");
    let doing = input::menu_input();
    match doing.trim() {
        "1" => item_opperations::stack_to_items(),
        "2" => item_opperations::items_to_stack(),
        "3" => item_opperations::items_to_shulker(),
        "4" => item_opperations::shulker_to_items(),
        "5" => item_opperations::items_to_dchests(),
        "6" => item_opperations::dchests_to_items(),
        "7" => item_opperations::stacks_to_dchests(),
        "8" => item_opperations::dchests_to_stacks(),
        "9" => item_opperations::dchests_to_fullshulker(),
        "10" => item_opperations::fullshulker_to_dchests(),
        "11" => main(),
        _ => {
            println!("Invalid Input");
            item_operations_input()
        }
    }
}