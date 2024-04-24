pub struct Store;

#[cfg(not(target_os = "emscripten"))]
impl Store {
    pub fn volume(&self) -> Option<u32> {
        None
    }

    pub fn set_volume(&self, volume: u32) {
        // Do nothing
    }
}

#[cfg(target_os = "emscripten")]
extern "C" {
    pub fn emscripten_run_script(script: *const libc::c_char);
    pub fn emscripten_run_script_string(script: *const libc::c_char) -> *mut libc::c_char;
}

#[cfg(target_os = "emscripten")]
impl Store {
    fn run_script(script: &str) {
        use std::ffi::CString;

        let script = CString::new(script).unwrap();
        unsafe {
            emscripten_run_script(script.as_ptr());
        }
    }

    fn run_script_string(script: &str) -> String {
        use std::ffi::{CStr, CString};

        let script = CString::new(script).unwrap();
        unsafe {
            let ptr = emscripten_run_script_string(script.as_ptr());
            let c_str = CStr::from_ptr(ptr);
            String::from(c_str.to_str().unwrap())
        }
    }

    pub fn volume(&self) -> Option<u32> {
        // Use local storage to try and read volume
        let script = "localStorage.getItem('volume')";
        let response = Store::run_script_string(&script);
        response.parse::<u32>().ok()
    }

    pub fn set_volume(&self, volume: u32) {
        // Use local storage to set volume
        let script = format!("localStorage.setItem('volume', '{}')", volume);
        Store::run_script_string(&script);
    }
}