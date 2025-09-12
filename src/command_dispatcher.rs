use crate::{
    basic_commands::white_space_divider,
    beta::test_kukilang_code,
    builtin::{chg_fg_bg, print_colorful, print_current_colors_debug, println_colorful},
    interrupts::{acpi_shutdown, input, Helper},
    mem_filesystem::FileSystem,
    misc::{print_history, view_logo},
    println, sleep,
};
use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::time::Duration;
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
        m.insert("load_animation", load_animation as fn());
        m.insert("yirsp", yirsp as fn());
        m.insert("write_debug", write_debug as fn());
        m.insert("change_color", change_color as fn());
        m.insert("gbcd", print_current_colors_debug as fn());
        m.insert("color_print_debug", print_debug_colors as fn());
        m.insert("test_colors", test_colors as fn());
        m.insert("neoget", view_logo as fn());
        m.insert("future", print_history as fn());
        m.insert("tester", test_kukilang_code as fn());
        m.insert("viewcom", view_commands as fn());
        Mutex::new(m)
    };
    static ref FILESYSTEM: Mutex<FileSystem> = Mutex::new(FileSystem::new(1024, 128, 512));
    static ref FILES: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn datetime() {}

fn view_commands() {
    println!("Started the command");
    let commands = COMMANDS.lock();
    println!("Command finished and command(s) acknowledged.");
    let commands_list = commands.iter();
    println!("OK!");
    // match commandss {
    //     Err(err) => {}
    //     Ok(_) => {}
    // }

    println!("Command stopped.");
    println_colorful("COMMAND | FUNCTION", "cyan", "black");
    println!("colors");
    println!("Command(s) total: {}", commands.len());
    println!("total");
    for (command, function) in commands.iter() {
        println!("{command} | {function:?}");
    }
}

fn help() {
    white_space_divider(5);
    println!("======================== KukiOS HELP center ========================");
    println!("Hello, This is KukiOS. \n We're happy to see you join our community. \n We are developing, a free linux-like system, entirely in Rust and x86_64 Assembly. \n We love Linux, Mac, but hate Windows. \n This is just a free, non-opensource project. \n --- Kukiweb.cz and KukiOS Admin, Kuki202");
    println!("====================================================================");
}

pub fn clear() {
    let _whitespaces = white_space_divider(40);
}

fn welcome() {
    let name = input(crate::interrupts::Helper::Empty);
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
    let file_name = input(crate::interrupts::Helper::Empty);
    white_space_divider(1);
    println!("Enter file's text: ");
    let content = input(crate::interrupts::Helper::Empty);
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
    let file_name = input(crate::interrupts::Helper::Empty);
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
    let file_name = input(Helper::Is("open_file".to_string()));
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

fn load_animation() {
    let wait_ms: u64 = 7500000;
    println!("[==                  ]");
    sleep(wait_ms);
    println!("[====                ]");
    sleep(wait_ms);
    println!("[=====               ]");
    sleep(wait_ms);
    println!("[======              ]");
    sleep(wait_ms);
    println!("[=======             ]");
    sleep(wait_ms);
    println!("[========            ]");
    sleep(wait_ms);
    println!("[=========           ]");
    sleep(wait_ms);
    println!("[==========          ]");
    sleep(wait_ms);
    println!("[===========         ]");
    sleep(wait_ms);
    println!("[=============       ]");
    sleep(wait_ms);
    println!("[===============     ]");
    sleep(wait_ms);
    println!("[=================== ]");
    sleep(wait_ms);
    println!("[====================]");
    sleep(wait_ms);
    clear();
}

fn echo() {
    println!("");
    println!("What to echo?");
    let echo = input(crate::interrupts::Helper::Empty);
    println!("");
    println!("Where to echo? (file-which, here)");
    let whereto = input(crate::interrupts::Helper::Empty);
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

fn write_debug() {
    println!("THIS IS DEBUG WRITE FUNCTION THAT OCCUPIES A LOT MEMORY. DO YOU WISH TO RUN THIS FUNCTION? (n/y)");
    let command = input(Helper::Is("write_debug".to_string()));
    if command == "y" {
        let mut fs = FILESYSTEM.lock();
        let lorem_ipsum = format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?");
        let file = fs.create_file(1024, "debug.txt").unwrap();
        fs.write_file(file, lorem_ipsum.as_bytes());

        println!("OK! Writing lorem ipsum file - debug.txt");
    }
}

fn yirsp() {
    println!("Y.I.R.S.P. - Your Interactive Reading Service Provider!");
    println!("Enter the name of the file you want to read: ");
    let mut buffer = [0u8; 1024];
    let mut current_limit: u32 = 0;
    let mut upper_limit = 200;
    let fs = FILESYSTEM.lock();
    let filename = input(Helper::Is("Y.I.R.S.P.".to_string()));
    println!("Opening {}", filename.trim_end());
    clear();
    if let Some(bytes_read) = fs.read_file_by_name(filename.trim_end(), &mut buffer) {
        let data = core::str::from_utf8(&buffer[..bytes_read]).unwrap();
        println!("============================================");
        loop {
            println!("{}", first_chars_get(data, current_limit, upper_limit));
            let command = input(Helper::Is("Y.I.R.S.P.".to_string()));
            if command == "d" {
                if data.len() as u32 >= upper_limit || data.len() as u32 >= current_limit {
                    println!("Sorry cannot go down anymore!");
                } else {
                    current_limit += 200;
                    upper_limit += 200;
                }
                clear();
            }
            if command == "u" {
                if current_limit <= 0 {
                    println!("Sorry! Cannot go up anymore!")
                } else {
                    current_limit -= 200;
                    upper_limit -= 200;
                    clear();
                }
            }
            if command == "q" {
                break;
            }
        }
    } else {
        println!("File not found: {filename}");
    }
    println!("=============================================");
}

fn first_chars_get(txt: &str, from: u32, to: u32) -> String {
    let mut final_str = String::new();
    for i in from..to {
        final_str.push_str(txt.chars().nth(i as usize).unwrap().to_string().as_str());
    }
    final_str
}

fn change_color() {
    println!("Enter new colors in this order: fg,bg - quit");
    let fg_bg = input(Helper::Is("change_color/fg_bg".to_string()));
    let fg_bg_order = fg_bg.split(",").collect::<Vec<&str>>();
    let fg = fg_bg_order[0];
    let bg = fg_bg_order[1];
    chg_fg_bg(fg, bg);
    clear();
}

fn print_debug_colors() {
    println!("[OK] Start of function");
    println!("[INFO] BEFORE: ");
    println!("THIS IS COLORFUL!");
    println!("[INFO] NOW: ");

    print_colorful("T", "Blue", "black");
    print_colorful("H", "Black", "black");
    print_colorful("I", "Green", "black");
    print_colorful("S", "Cyan", "black");
    print_colorful(" ", "Red", "black");
    print_colorful("IS", "Magenta", "black");
    print_colorful("", "Brown", "black");
    print_colorful("C", "LightGray", "black");
    print_colorful("O", "DarkGray", "black");
    print_colorful("L", "LightBlue", "black");
    print_colorful("O", "LightGreen", "black");
    print_colorful("R", "LightCyan", "black");
    print_colorful("F", "LightRed", "black");
    print_colorful("U", "Pink", "black");
    print_colorful("L", "Yellow", "black");
    print_colorful("!", "White", "black");
}

fn test_colors() {
    print_colorful("TEST", "blue", "black");
}
