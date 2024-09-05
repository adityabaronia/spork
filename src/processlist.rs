extern crate winapi;



use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, PROCESSENTRY32, Process32First, Process32Next};
use winapi::um::winnt::HANDLE;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;


//by default the functions in rust are private 
pub fn processlist(){
    unsafe{
    println!("From process list");
    let h_snap: HANDLE ;
    let mut p_entry: PROCESSENTRY32 = std::mem::zeroed();
    p_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
        
    h_snap =  CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0 ) ;
    if h_snap == INVALID_HANDLE_VALUE{ 
        println!("INVALID HANDLE FOR CREATE SNAPSHOT");
    }

    Process32First(h_snap, &mut p_entry);
    println!("{}",p_entry.th32ProcessID);

    let flag: bool = true;
    while flag {
        let flag: i32 = Process32Next(h_snap, &mut p_entry);
        if flag == 0{
            break;
        }
        
        
        println!("{}",p_entry.th32ProcessID);
       
    }
}
}


