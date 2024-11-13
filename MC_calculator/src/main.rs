use std::process::exit;
mod input;
fn main() {
    println!("Enter:\n 1 => portal linking\n 2 => item operations\n 3 => quit");
    let doing = input::menu_input();
    match doing.trim() {
        "1" => portal(),
        "2" => item_operations(),
        "3" => exit(0),
        _ => {
            println!("Invalid Input");
            main()
        }
    }
}
fn portal() {
    println!("Please enter to where the portal should be connected?:\n 1 => to nether \n 2 => to overworld \n 3 => menu ");
    let dim = input::menu_input();
    match dim.trim() {
        "1" => ptn(),
        "2" => pto(),
        "3" => main(),
        _ => {
            println!("Please enter a dimension");
            portal()
        }
    }
}

fn ptn() {
    let cords: [f32; 2] = input::portal_input();
    let xn = cords[0] * 8.0;
    let yn = cords[1] * 8.0;
    println!("the x coordinate is {}, the y coordinate is {}", xn, yn);
    portal()
}
fn pto() {
    let cords: [f32; 2] = input::portal_input();
    let xo: f32 = cords[0] / 8.0;
    let yo: f32 = cords[1] / 8.0;
    println!("the x coordinate is {}, the y coordinate is {}", xo, yo);
    portal()
}
fn item_operations() {
    println!("Enter:\n 1 => stacks to items\n 2 => items to stacks \n 3 => menu");
    let doing = input::menu_input();
    match doing.trim() {
        "1" => stack_to_items(),
        "2" => items_to_stack(),
        "3" => main(),
        _ => {
            println!("Invalid Input");
            item_operations()
        }
    }
}
fn stack_to_items() {
    println!("Please enter the stack count that you want to turn into items");
    let mut ic = input::storage_input();
    ic = ic * 64.0;
    println!("the item count is {}", ic);
    item_operations()
}
fn items_to_stack() {
    println!("Please enter the item count that you want to turn into stacks");
    let mut sc = input::storage_input();
    sc = sc / 64.0;
    println!("the stack count is {}", sc);
    item_operations()
}
