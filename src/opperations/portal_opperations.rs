use crate::lib::check_for_error::check_for_error;
use crate::{input, menus};

pub fn portal_to_nether() {
    let cords: [f64; 2] = input::portal_input();
    let is_error = check_for_error(cords[0]);
    if is_error == true {
        println!("Invalid Input, please Input a valid number.");
        portal_to_nether()
    } else {
        let is_error2 = check_for_error(cords[1]);
        if is_error2 == true {
            println!("Invalid Input, please Input a valid number.");
            portal_to_nether()
        } else {
            let xn = cords[0] * 8.0;
            let yn = cords[1] * 8.0;
            println!("the x coordinate is {}, the y coordinate is {}", xn, yn);
            menus::portal_input();
        }
    }
}
pub fn portal_to_overworld() {
    let cords: [f64; 2] = input::portal_input();
    let is_error = check_for_error(cords[0]);
    if is_error == true {
        println!("Invalid Input, please Input a valid number.");
        portal_to_nether()
    } else {
        let is_error2 = check_for_error(cords[1]);
        if is_error2 == true {
            println!("Invalid Input, please Input a valid number.");
            portal_to_overworld()
        } else {
            let xo = cords[0] / 8.0;
            let yo = cords[1] / 8.0;
            println!("the x coordinate is {}, the y coordinate is {}", xo, yo);
            menus::portal_input();
        }
    }
}
