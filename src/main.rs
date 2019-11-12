extern crate winapi;

use std::ffi::OsStr;
//use std::os::windows::ffi::OsStrExt;
use std::ffi::OsString;
use std::io::Error;
use std::os::windows::prelude::*;
use std::ptr::null_mut;

use winapi::{
    shared::{minwindef::LPARAM, windef::HWND},
    um::{
        winuser::{
            EnumWindows, GetWindowTextW,
        },
    },
};

extern "system" fn enum_windows_proc(hwnd: HWND, l_param: LPARAM) -> i32 {
    unsafe {
        let mut lp_string = [0; 128];
        let len = GetWindowTextW(hwnd, lp_string.as_mut_ptr(), 128);
        let title = OsString::from_wide(&lp_string[..len as usize]);
        println!("{:?} {:?}", hwnd, title);
        if title.to_str().unwrap().contains("微信") {
            (*(l_param as *mut McClient)).h_wnd = hwnd;
            0
        } else { 1 }
    }
}


struct McClient {
    h_wnd: HWND
}

impl McClient {
    fn new() -> McClient {
        McClient {
            h_wnd: 0 as HWND
        }
    }
    fn find(&mut self, title: &str) {
        unsafe {
            EnumWindows(Some(enum_windows_proc), self as *mut McClient as LPARAM)
        };
        println!("{:?}", self.h_wnd);
    }
}

fn print_message(msg: &str) -> Result<i32, Error> {
    use std::iter::once;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) } else { Ok(ret) }
}

fn main() {
    println!("Hello, world!");
    let mut mc_client = McClient::new();
    mc_client.find("微信");
    print_message("Hello, world!").unwrap();
}