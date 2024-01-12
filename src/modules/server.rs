
use crate::traits::runnable::Runnable;

use tokio::runtime::Runtime;
use warp::Filter;
use serde::{Serialize, Deserialize};

use std::thread;

pub struct Server;


// http://127.0.0.1:3030/hello/paul This work when it's not wrapped in a new thread
fn boot() {
        let rt = Runtime::new().expect("Failed to create Tokio runtime");

        rt.block_on(async {
            println!("Tokio runtime initialized....");
            
            #[derive(Serialize, Deserialize)]
            struct MyResponse {
                message: String,
            }

            // Define a route
            let hello_route = warp::path!("hello" / String)
            .map(move |name| {
                let response = MyResponse {
                    message: format!("Hello, {}!", name)
                };
                warp::reply::json(&response)
            });

            // Start the server
            warp::serve(hello_route)
            .run(([127, 0, 0, 1], 3030))
            .await;
            // thread::spawn(move || {});
        });
}

// In payload.rs
impl Runnable for Server {
    fn run(&self) {
        println!("Server module initializing...");
        let server_thread = thread::spawn(|| {
            boot(); 
        });

        // Optionally, wait for the server thread to complete
    let _ = server_thread.join();
    }
}

