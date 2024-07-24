use alloc::string::String;

use crate::{interrupts::input, print, println};

pub struct Network {
    pub ssid: &'static str,
    pub password: &'static str,
}

impl Network {
    pub fn init(ssid: &'static str, password: &'static str) -> Self {
        Self { ssid, password }
    }
    pub fn user_pick() -> Option<Network> {
        println!("Network connecting process has been started.");
        println!("Enter the SSID: ");
        let ssid = input();
        println!("Enter password: ");
        let password = input();
        let stars = star_gen(password.len() as i32);
        println!("Your ssid is {ssid} and password is {stars}");
        todo!()
    }
}

fn star_gen(hm: i32) -> String {
    let mut stars = String::new();
    for _ in 0..hm {
        stars.push_str("*");
    }
    stars
}
