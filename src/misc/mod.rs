use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use ascii::LOGO;

use crate::{
    builtin::{print_colorful, println_colorful},
    interrupts::HISTORY,
    println,
};

mod ascii;

pub fn view_logo() {
    println_colorful(LOGO, "cyan", "black");
}

pub fn vec_to_str(le: Vec<char>) -> String {
    let mut finished_s = String::new();
    for x in 0..le.len() {
        finished_s.insert_str(x, le[x].to_string().as_str());
    }
    finished_s
}

pub fn print_history() {
    let history = HISTORY.lock();
    for (command, index) in history.iter() {
        println_colorful(command, "cyan", "black");
    }
}
