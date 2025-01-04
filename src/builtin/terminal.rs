use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref CURRENT_COLOR: Mutex<(&'static str, &'static str)> =
        Mutex::new(("white", "black"));
}
