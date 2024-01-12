
use crate::traits::runnable::Runnable;

pub struct Module;

// In payload.rs
impl Runnable for Module {
    fn run(&self) {
        println!("Module module initializing...");
    }
}