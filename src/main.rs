#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::arch::asm;
use core::panic::PanicInfo;
use kukios::command_dispatcher::dispatch_command;
use kukios::interrupts::input;

mod asm;
mod functions;
mod serial;
mod startup_prompt;
mod vga_buffer;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use kukios::allocator;
    use kukios::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    // Initialize basic systems first
    kukios::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("SERIOUS EXCEPTION: HEAP init failed");

    // Ask user for GUI preference
    let use_gui = startup_prompt::ask_for_gui();

    if use_gui {
        // Start GUI mode
        startup_prompt::show_gui_loading();
        start_gui_mode();
    } else {
        // Continue with CLI mode
        startup_prompt::show_cli_continuation();
        start_cli_mode();
    }
}

// extern "C" {
//     fn my_adder(a: i64, b: i64) -> i64;
// }

#[no_mangle]
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use kukios::{interrupts::acpi_shutdown, sleep};

    println!("[fail]");
    println!(
        "ERROR: KukiOS panicked: {}. Preparing the system to shutdown.",
        info
    );
    sleep(1000000000);
    unsafe { acpi_shutdown() }
    kukios::hlt_loop();
}

/// Start GUI mode
fn start_gui_mode() -> ! {
    use kukios::gui::init_gui;

    println!("Initializing graphics subsystem...");

    // Initialize GUI context
    let mut gui = init_gui();

    println!("Starting desktop environment...");

    // Run GUI main loop
    gui.run();

    // Should never reach here, but just in case
    kukios::hlt_loop();
}

/// Start CLI mode (original behavior)
fn start_cli_mode() -> ! {
    // Ensure cursor is enabled for CLI
    use crate::vga_buffer::WRITER;
    WRITER.lock().enable_cursor();

    println!("Kukiweb + intelligence = KukiOS!");
    println!("Welcome, Default User!");

    println!("Written to disk test data (all ones - sector 1, 512 l).");

    let heap_value = Box::new(41);
    println!("Heap Value well-known ({})", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i)
    }
    println!("vec is located at {:p}", vec.as_slice());
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    core::mem::drop(reference_counted);

    println!("Now in command mode. For help, type help.");
    println!("Type 'gui' to switch to graphical mode.");
    loop {
        let x = input(kukios::interrupts::Helper::Empty);
        dispatch_command(&x);
        if x == "jailbreak" {
            println!("Out of the command mode. Good luck soldier, you're on your own.");
            break;
        }
    }

    kukios::hlt_loop();
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

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

#[test_case]
fn trivial_assertion() {
    serial_print!("Trivial assertion....");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[fail]\n");
    serial_println!("Error: KukiOS panicked: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
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
async fn _async_number() -> u32 {
    42
}

async fn _example_task() {
    let number = _async_number().await;
    println!("The async number is {}", number);
}

pub fn shutdown() {
    unsafe {
        asm!(
            "mov ax, 0x1000",
            "mov ax, ss",
            "mov sp, 0xf000",
            "mov ax, 0x5307",
            "mov bx, 0x0001",
            "mov cx, 0x0003",
            "int 0x15",
        );
    }
}

fn _delay(seconds: u64) {
    const CYCLES_PER_SECOND: u64 = 2_900_000_000;
    let target = seconds * CYCLES_PER_SECOND;
    for _ in 0..target {
        core::hint::spin_loop();
    }
}

fn _vec_u8_to_string(vec: Vec<u8>) -> Result<String, &'static str> {
    match core::str::from_utf8(&vec) {
        Ok(valid_str) => Ok(valid_str.to_string()),
        Err(_) => Err("Invalid UTF-8 sequence."),
    }
}
