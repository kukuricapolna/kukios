use crate::{
    functions::{help, last_two_keys, list_dir},
    gdt, hlt_loop, print, println, sleep,
};
use alloc::{
    string::{String, ToString},
    vec::{self, Vec},
};
use core::arch::{asm, x86_64::_rdtsc};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::{self, Mutex};
use x86_64::{
    instructions::port::Port,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
};
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }
    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    static ref KEYS_PRESSED: Mutex<Vec<char>> = Mutex::new(Vec::new());
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt
    };
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // print!(".");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("SERIOUS EXCEPTION! : BREAKPOINT\n{:#?}", stack_frame)
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("SERIOUS EXCEPTION: DOUBLE FAULT:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // print!("k");
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;
    let mut keys_pressed: Vec<char> = Vec::new();
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore
            ));
    }
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    use alloc::vec;
                    let mut keyspressed = KEYS_PRESSED.lock();
                    keyspressed.push(character);
                    let ltwo = last_two_keys(&mut keyspressed);
                    let shutdown_combination: Vec<char> =
                        vec!["S".parse().unwrap(), "D".parse().unwrap()];
                    let help_combination: Vec<char> =
                        vec!["L".parse().unwrap(), "P".parse().unwrap()];
                    let list_combination: Vec<char> =
                        vec!["L".parse().unwrap(), "I".parse().unwrap()];
                    if ltwo.contains(&shutdown_combination[0])
                        && ltwo.contains(&shutdown_combination[1])
                    {
                        unsafe { acpi_shutdown() }
                    }

                    if ltwo.contains(&help_combination[0]) && ltwo.contains(&help_combination[1]) {
                        help();
                    }
                    if ltwo.contains(&list_combination[0]) && ltwo.contains(&list_combination[1]) {
                        list_dir();
                    }

                    if character.to_string().as_str().trim_end() == "LShift" {
                        print!("");
                    } else if character.to_string().as_str().trim_end() == "l" {
                        print!("Latest keys pressed ({} keys pressed): ", keyspressed.len());
                        for key in &*keyspressed {
                            print!("{}", key);
                        }
                    } else {
                        let _ = character.clone();
                        print!("{}", character.clone())
                    }
                }
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    // let key = match scancode {
    //     0x02 => Some("1"),
    //     0x03 => Some("2"),
    //     0x04 => Some("3"),
    //     0x05 => Some("4"),
    //     0x06 => Some("5"),
    //     0x07 => Some("6"),
    //     0x08 => Some("7"),
    //     0x09 => Some("8"),
    //     0x0a => Some("9"),
    //     0x0b => Some("0"),
    //     _ => None,
    // };
    // if let Some(key) = key {
    //     print!("{}", key)
    // }
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8())
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("SERIOUS EXCEPTION: PAGE FAULT");
    println!("Accessed Address : {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}

unsafe fn acpi_shutdown() {
    println!("Shutting down in few seconds. Get ready!");
    use x86_64::instructions::port::Port;
    println!("Performing shutdown using writing to ACPI control block. [ok]");
    sleep(10000000);
    const PM1A_CNT_BLK: u16 = 0xB004;
    const SLP_TYPA: u16 = 0x2000;
    const SLP_EN: u16 = 1 << 13;

    let shutdown_cmd: u16 = SLP_TYPA | SLP_EN;
    let mut port = Port::new(PM1A_CNT_BLK);

    port.write(shutdown_cmd);
    let mut port_604 = Port::new(0x604);
    port_604.write(0x2000u16);
}

// fn ask(text: &str) {
//     let mut input = [0; 128];

//     print!("Enter {text}: ");
//     let _ =
// }

/*if character.to_string().as_str() == "s" {
    // let _ = car.clone();
    print!("Shutting down!");

    unsafe {
        print!("Shutting down....");
        // asm!(
        //     "mov ax, 0x1000",
        //     "mov ax, ss",
        //     "mov sp, 0xf000",
        //     "mov ax, 0x5307",
        //     "mov bx, 0x0001",
        //     "mov cx, 0x0003",
        //     "int 0x15",
        // );
        acpi_shutdown();
    }
}  */
