use std::{io::Seek, path::PathBuf};

#[cfg(not(target_os = "emscripten"))]
pub struct Store {
    file: std::fs::File,
}

#[cfg(target_os = "emscripten")]
pub struct Store;

#[cfg(not(target_os = "emscripten"))]
impl Store {
    pub fn new () -> Self {
        let path = Store::get_path();
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .unwrap();
        Store { file }
    }

    fn get_path() -> PathBuf {
        use std::env;
        let mut path = env::current_exe().unwrap();
        path.pop();
        path.push("volume.txt");
        path
    }

    pub fn volume(&mut self) -> Option<u32> {
        use std::io::Read;

        let mut buffer = String::new();
        self.file.read_to_string(&mut buffer).unwrap();
        buffer.trim().parse::<u32>().ok()
    }

    pub fn set_volume(&mut self, volume: u32) {
        use std::io::Write;

        self.file.set_len(0).unwrap();
        self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
        write!(self.file, "{}", volume).unwrap();
    }
}

#[cfg(target_os = "emscripten")]
extern "C" {
    pub fn emscripten_run_script(script: *const libc::c_char);
    pub fn emscripten_run_script_string(script: *const libc::c_char) -> *mut libc::c_char;
}

#[cfg(target_os = "emscripten")]
impl Store {
    pub fn new () -> Self {
        Store
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
}