
use crate::traits::runnable::Runnable;

pub struct Payload;

// In payload.rs
impl Runnable for Payload {
    fn run(&self) {
        println!("Payload module initializing...");
    }
}

// Similarly, for server.rs and module.rs
