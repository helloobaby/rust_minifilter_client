//https://github.com/SubconsciousCompute/fsfilter-rs
use libc::getchar;
use rust_minifilter_client::driver_communication::{Driver, IoMessage};
use std::mem::{self, size_of};
use std::os::raw::{c_ulong, c_ulonglong};
use std::process::Command;
use std::string::String;
use std::u16;
use widestring::U16String;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
fn main() {
    // rust类型大小必须与C/C++编译器下类型大小相同
    println!(
        "c_ulong size : {}\nc_ulonglong size :  {}
        IoMessage size : {}\n",
        size_of::<c_ulong>(),
        size_of::<c_ulonglong>(),
        size_of::<IoMessage>()
    );



    let mut drv = Driver { handle: HANDLE(0) };
    drv
        .open_kernel_driver_com()
        .expect("minifilter 驱动已经加载?\n");

    println!("press command..\n");
    while true {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("读命令失败");

        //println!("Command {}\n", input);

        if input == "q\r\n" {
            //println!("exit command\n");
            break;
        }

        // 用户层向内核层发消息
        if input == "a\r\n" {
            drv.seen_message();
            continue;
        }
    }

    drv.close_kernel_communication();
}
