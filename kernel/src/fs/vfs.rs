extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

// Add `Send + Sync` to the trait definition to make it thread-safe
pub trait FileSystem: Send + Sync {
    fn read(&self, path: &str) -> Option<Vec<u8>>;
    fn write(&self, path: &str, data: &[u8]) -> Result<(), &'static str>;
    fn list(&self, path: &str) -> Option<Vec<String>>;
}

pub static ROOT_FS: Mutex<Option<&'static dyn FileSystem>> = Mutex::new(None);
