#![feature(abi_x86_interrupt)]
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(const_mut_refs)]

extern crate alloc;

pub mod allocator;
// pub mod kukisys;
// pub mod network;
// pub mod request;
// pub mod utils;
// pub mod functions;
// pub mod ethernet;
pub mod asm;
pub mod assembler;
pub mod basic_commands;
pub mod disk;
// pub mod drive_filesystem2;
pub mod command_dispatcher;
pub mod filesystem;
pub mod functions;
pub mod gdt;
pub mod interrupts;
pub mod mem_filesystem;
pub mod memory;
pub mod realsys;
pub mod serial;
pub mod task;
pub mod vga_buffer;

use core::{arch::asm, panic::PanicInfo};

use alloc::vec::Vec;
#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn sleep(ms: u64) {
    let cycles = ms * 1_000_000;
    let ticks = cycles / (1_000_000_000 / 50);
    let mut _s: Vec<u64> = Vec::new();
    for _x in 0..ticks {
        // println!("Waiting: {x}");
        // s.push(x);
        unsafe { asm!("nop") }
    }
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}....\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success)
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[fail]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
#[no_mangle]

pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}
