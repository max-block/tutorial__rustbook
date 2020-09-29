mod front_of_house;

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        use std::io;
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

use crate::front_of_house::hosting;

fn main() {

}
