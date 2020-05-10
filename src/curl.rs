use std::os::raw::{c_char, c_int, c_void};

pub type OnDataReceived =
    unsafe extern "C" fn(*const c_char, usize, *mut c_void);

extern "C" {
    pub fn send_get_request(
        url: *const c_char,
        cb: OnDataReceived,
        user_data: *mut c_void,
    ) -> c_int;
}

pub unsafe extern "C" fn trampoline<C>(
    buffer: *const c_char,
    buffer_len: usize,
    data: *mut c_void,
) where
    C: FnMut(&[u8]),
{
    let cb = &mut *(data as *mut C);
    let buffer = std::slice::from_raw_parts(buffer as *mut u8, buffer_len);

    cb(buffer);
}

pub fn get_trampoline<C>(_closure: &C) -> OnDataReceived
where
    C: FnMut(&[u8]),
{
    trampoline::<C>
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn send_a_get_request_to_google() {
        let url = CString::new("https://google.com/").unwrap();
        let mut buffer = Vec::new();

        unsafe {
            let mut closure = |data: &[u8]| buffer.extend_from_slice(data);
            let cb = get_trampoline(&closure);

            let ret = send_get_request(
                url.as_ptr(),
                cb,
                &mut closure as *mut _ as *mut c_void,
            );
            assert_eq!(ret, 0);
        }

        let response = dbg!(String::from_utf8_lossy(&buffer));
        assert!(response.contains("The document has moved"));
        assert!(response.contains("https://www.google.com/"));
    }
}
