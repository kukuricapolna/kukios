#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use kukios::{
    memory::{self, BootInfoFrameAllocator},
    task::{executor::Executor, keyboard, simple_executor::SimpleExecutor, Task},
};
use vga_buffer::print_something;
use x86_64::structures::paging::Page;

mod serial;
mod vga_buffer;

// static HELLO: &[u8] = b"Hello World!";

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // use kukios::memory::active_level_4_table;
    use kukios::allocator;
    use kukios::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Kukiweb + intelligence = KukiOS{}", "!");
    kukios::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("SERIOUS EXCEPTION: HEAP init failed");
    let heap_value = Box::new(41);
    println!("heap_value is located at {:p}", heap_value);
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
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
    let page = Page::containing_address(VirtAddr::new(0xdeadbeef000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };
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
    println!("{}", info);
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
    serial_println!("Error: {}\n", info);
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
async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("The async number is {}", number);
}
