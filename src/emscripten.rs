use std::os::raw::c_uint;

extern "C" {
    fn emscripten_get_now() -> f64;
    fn emscripten_sleep(ms: c_uint);
}

/// Milliseconds since start of program
pub fn now() -> f64 {
    unsafe { emscripten_get_now() }
}

pub fn sleep(ms: u32) {
    unsafe {
        emscripten_sleep(ms);
    }
}
