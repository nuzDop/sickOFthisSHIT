// This is a STUB for the VFS traits.
// We will expand this in the future with proper Vnode/Inode abstractions.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

// For now, a simple trait for filesystem operations.
pub trait FileSystem {
    fn read(&self, path: &str) -> Option<Vec<u8>>;
    fn write(&self, path: &str, data: &[u8]) -> Result<(), &'static str>;
    fn list(&self, path: &str) -> Option<Vec<String>>;
}

// A global handle to the root filesystem.
// This is a temporary solution for our simple, single-filesystem setup.
pub static ROOT_FS: Mutex<Option<&'static dyn FileSystem>> = Mutex::new(None);
