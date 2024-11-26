use crate::lib::input;
pub mod lib {
    pub mod input;
}
pub mod opperations {
    pub mod item_opperations;
    pub mod portal_opperations;
}
mod menus;

fn main() {
    menus::main_menu();
}
