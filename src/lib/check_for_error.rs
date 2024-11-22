pub fn check_for_error(data: f64) -> bool {
    if data == 0.0 {
        true
    } else {
        false
    }
}
pub fn check_for_errors_p1(cords: [f64; 2]) -> bool {
    if cords[0] == 0.0 {
        true
    } else {
        false
    }
}
pub fn check_for_errors_p2(cords: [f64; 2]) -> bool {
    if cords[1] == 0.0 {
        true
    } else {
        false
    }
}