use std::io;

pub fn portal_input() -> [f64; 2] {
    loop {
        println!("Please enter the x coordinate");
        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("Failed to read line");
        println!("Please enter the z coordinate");
        let mut z = String::new();
        io::stdin().read_line(&mut z).expect("Failed to read line");
        let z = z.trim().parse::<f64>();
        let x = x.trim().parse::<f64>();
        match x {
            Ok(ok) => {
                let x = ok;
                match z {
                    Ok(ok) => {
                        let z = ok;
                        let cords =[x,z];
                        return cords;
                    }
                    Err(e) => println!("Error: ({})", e),
                }
            }
            Err(e) => println!("Error: ({})", e),
        }
    }
}
pub fn storage_input() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().parse::<f64>();
        match input {
            Ok(ok) => {
                return ok;
            }
            Err(e) => println!("Error: ({}). Please try again", e),
        }
    }
}
pub fn menu_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}
