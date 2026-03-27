#[cfg(not(target_os = "emscripten"))]
pub struct Store {
    file: std::fs::File,
}

#[cfg(target_os = "emscripten")]
pub struct Store;

#[cfg(not(target_os = "emscripten"))]
impl Store {
    pub fn new() -> Self {
        Self::with_path(Store::default_path())
    }

    fn with_path(path: std::path::PathBuf) -> Self {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .unwrap();
        Store { file }
    }

    fn default_path() -> std::path::PathBuf {
        let mut path = std::env::current_exe().unwrap();
        path.pop();
        path.push("volume.txt");
        path
    }

    pub fn volume(&mut self) -> Option<u32> {
        use std::io::{Read, Seek};

        self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
        let mut buffer = String::new();
        self.file.read_to_string(&mut buffer).unwrap();
        buffer.trim().parse::<u32>().ok()
    }

    pub fn set_volume(&mut self, volume: u32) {
        use std::io::{Seek, Write};

        self.file.set_len(0).unwrap();
        self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
        write!(self.file, "{}", volume).unwrap();
    }
}

#[cfg(target_os = "emscripten")]
extern "C" {
    pub fn emscripten_run_script_string(script: *const libc::c_char) -> *mut libc::c_char;
}

#[cfg(target_os = "emscripten")]
impl Store {
    pub fn new() -> Self {
        Store
    }

    pub fn volume(&self) -> Option<u32> {
        let script = "localStorage.getItem('volume')";
        let response = Store::run_script_string(script);
        response.parse::<u32>().ok()
    }

    pub fn set_volume(&self, volume: u32) {
        let script = format!("localStorage.setItem('volume', '{}')", volume);
        Store::run_script_string(&script);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_store() -> Store {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("spiritus-test-{}.txt", std::process::id()));
        Store::with_path(path)
    }

    #[test]
    fn volume_returns_none_on_empty_file() {
        let mut store = temp_store();
        assert_eq!(store.volume(), None);
    }

    #[test]
    fn set_and_get_volume() {
        let mut store = temp_store();
        store.set_volume(42);
        assert_eq!(store.volume(), Some(42));
    }

    #[test]
    fn set_volume_overwrites_previous() {
        let mut store = temp_store();
        store.set_volume(100);
        store.set_volume(50);
        assert_eq!(store.volume(), Some(50));
    }

    #[test]
    fn volume_boundary_values() {
        let mut store = temp_store();

        store.set_volume(0);
        assert_eq!(store.volume(), Some(0));

        store.set_volume(128);
        assert_eq!(store.volume(), Some(128));
    }
}
