// cargo build --target thumbv7em-none-eabihf
// ^^^ Use this to build on WSL. It sets it to have no target triple.

#![no_std] // This needs to run on bare metal.
#![no_main] // We are not using the normal entry point of 'main', as we are not using crt0 (C runtime 0) which would usually set up runtime features like a garbage collector in java. We cannot use crt0 and the rust runtime as we are on bare metal. Implementing 'start' is therefore useless as we do not have access to crt0. Hence we must overwrite crt0.

use core::panic::PanicInfo; // We need to define the function that the compiler should invoke when a panic occurs- we will define this ourselves.

/// This function will be called whenever we have a panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello world!";

mod vga_buffer;

#[no_mangle] // Ensures that the function name is not mangled.
/* We are not ever returning a value (! means it is diverging). This is because the entry point is not called by any function but it is invoked by the operating system or bootloader instead. It could invoke something like shutting down, but we'll loop ad infinitum for now. */
pub extern "C" fn _start() -> ! {

    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers! {} {} {}", 13, 04, 2021).unwrap();
    // The linker is looking for a functio nnamed '_start' by default.
    loop {}
}

// Module to handle printing

