#![no_std] // We don't want to link the Rust standard library
#![no_main] // We don't want the Rust runtime, we'll write our own entry point (the bootloader will call _start)

use core::panic::PanicInfo;


// Entry point, called by the bootloader
#[no_mangle] // Don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    loop {}
}


// Panic handler, called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}