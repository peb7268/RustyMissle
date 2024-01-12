
/*
* RustyMissle
* A modular, multi-process, multi-threaded, multi-platform, multi-architecture, practice project for malware delivery and execution C2 style.
* Can be used for other purposes as well but thinking about this in the context of malware, for learning purposes only
*
* Objectives:
* 1. [x] Create a main process which organizes / controls child processes, monitors for restarts, ect..
* 2. [ ] Create a child process which will be an payload client which will poll a server for payloads. Payloads can be updates, new modules, rule conditions for payload invocation and delivery, ect..
* 3. Create a child process which will be a server which will serve payloads
* 4. Create a child process which will be a module service which dictates which modules to run and spawns child processes
* 
*/


mod traits {
    pub mod runnable;
}

mod modules {
    pub mod payload;
    pub mod server;
    pub mod module;
    pub mod main_module;
}

use crate::modules::main_module; 

// Set the project name as a constant
pub static PROJECT_NAME: &str = "RustyMissle"; 

fn main() {
    println!("Starting now {} ...", PROJECT_NAME);
    main_module::run();
}
