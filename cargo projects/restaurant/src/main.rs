mod back;
mod front;

use back::*;
use front::*;

fn main() {
    println!("Hello, world!");
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    kitchen::read_kot();
    kitchen::cook();
    kitchen::pass_to_waiter();
    serving::serve_order();
    serving::take_payment();
    admin::shut_shop();
}
