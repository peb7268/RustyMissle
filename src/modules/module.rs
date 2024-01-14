
use crate::traits::runnable::Runnable;

#[derive(Clone)]
pub struct Module;

// In payload.rs
impl Runnable for Module {
    fn run(&self) {
        println!("Module module initializing...");
    }
}