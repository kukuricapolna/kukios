use crate::{
    basic_commands::white_space_divider,
    interrupts::{acpi_shutdown, input},
    mem_filesystem::FileSystem,
    println,
};
use alloc::{collections::BTreeMap, string::String, vec, vec::Vec};
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
        Mutex::new(m)
    };
    static ref FILESYSTEM: Mutex<FileSystem> = Mutex::new(FileSystem::new(1024, 128, 512));
    static ref FILES: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn help() {
    println!("=========================== KukiOS HELP center =================================");
    println!("Hello, This is KukiOS. We're happy to see you join our community. We are developing, a free linux-like system, entirely in Rust and x86_64 Assembly. We love Linux, Mac, but hate Windows. This is just a free, non-opensource project. --- Kukiweb.cz and KukiOS Admin, Kuki202");
    println!("================================================================================");
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
    let file_name = input();
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
