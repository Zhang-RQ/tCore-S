#![no_std]
#![no_main]
#![feature(naked_functions, asm_sym, asm_const)]

mod config;
mod trap;
mod console;

use config::BOOT_STACK_SIZE;

/// Entry for kernel.
#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    // Initialize kernel stack in .bss section.
    #[link_section = ".bss.stack"]
    static mut STACK: [u8; BOOT_STACK_SIZE] = [0u8; BOOT_STACK_SIZE];

    core::arch::asm!(
        // Set stack pointer to the kernel stack.
        "la sp, {stack} + {stack_size}",
        // Jump to the main function.
        "j  {main}",
        stack_size = const BOOT_STACK_SIZE,
        stack      =   sym STACK,
        main       =   sym rust_main,
        options(noreturn),
    )
}

extern "C" fn rust_main() -> ! {
    panic!("Panic")
}
