use crate::lib::check_for_error::check_for_error;
use crate::{input, menus};

pub fn stack_to_items() {
    println!("Please enter the stack count that you want to turn into items");
    let mut ic = input::storage_input();
    let is_error = check_for_error(ic);
    match is_error {
        true => {
            println!("Invalid Input, please Input a valid number.");
            stack_to_items()
        }
        false => {
            ic *= 64.0;
            println!("the item count is {}", ic);
            menus::item_operations_input()
        }
    }
}
pub fn items_to_stack() {
    println!("Please enter the item count that you want to turn into stacks");
    let mut sc = input::storage_input();
    let is_error = check_for_error(sc);
    match is_error {
        true => {
            println!("Invalid Input, please Input a valid number.");
            items_to_stack()
        }
        false => {
            sc /= 64.0;
            println!("the item count is {}", sc);
            menus::item_operations_input()
        }
    }
}
pub fn items_to_shulker() {
    println!("Please enter the item count that you want to turn into shulkerboxes");
    let mut sc = input::storage_input();
    let is_error = check_for_error(sc);
    match is_error {
        true => {
            println!("Invalid Input, please Input a valid number.");
            items_to_shulker()
        }
        false => {
            sc /= 1728.0;
            println!("the shulker count is {}", sc);
            menus::item_operations_input()
        }
    }
}
pub fn shulker_to_items() {
    println!("Please enter the shulker count that you want to turn into items");
    let mut sc = input::storage_input();
    let is_error = check_for_error(sc);
    match is_error {
        true => {
            println!("Invalid Input, please Input a valid number.");
            shulker_to_items()
        }
        false => {
            sc *= 1728.0;
            println!("the item count is {}", sc);
            menus::item_operations_input()
        }
    }
}
pub fn items_to_dchests() {
    println!("Please enter the shulker count that you want to turn into items");
    let mut sc = input::storage_input();
    let is_error = check_for_error(sc);
    match is_error {
        true => {
            println!("Invalid Input, please Input a valid number.");
            items_to_dchests()
        }
        false => {
            sc /= 3456.0;
            println!("the item count is {}", sc);
            menus::item_operations_input()
        }
    }
}
pub fn dchests_to_items() {
    println!("Please enter the shulker count that you want to turn into items");
    let mut sc = input::storage_input();
    let is_error = check_for_error(sc);
    match is_error {
        true => {
            println!("Invalid Input, please Input a valid number.");
            dchests_to_items()
        }
        false => {
            sc *= 3456.0;
            println!("the item count is {}", sc);
            menus::item_operations_input()
        }
    }
}
pub fn stacks_to_dchests() {
    println!("Please enter the shulker count that you want to turn into items");
    let mut sc = input::storage_input();
    let is_error = check_for_error(sc);
    match is_error {
        true => {
            println!("Invalid Input, please Input a valid number.");
            stacks_to_dchests()
        }
        false => {
            sc /= 54.0;
            println!("the item count is {}", sc);
            menus::item_operations_input()
        }
    }
}
pub fn dchests_to_stacks() {
    println!("Please enter the shulker count that you want to turn into items");
    let mut sc = input::storage_input();
    let is_error = check_for_error(sc);
    match is_error {
        true => {
            println!("Invalid Input, please Input a valid number.");
            dchests_to_stacks()
        }
        false => {
            sc *= 54.0;
            println!("the item count is {}", sc);
            menus::item_operations_input()
        }
    }
}
pub fn dchests_to_fullshulker() {
    println!("Please enter the shulker count that you want to turn into items");
    let mut sc = input::storage_input();
    let is_error = check_for_error(sc);
    match is_error {
        true => {
            println!("Invalid Input, please Input a valid number.");
            dchests_to_fullshulker()
        }
        false => {
            sc *= 93312.0;
            println!("the item count is {}", sc);
            menus::item_operations_input()
        }
    }
}
pub fn fullshulker_to_dchests() {
    println!("Please enter the shulker count that you want to turn into items");
    let mut sc = input::storage_input();
    let is_error = check_for_error(sc);
    match is_error {
        true => {
            println!("Invalid Input, please Input a valid number.");
            fullshulker_to_dchests()
        }
        false => {
            sc /= 93312.0;
            println!("the item count is {}", sc);
            menus::item_operations_input()
        }
    }
}
