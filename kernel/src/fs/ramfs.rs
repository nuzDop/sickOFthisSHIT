extern crate alloc;
use crate::fs::vfs::{self, FileSystem};
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

// A simple file is just a vector of bytes.
type RamFile = Vec<u8>;
// A simple directory is a map of names to files.
type RamDir = BTreeMap<String, RamFile>;

lazy_static! {
    // The global instance of our in-memory filesystem.
    static ref RAMFS: Mutex<RamDir> = Mutex::new(BTreeMap::new());
}

pub struct RamFS;

impl FileSystem for RamFS {
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        RAMFS.lock().get(path).cloned()
    }

    fn write(&self, path: &str, data: &[u8]) -> Result<(), &'static str> {
        RAMFS.lock().insert(path.to_string(), data.to_vec());
        Ok(())
    }

    fn list(&self, _path: &str) -> Option<Vec<String>> {
        // NOTE: For simplicity, this lists all files regardless of path.
        Some(RAMFS.lock().keys().cloned().collect())
    }
}

// Singleton instance of our RamFS driver.
static RAMFS_INSTANCE: RamFS = RamFS;

pub fn init() {
    // Create a welcome file.
    RAMFS_INSTANCE.write("/welcome.txt", b"Hello from the LimitlessOS RamFS!").unwrap();
    
    // Register our ramfs instance as the root filesystem.
    *vfs::ROOT_FS.lock() = Some(&RAMFS_INSTANCE);
}
