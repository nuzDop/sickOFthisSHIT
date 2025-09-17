// This file makes the `fs` directory a module and defines its public interface.

pub mod ramfs;
pub mod vfs;

pub fn init() {
    ramfs::init();
    println!("[OK] VFS Initialized (ramfs).");
}
