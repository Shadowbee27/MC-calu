use crate::{input, menus};

pub fn stack_to_items() {
    println!("Please enter the stack count that you want to turn into items");
    let mut ic = input::storage_input();
    ic *= 64.0;
    println!("the item count is {}", ic);
    menus::item_operations_input()
}
pub fn items_to_stack() {
    println!("Please enter the item count that you want to turn into stacks");
    let mut sc = input::storage_input();
    sc /= 64.0;
    println!("the stack count is {}", sc);
    menus::item_operations_input()
}
