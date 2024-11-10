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
        "4" => exit(0),
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
    println!("Please enter the overwold x coordinate");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    println!("Please enter the overwold z coordinate");
    let mut z = String::new();
    io::stdin().read_line(&mut z).expect("Failed to read line");
    cords[1] = z.trim().parse().unwrap();
    cords[0] = x.trim().parse().unwrap();
    let xn= cords[0]*8;
    let yn= cords[1]*8;
    println!("the x coordinate is {}, the y coordinate is {}", xn, yn);
    main()
}
fn pto() {
    let mut cords: [f32; 2] = [0.0, 0.0];
    println!("Please enter the nether x coordinate");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    println!("Please enter the nether z coordinate");
    let mut z = String::new();
    io::stdin().read_line(&mut z).expect("Failed to read line");
    cords[1] = z.trim().parse().unwrap();
    cords[0] = x.trim().parse().unwrap();
    let xo:f32= cords[0]/8.0;
    let yo:f32= cords[1]/8.0;
    println!("the x coordinate is {}, the y coordinate is {}", xo, yo);
    main()
}
fn stack_to_items() {
    println!("Please enter the stack count that you want to turn into items");
    let mut sc = String::new();
    io::stdin().read_line(&mut sc).expect("Failed to read line");
    let mut ic:f32 = sc.trim().parse().unwrap();
    ic = ic*64.0;
    println!("the item count is {}", ic);
    main()
}
fn items_to_stack() {
    println!("Please enter the item count that you want to turn into stacks");
    let mut ic = String::new();
    io::stdin().read_line(&mut ic).expect("Failed to read line");
    let mut sc:f32 = ic.trim().parse().unwrap();
    sc = sc/64.0;
    println!("the stack count is {}", sc);
    main()
}
