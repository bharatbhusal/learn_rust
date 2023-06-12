// use error_handling::errors::primitive_error_handling;
// use std::fs::File;

use error_handling::errors::*;
fn main() {
    /*  let data0 = */
    // primitive_error_handling();
    // let data1 = read_username_from_file();
    // println!("{:?}", data0);
    // println!("{:?}", data1);
    // let data2 = error_propagation();
    let data2 = error_propagation_shortcut();
    println!("{:?}", data2);
}
