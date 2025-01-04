use terminal::CURRENT_COLOR;

use crate::{
    print, println,
    vga_buffer::{Color, WRITER},
};

mod terminal;

pub fn chg_fg_bg(fg: &str, bg: &str) {
    // let mut current_color = CURRENT_COLOR.lock();
    // let mut current_color_n = current_color.0;
    // let mut current_color_o = current_color.1;
    let final_fg = text_to_color(fg);
    let final_bg = text_to_color(bg);
    let mut writer = WRITER.lock();
    // current_color_n = fg as &str; //fg
    // current_color_o = bg as &str; //bg
    // println!(
    //     "Changed fg to {} and bg to {} in the system color registry",
    //     current_color_n, current_color_o
    // );
    writer.change_fg_bg(final_fg, final_bg);
}

pub fn println_colorful(t: &str, f: Color, b: Color) {
    let mut writer = WRITER.lock();
    writer.change_fg_bg(f, b);
    println!("{t}");
    writer.change_fg_bg(Color::White, Color::Black);
}

pub fn print_colorful(t: &str, f: &str, b: &str) {
    chg_fg_bg(f, b);
    print!("{t}");
    chg_fg_bg("white", "black");
}

pub fn print_current_colors_debug() {
    let cc = get_current_colors_terminal();
    println!("FG: {0} BG: {1}", cc.0, cc.1);
}

pub fn get_current_colors_terminal() -> (&'static str, &'static str) {
    let current_color = CURRENT_COLOR.lock();
    (current_color.0, current_color.1)
}

fn text_to_color(t: &str) -> Color {
    match t {
        "black" => Color::Black,
        "blue" => Color::Blue,
        "green" => Color::Green,
        "cyan" => Color::Cyan,
        "red" => Color::Red,
        "magenta" => Color::Magenta,
        "brown" => Color::Brown,
        "light_gray" => Color::LightGray,
        "dark_gray" => Color::DarkGray,
        "light_blue" => Color::LightBlue,
        "light_green" => Color::LightGreen,
        "light_cyan" => Color::LightCyan,
        "light_red" => Color::LightRed,
        "pink" => Color::Pink,
        "yellow" => Color::Yellow,
        "white" => Color::White,
        _ => Color::Black,
    }
}
