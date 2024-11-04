use winapi::um::processthreadsapi::{CreateProcessA, STARTUPINFOA, PROCESS_INFORMATION};
//use winapi::um::winbase::{CREATE_NEW_CONSOLE, CREATE_NO_WINDOW};
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use std::ffi::CString;
use std::mem;
use std::ptr;

pub fn cmdexec(mut cmd: String){

    unsafe{
        
        let mut startup_info:  STARTUPINFOA = std::mem::zeroed();
        let mut process_info:  PROCESS_INFORMATION = std::mem::zeroed();

        startup_info.cb = mem::size_of::<STARTUPINFOA>() as u32;
        println!("{}", cmd);
        let c_string = CString::new("C:\\Windows\\System32\\cmd.exe").expect("CString::new failed");
        let c_string_ptr: *const i8 = c_string.as_ptr();

        let flag: String = String::from("/c ");
        cmd = flag + &cmd;
        println!("command with flag: {}", cmd);
        let result = CreateProcessA( /*ptr::null()*/c_string_ptr, cmd.as_ptr() as *mut i8, 
            ptr::null_mut() as *mut SECURITY_ATTRIBUTES, ptr::null_mut() as *mut SECURITY_ATTRIBUTES,
                false as i32,  0,
                  ptr:: null_mut(), ptr::null_mut(),  &mut startup_info, &mut process_info);
    
        if result == 0 {
            println!("Failed to create process.");
        } 
        else {
            println!("Process created successfully.");
        }
    }
}