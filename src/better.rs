use std::os::raw::{c_int, c_void};

pub type AddCallback = unsafe extern "C" fn(c_int, *mut c_void);

extern "C" {
    pub fn better_add_two_numbers(
        a: c_int,
        b: c_int,
        cb: AddCallback,
        user_data: *mut c_void,
    );
}
