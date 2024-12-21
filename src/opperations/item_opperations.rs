use crate::{input, lib, menus};

pub fn division(divisor: f64, base:String, result:String) {
    println!("Please enter the amount of {base} that you want to convert to {result}");
    let dividend = lib::input::storage_input();
    let quotient = dividend/divisor;
    println!("The {result} size is {quotient}");
    menus::item_operations_input()
}
pub fn multiplication(factor1:f64, base:String, result:String) {
    println!("Please enter the amount of {base} that you want to convert to {result}");
    let factor2 = lib::input::storage_input();
    let product = factor1*factor2;
    println!("The {result} size is {product}");
    menus::item_operations_input()
}