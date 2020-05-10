use rust_closures_and_ffi::simple::{
    on_two_numbers_added, simple_add_two_numbers,
};

fn main() {
    let a = 1;
    let b = 2;

    println!("Adding {} and {}", a, b);

    unsafe {
        simple_add_two_numbers(1, 2, on_two_numbers_added);
    }
}
