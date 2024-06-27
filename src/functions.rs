// use core::arch::{asm};

// use crate::filesystem::FileSystem;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::println;

pub fn _translate_to_string_utf8loosy(buffer: &[u8]) -> String {
    String::from_utf8_lossy(buffer).to_string()
}

//.section .text

// global_asm!(
//     r#"
// .globl open_dir_a
// .globl read_dir_a
// .globl close_dir_a

// open_dir_a:
//     push rbx
//     push rbp
//     mov rbx, rdi
//     mov rax, 1
//     pop rbp
//     pop rbx
//     ret

// read_dir_a:
//     push rbx
//     push rbp
//     mov rbx, rdi
//     mov rcx, rsi
//     mov rdx, rdx
//     mov rax, 64
//     pop rbp
//     pop rbx
//     ret

// close_dir_a:
//     push rbx
//     push rbp
//     mov rbx, rdi
//     mov rax, 0
//     pop rbp
//     pop rbx
//     ret
// "#
// );

// extern "C" {
//     fn open_dir_a() -> isize; //path: *const u8
//     fn read_dir_a() -> isize; //buffer: *mut u8, size: usize
//     fn close_dir_a(); // fd: isize
// }

// extern "C" {
//     fn open_dir() -> u64;
//     fn read_dir() -> u64;
//     fn close_dir() -> u64;
// }

//include_str!("syscalls.asm")
// pub fn shutdown() {
//     unsafe {
//         asm!(
//             "mov ax, 0x1000",
//             "mov ax, ss",
//             "mov sp, 0xf000",
//             "mov ax, 0x5307",
//             "mov bx, 0x0001",
//             "mov cx, 0x0003",
//             "int 0x15",
//         );
//     }
// }

pub fn _last_two_keys(keys: &mut Vec<char>) -> &[char] {
    if keys.len() >= 2 {
        &keys[keys.len() - 2..]
    } else {
        &[]
    }
}

// fn x() {
//     let mut x: Vec<char> = Vec::new();
//     x.push("a".parse().unwrap());
//     let sht: [char; 2] = ["z".parse().unwrap(), "l".parse().unwrap()];
//     let la = last_two_keys(&mut x);
//     println!("{:#?}", la);
//     if la.contains(&sht[0]) && la.contains(&sht[1]) {
//         println!("nice")
//     }
// }

pub fn _help() {
    println!("Welcome to Kuki OS! \n\n We're thrilled to have you join our community. \n Whether you're a seasoned tech enthusiast or just getting started, we've crafted an intuitive and dynamic environment to make your computing experience smooth and enjoyable.\nFrom seamless navigation to powerful features, we've designed KukiOS with you in mind. \n\n So, welcome aboard! Feel free to explore, customize, and make this OS your own. Let's embark on this journey together, where innovation meets simplicity. Enjoy your stay!");
}

#[allow(unused)]
const BUFFER_SIZE: usize = 1024;

#[allow(unused)]
struct DirEntry {
    inode: u64,
    offset: i64,
    reclen: u16,
    name_len: u8,
    name: [u8; 255],
}

// pub fn list_directory(path: &str) {
//     let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
//     let path_cstr = {
//         let mut buf = [0u8, 64];
//         let bytes = path.as_bytes();
//         for (i, &byte) in bytes.iter().enumerate() {
//             buf[i] = byte;
//         }
//         buf
//     };
//     let fd: isize;
//     unsafe {
//         asm!(
//             "call open_dir",
//             in("rdi") path_cstr.as_ptr(),
//             lateout("rax") fd,
//         );
//     }
//     if fd < 0 {
//         panic!("Failed to open directory!");
//     }
//     loop {
//         let nread: isize;

//         unsafe {
//             asm!("call read_dir",
//             in("rdi") buffer.as_mut_ptr(),
//             in("rsi") BUFFER_SIZE,
//             lateout("rax") nread,
//             );
//         }
//         if nread == 0 {
//             break;
//         } else if nread < 0 {
//             panic!("Failed to read directory entries!");
//         }
//         let mut bpos = 0;
//         while bpos < nread as usize {
//             let d = unsafe { &*(buffer.as_ptr().add(bpos) as *const DirEntry) };
//             let name = unsafe {
//                 let name_ptr = buffer
//                     .as_ptr()
//                     .add(bpos)
//                     .add(core::mem::size_of::<DirEntry>())
//                     as *const u8;
//                 let name_len = d.reclen as usize - core::mem::size_of::<DirEntry>();
//                 core::slice::from_raw_parts(name_ptr, name_len)
//             };
//             let name_str = core::str::from_utf8(name).unwrap_or("<invalid utf-8>");
//             println!("{}", name_str.trim_end_matches("\0"));
//             bpos += d.reclen as usize;
//         }
//     }
//     unsafe {
//         asm!("call close_dir", in("rdi") fd,);
//     }
// }

pub fn _list_dir() {
    println!("Reading test.txt");
    let mut _buffer = [0u8; 1024];

    // use alloc::vec;
    // use core::str;
    // let dirname = "./";
    // let dirname_cstr = CString::new(dirname).unwrap();
    // let buffer_len = 1024;
    // let mut buffer = vec![0u8; buffer_len];
    // unsafe {
    //     let result = read_directory(dirname_cstr.as_ptr());
    //     if result == -1 {
    //         panic!("FAILED TO READ DIRECTORY!")
    //     } else {
    //         let bytes_read = result as usize;
    //         println!("Read {bytes_read} bytes from your directory.");
    //         for entry in buffer.chunks_exact(256).take(bytes_read / 256) {
    //             let entry_str = from_utf8(entry).unwrap_or("ERROR: Invalid UTF-8");
    //             println!("Directory Entry: {entry_str}");
    //         }
    //     }
    // }
    // let ls = unsafe {
    //     get_current_directory();
    // };
    // println!("{:?}", ls);
    // let mut fs = FileSystem::new(1024, 128, 512);
    // let file_inode = fs.create_file(1024);
    // let data = b"a";
    // fs.write_file(file_inode, data);
}
// fn load_pretty() {
//     let msgs = vec!["[=  ]", "[ = ]", "[  =]", "[ = ]"];
//     loop {
//         for msg in msgs.iter() {
//             print!("{msg}\r");

//         }
//     }
// }
