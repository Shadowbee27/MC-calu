use std::io;
use std::process::exit;
fn main() {
    let mut doing = String::new();
    io::stdin()
        .read_line(&mut doing)
        .expect("Failed to read line");
    match doing.trim() {
        "1" => portal(),
        "2" => stack_to_items(),
        "3" => items_to_stack(),
        "4" => exit(1),
        _ => panic!(),
    }
}
fn portal() {
    println!("Please enter to where the portal should be connected");
    let mut dim = String::new();
    io::stdin()
        .read_line(&mut dim)
        .expect("Failed to read line");
    match dim.trim() {
        "n" => ptn(),
        "o" => pto(),
        _ => {
            println!("Please enter a dimension");
            portal()
        }
    }
}

fn ptn() {
    let mut cords: [i32; 2] = [0, 0];
    println!("Please enter the x coordinate");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    println!("Please enter the z coordinate");
    let mut z = String::new();
    io::stdin().read_line(&mut z).expect("Failed to read line");
    cords[1] = z.trim().parse().unwrap();
    cords[0] = x.trim().parse().unwrap();
    let count =0;
    for i in cords {
        i * 8;
        if count == 0 {
            println!("Please enter a ");
        }
    }
}
fn pto() {
    todo!()
}
fn stack_to_items() {
    todo!()
}
fn items_to_stack() {
    todo!()
}
