/**
 * The purpose of this module is to be the brain to organize, run, and restart child processes
 * 
 */
use std::collections::HashMap;
use crate::modules::{payload, server, module};
use crate::traits::runnable::Runnable;

use std::process::Child;
use std::sync::Mutex;

pub static KNOWN_PROCESSES: Mutex<Vec<String>> = Mutex::new(Vec::new());
pub static CHILD_PROCESSES: Mutex<Vec<Child>>  = Mutex::new(Vec::new());

pub static DEFAULT_PROCESSES: Mutex<Vec<String>> = Mutex::new(Vec::new());


pub fn run () {
    use std::sync::MutexGuard;
    let default_processes: Vec<String> = vec![
        "payload".to_string(), 
        "server".to_string(), 
        "module".to_string()
    ];

    DEFAULT_PROCESSES.lock().unwrap().extend(default_processes);

    // TODO: Streamline this Hashmap insertion process later
    let mut process_map: HashMap<String, Box<dyn Runnable>> = HashMap::new();
    process_map.insert("payload".to_string(), Box::new(payload::Payload));
    process_map.insert("server".to_string(), Box::new(server::Server));
    process_map.insert("module".to_string(), Box::new(module::Module));

    println!("Main module initializing with {:?} default processeses...", DEFAULT_PROCESSES.lock().unwrap());
    
    // Call the run functions of each DEFAULT_PROCESS
    for process in DEFAULT_PROCESSES.lock().unwrap().iter() {
        // println!("Spawning default process: {}", process);
        if let Some(process) = process_map.get(process) {
            process.run();
        } else {
            println!("Unknown process: {}", process);
        }
    }

    let child_processes: MutexGuard<Vec<Child>> = CHILD_PROCESSES.lock().unwrap();
    println!("There are {} child processes running", child_processes.len());

    let known_processes: MutexGuard<Vec<String>> = KNOWN_PROCESSES.lock().unwrap();
    println!("There are {} known processes", known_processes.len());
}