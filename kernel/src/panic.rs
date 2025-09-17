use crate::hlt_loop;
use core::panic::PanicInfo;

// The `use crate::println;` line is no longer needed
// because the macro is now in scope everywhere.

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n*** KERNEL PANIC ***");
    println!("Error: {}", info);
    println!("Halting system.");
    hlt_loop();
}
