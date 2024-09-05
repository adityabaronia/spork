mod processlist;
mod cmdexec;
mod keylogger;
mod network;
use std::io::{self, Write};



fn command_exec(){   
    print!("Enter command: ");
    io::stdout().flush().unwrap(); 
    let mut command = String::new();
    io::stdin()
         .read_line(&mut command)
         .expect("Failed to read line");
    // println!("{}", name);
    
    println!("{}", command);
    cmdexec::cmdexec(command)
}


fn list_process(){
    processlist::processlist();
}



fn main() {
    println!("Hello from main");
    //keylogger::keylog()
    network::network();
}
