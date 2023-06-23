// use restaurant::{back::*, front::*};

// fn main() {
//     println!("Hello, world!");
//     admin::open_shop();
//     hosting::add_to_waitlist();
//     hosting::seat_at_table();
//     serving::take_order();
//     kitchen::read_kot();
//     kitchen::cook();
//     kitchen::pass_to_waiter();
//     serving::serve_order();
//     serving::take_payment();
//     admin::shut_shop();
// }

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Add to waitlist.");
        }
    }
}

mod something {
    pub use super::front_of_house::hosting as this_is;
    // pub fn so() {
    //     this_is::add_to_waitlist();
    // }
}

use crate::something::*;
fn main() {
    this_is::add_to_waitlist();
}
