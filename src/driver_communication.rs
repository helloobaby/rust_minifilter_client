use libc::memcpy;
use libc::{memccpy, strncpy};
use std::ffi::c_void;
use std::os::raw::{c_char, c_ulong, c_ulonglong};
use wchar::wchar_t;
use widestring::{U16CString, U16String};
use windows::core::{HRESULT, PCSTR, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Storage::InstallableFileSystems::{
    FilterConnectCommunicationPort, FilterSendMessage,
};

pub struct IoMessage {
    is_safe: c_char,
    pid: c_ulong,
    Handle: c_ulonglong,
    pub FileName: [wchar_t; 256],
}

pub struct Driver {
    pub handle: HANDLE,
}

impl Driver {
    pub fn close_kernel_communication(&self) -> bool {
        unsafe { CloseHandle(self.handle).as_bool() }
    }

    pub fn open_kernel_driver_com(&mut self) -> Result<&mut Self, windows::core::Error> {
        let com_port_name = U16CString::from_str("\\naGuardPort").unwrap().into_raw();

        unsafe {
            self.handle = FilterConnectCommunicationPort(
                PCWSTR(com_port_name),
                0,
                std::ptr::null(),
                0,
                std::ptr::null(),
            )?;
        }
        Ok(self)
    }
    pub fn seen_message(&mut self) {
        // 创建信息结构体
        let mut msg = IoMessage {
            is_safe: 0x6,
            pid: 0x66666666,
            Handle: 0x6666666666666666,
            FileName: [0; 256],
        };

        let str = U16String::from("helloworld.txt");
        unsafe {
            memcpy(
                msg.FileName.as_ptr() as *mut c_void,
                str.as_ptr() as *const c_void,
                str.len(),
            );
        }

        let mut res_size: u32 = 0;
        unsafe {
            FilterSendMessage(self.handle, std::ptr::addr_of_mut!(msg).cast::<std::ffi::c_void>(), std::mem::size_of::<IoMessage>() as c_ulong, std::ptr::null_mut(), 0, std::ptr::addr_of_mut!(res_size).cast::<u32>());
        }
    }
}
