
use windows::{
    core::*,
    Win32::UI::WindowsAndMessaging::*
};

fn main() {
    println!("Hello, world!");
    unsafe {
        MessageBoxA(None, s!("Ansi"), s!("World"), MB_OK);
        MessageBoxW(None, w!("Wide"), w!("World"), MB_OK);
    }
}
