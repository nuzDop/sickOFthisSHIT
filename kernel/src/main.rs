#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::arch::x86_64::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

mod arch;
mod core;
mod fs;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    x86_64::instructions::interrupts::disable();
    arch::x86_64::vga_buffer::init();
    
    println!("LimitlessOS Kernel -- Booting...");

    // FIX: The field is `physical_memory_offset` and returns an Option<u64>
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.unwrap());
    
    let mut mapper = unsafe { crate::core::memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        crate::core::allocator::BootInfoFrameAllocator::new(&boot_info.memory_map)
    };

    crate::core::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialization failed");

    println!("[OK] Kernel Heap Initialized.");
    
    fs::init();

    println!("\nTesting VFS...");
    if let Some(file_content) = fs::vfs::ROOT_FS.lock().as_ref().unwrap().read("/welcome.txt") {
        // FIX: `from_utf8` is in `core::str`
        let text = core::str::from_utf8(&file_content).unwrap_or("Invalid UTF-8");
        println!("  Read from /welcome.txt: \"{}\"", text);
    } else {
        println!("  Failed to read /welcome.txt");
    }

    println!("\nInitialization complete. Halting CPU.");
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n*** KERNEL PANIC ***");
    println!("{}", info);
    hlt_loop();
}
