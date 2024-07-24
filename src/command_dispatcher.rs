use core::arch::global_asm;

use crate::{
    assembler::*,
    basic_commands::white_space_divider,
    interrupts::{acpi_shutdown, input},
    mem_filesystem::FileSystem,
    println,
};
use alloc::{collections::BTreeMap, format, string::String, vec, vec::Vec};
use lazy_static::lazy_static;

use spin::Mutex;

lazy_static! {
    static ref COMMANDS: Mutex<BTreeMap<&'static str, fn()>> = {
        let mut m = BTreeMap::new();
        m.insert("shutdown", shutdown as fn());
        m.insert("name", welcome as fn());
        m.insert("welcome", welcome as fn());
        m.insert("touch", create_file as fn());
        m.insert("help", help as fn());
        m.insert("micro", open_file as fn());
        m.insert("clear", clear as fn());
        m.insert("kas", kas as fn());
        m.insert("ras", run_assembly as fn());
        Mutex::new(m)
    };
    static ref FILESYSTEM: Mutex<FileSystem> = Mutex::new(FileSystem::new(1024, 128, 512));
    static ref FILES: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn help() {
    white_space_divider(5);
    println!("======================== KukiOS HELP center ========================");
    println!("Hello, This is KukiOS. \n We're happy to see you join our community. \n We are developing, a free linux-like system, entirely in Rust and x86_64 Assembly. \n We love Linux, Mac, but hate Windows. \n This is just a free, non-opensource project. \n --- Kukiweb.cz and KukiOS Admin, Kuki202");
    println!("====================================================================");
}

fn clear() {
    let _whitespaces = white_space_divider(40);
}

fn welcome() {
    let name = input();
    println!("Hello, {name}!")
}

fn shutdown() {
    println!("Shutting down.");
    unsafe { acpi_shutdown() }
}

fn create_file() {
    let mut fs = FILESYSTEM.lock();
    let mut _files = FILES.lock();
    white_space_divider(1);
    println!("Enter file name: ");
    let file_name = input();
    white_space_divider(1);
    println!("Enter file's text: ");
    let content = input();
    let x = fs.create_file(1024, file_name.as_str()).unwrap();
    fs.write_file(x, content.trim().as_bytes());
    white_space_divider(1);
    println!("The file's name is {file_name}.");
    // files.push(x);
    // inodes.push(x);
}

fn run_assembly() {
    let mut fs = FILESYSTEM.lock();
    let mut buffer = vec![0; 1024];
    let file_name = input();
    if let Some(size) = fs.read_file_by_name(&file_name, &mut buffer) {
        buffer.truncate(size);
        // Some(buffer)
    } else {
        // None
        println!("ERROR: ASM Buffer not found.")
    }
    let entry_point = buffer.as_ptr() as usize;
    unsafe {
        let func: extern "C" fn() -> ! = core::mem::transmute(entry_point);
        func();
    }
}

fn open_file() {
    let fs = FILESYSTEM.lock();
    let mut buffer = vec![0u8; 1024];
    let mut _files = FILES.lock();
    white_space_divider(1);
    println!("Enter file name to open: ");
    let file_name = input();
    println!("Opening {file_name}....");
    if let Some(bytes_read) = fs.read_file_by_name(file_name.as_str(), &mut buffer) {
        let data = core::str::from_utf8(&buffer[..bytes_read]).unwrap();
        println!("====================== FILE {file_name} (read-only) ======================");
        println!("{}", data.replace(" ", "").trim().trim_end()); //.replace(" ", "").trim()
        println!("==============================================================");
    } else {
        println!("File not found: {file_name}");
    }
    //
    // let _content = fs
    //     .read_file_by_name(file_name.as_str(), &mut buffer)
    //     .unwrap();
    // let data = core::str::from_utf8(&buffer).unwrap();
    // white_space_divider(40);
}

pub fn dispatch_command(cmd: &str) {
    let commands = COMMANDS.lock();
    if let Some(&command_fn) = commands.get(cmd) {
        command_fn();
    } else {
        white_space_divider(1);
        println!("KukiOS command center: Unknown command: >>> {cmd} <<<");
        white_space_divider(1);
    }
}

pub fn uname() {
    println!("KukiOS: 0.1.0");
}

fn echo() {
    println!("");
    println!("What to echo?");
    let echo = input();
    println!("");
    println!("Where to echo? (file-which, here)");
    let whereto = input();
}

fn kas() {
    let mut msg = b"";
    let mut g: &str = "";

    // let mut fs = FILESYSTEM.lock();
    // let mut buffer = vec![0u8; 1024];
    // white_space_divider(2);
    // println!("Name of assembly file (we will create it for you) ? ");
    // let assembly_file = input();
    // let x = fs.create_file(1024, &assembly_file).unwrap();
    // println!("Write your assembly here: ");
    // let newasm = input();
    // fs.write_file(x, newasm.as_bytes());
    // println!("Assembling file {assembly_file}");
    // if let Some(bytes_read) = fs.read_file_by_name(&assembly_file, &mut buffer) {
    //     let data = core::str::from_utf8(&buffer[..bytes_read]).unwrap();
    //     let asm = Assembler::assemble(data);
    //     let y = fs
    //         .create_file(1024, &format!("{}.bin", assembly_file.replace(".asm", "")))
    //         .unwrap();
    //     let _ = fs.write_file(y, &asm);
    //     println!("Successfully created and assembled a file named {assembly_file}");
    // } else {
    //     println!("Assembly file {assembly_file} not found!");
    // }
}
