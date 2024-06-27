#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use kukios::command_dispatcher::dispatch_command;
use kukios::interrupts::input;
// use kukios::realsys::FileSystem;
use core::arch::asm;
use core::panic::PanicInfo;
use kukios::mem_filesystem::FileSystem;

// use functions::{delay, shutdown};

mod asm;
mod functions;
mod serial;
mod vga_buffer;

// static HELLO: &[u8] = b"Hello World!";

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use kukios::allocator;
    use kukios::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;
    println!("Kukiweb + intelligence = KukiOS{}", "!");
    println!("Starting up KukiOS!");
    println!("Welcome, Default User!");

    kukios::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("SERIOUS EXCEPTION: HEAP init failed");
    let heap_value = Box::new(41);
    let mut buffer = [0u8; 1024];
    let mut fs = FileSystem::new(1024, 128, 512);
    println!("Creating default in-memory files.");
    let file_inode = fs.create_file(1024, "readme.txt").unwrap();
    fs.write_file(file_inode, "Welcome to Kukiweb KukiOS. This is a lincensed original Kukiweb product. We are planning to go open-source, but now it is not possible due to system-restrictions. We hope you enjoy our free system.".as_bytes());
    println!("Every file neccesary created");

    println!("heap_value is located at {:p}", heap_value);
    // let mut executor = Executor::new();
    // executor.spawn(Task::new(example_task()));
    // executor.spawn(Task::new(keyboard::print_keypresses()));
    // executor.spawn(Task::new(future))
    // executor.run();

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i)
    }
    println!("vec is located at {:p}", vec.as_slice());
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "Current reference count is at value of {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "Reference count is at value of {} now.",
        Rc::strong_count(&cloned_reference)
    );
    println!("Now in command mode. For help, type help.");
    loop {
        let x = input();
        dispatch_command(&x);
        if x == "jailbreak" {
            println!("Out of the command mode. Good luck soldier, you're on your own.");
            break;
        }
    }
    // let data = b"Somebody may say love is everything but thats wrong! KukiOS is everything!";
    // fs.write_file(file_inode, data);
    // let bytes_read = fs.read_file_by_name("test.txt", &mut buffer).unwrap();
    // let x = translate_to_string_utf8loosy(&buffer[..bytes_read]);
    // println!("{x}");
    // let mut fs = FileSystem::new();
    // fs.create_file("welcome.txt", b"Hello, World!").unwrap();
    // let x = fs.read_file("welcome.txt").unwrap();
    // let dat = vec_u8_to_string(x).unwrap();
    // println!("{dat}");

    // let mut buffer = vec![0u8; 1024];
    // let bytes_read = fs.read_file(file_inode, &mut buffer);

    // let translated_text = translate_to_string_utf8loosy(&buffer[..bytes_read]);
    // println!("Translated text: {translated_text}");
    // assert_eq!(&buffer[..bytes_read], data);
    // println!(
    // "File system operational. Written and read: {:?}",
    // &buffer[..bytes_read]
    // );
    // fs.re
    // let page = Page::containing_address(VirtAddr::new(0xdeadbeef000));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };
    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };
    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);
    //     }
    // }
    #[cfg(test)]
    test_main();
    // #[cfg(test)]
    // test_main();

    println!("Works!");

    kukios::hlt_loop();
}

#[no_mangle]
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
//     println!("Welcome user! Kukiweb + intelligence = KukiOS{}", "!");
//     kukios::init();
//     use x86_64::registers::control::Cr3;
//     let (level_4_page, _) = Cr3::read();
//     println!("Level 4 page table at {:?}", level_4_page.start_address());
//     // unsafe {
//     //     *(0xdeadbeef as *mut u8) = 42;
//     // }

//     // fn stack_overflow() {
//     //     stack_overflow();
//     // }
//     // stack_overflow();
//     // x86_64::instructions::interrupts::int3();
//     // let ptr = 0x206476 as *mut u8;
//     // unsafe {
//     //     let x = *ptr;
//     // }
//     // println!("read worked");
//     // unsafe {
//     //     *ptr = 42;
//     // }
//     // println!("write worked");

//     #[cfg(test)]
//     test_main();

//     #[cfg(test)]
//     println!("Status: [ok]");
//     print_something();
//     kukios::hlt_loop();
//     // loop {
//     //     use kukios::print;
//     //     print!("-")
//     // }
// }
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[fail]");
    println!("ERROR: KukiOS panicked: {}", info);
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

// pub struct MyDevice;

// impl Device for MyDevice {
//     fn capabilities(&self) -> DeviceCapabilities {
//         let mut caps = DeviceCapabilities::default();
//         caps.max_transmission_unit = 1500;
//         caps
//     }
//     fn transmit(&mut self) -> Option<TxToken> {
//         None
//     }
//     fn receive(&mut self) -> Option<RxToken> {
//         None
//     }
// }
fn _vec_u8_to_string(vec: Vec<u8>) -> Result<String, &'static str> {
    match core::str::from_utf8(&vec) {
        Ok(valid_str) => Ok(valid_str.to_string()),
        Err(_) => Err("Invalid UTF-8 sequence."),
    }
}
