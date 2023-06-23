pub mod back;
pub mod front;

//children can accessing this function even though it is not public.
fn nothing() {
    println!("Nothing");
}
