use crate::{input, menus};
pub fn portal_to_nether() {
    let cords: [f64; 2] = input::portal_input();
    let xn = cords[0] * 8.0;
    let yn = cords[1] * 8.0;
    println!("the x coordinate is {}, the y coordinate is {}", xn, yn);
    menus::portal_input()
}
pub fn portal_to_overworld() {
    let cords: [f64; 2] = input::portal_input();
    let xo = cords[0] / 8.0;
    let yo = cords[1] / 8.0;
    println!("the x coordinate is {}, the y coordinate is {}", xo, yo);
    menus::portal_input()
}
