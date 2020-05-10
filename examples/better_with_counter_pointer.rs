use rust_closures_and_ffi::better::better_add_two_numbers;
use std::os::raw::{c_int, c_void};

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7];
    let mut total = 0;

    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];

            unsafe {
                better_add_two_numbers(
                    a,
                    b,
                    add_result_to_total,
                    &mut total as *mut c_int as *mut c_void,
                );
            }
        }
    }

    println!("The sum is {}", total);
}

unsafe extern "C" fn add_result_to_total(
    result: c_int,
    user_data: *mut c_void,
) {
    let total = &mut *(user_data as *mut c_int);
    *total += result;
}
