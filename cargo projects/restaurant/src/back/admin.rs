pub fn shut_shop() {
    println!("Shop is closed by admin");
    //child modules can access private elements of parent module.
    super::super::nothing(); //relative addressing
    crate::nothing(); //absolute addressing - crate = lib.rs
}
pub fn open_shop() {
    println!("Shop is opened by admin");
}

//no elements outside this file(admin.rs) can access this function.
fn nothing_of_admin() {
    println!("Nothing of admin");
}
