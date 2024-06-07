use core::arch::asm;

use alloc::vec::{self, Vec};

use crate::println;

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

pub fn last_two_keys(keys: &mut Vec<char>) -> &[char] {
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

pub fn help() {
    println!("Welcome to Kuki OS!\n We're thrilled to have you join our community and explore the endless possibilities our operating system has to offer. Whether you're a seasoned tech enthusiast or just getting started, we've crafted an intuitive and dynamic environment to make your computing experience smooth and enjoyable.\nFrom seamless navigation to powerful features, we've designed [Your OS Name] with you in mind. Dive into productivity, creativity, and entertainment with confidence, knowing that our OS is here to support your every need. \n So, welcome aboard! Feel free to explore, customize, and make this OS your own. Let's embark on this journey together, where innovation meets simplicity. Enjoy your stay!");
}
