use std::os::raw::c_int;

pub type AddCallback = unsafe extern "C" fn(c_int);

extern "C" {
    pub fn simple_add_two_numbers(a: c_int, b: c_int, cb: AddCallback);
}

pub unsafe extern "C" fn on_two_numbers_added(result: c_int) {
    println!("Got {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_simple_function_callback() {
        unsafe {
            simple_add_two_numbers(1, 2, on_two_numbers_added);
        }
    }
}
