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

pub fn get_trampoline<F>(_closure: &F) -> AddCallback
where
    F: FnMut(c_int),
{
    trampoline::<F>
}

unsafe extern "C" fn trampoline<F>(result: c_int, user_data: *mut c_void)
where
    F: FnMut(c_int),
{
    let user_data = &mut *(user_data as *mut F);
    user_data(result);
}

/// Add two numbers, passing the result to the provided closure for further
/// processing.
pub fn add_two_numbers<F>(a: i32, b: i32, on_result_calculated: F)
where
    F: FnMut(i32),
{
    unsafe {
        let mut closure = on_result_calculated;
        let cb = get_trampoline(&closure);

        better_add_two_numbers(a, b, cb, &mut closure as *mut _ as *mut c_void);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_the_trampoline_function() {
        let mut got = 0;

        {
            let mut closure = |result: c_int| got = result;
            let trampoline = get_trampoline(&closure);

            unsafe {
                better_add_two_numbers(
                    1,
                    2,
                    trampoline,
                    &mut closure as *mut _ as *mut c_void,
                );
            }
        }

        assert_eq!(got, 1 + 2);
    }
}
