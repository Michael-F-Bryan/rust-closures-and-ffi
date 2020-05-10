use rust_closures_and_ffi::simple::simple_add_two_numbers;
use std::os::raw::c_int;

static mut TOTAL: c_int = 0;

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7];

    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];

            unsafe {
                simple_add_two_numbers(a, b, add_result_to_total);
            }
        }
    }

    unsafe {
        println!("The sum is {}", TOTAL);

        let expected: c_int = (0..numbers.len())
            .flat_map(|i| (i..numbers.len()).map(move |j| (i, j)))
            .map(|(i, j)| numbers[i] + numbers[j])
            .sum();
        assert_eq!(expected, TOTAL);
    }
}

unsafe extern "C" fn add_result_to_total(result: c_int) { TOTAL += result; }
