
use crate::traits::runnable::Runnable;

use tokio::runtime::Runtime;
use warp::Filter;
use serde::{Serialize, Deserialize};

use std::thread;

/**
 * Current Issue: The tokio runtime blocks the main thread once it starts loading
 
 ChatGPT says:
 Run Other Modules Asynchronously: If your other modules can be run asynchronously, you can modify them to be async and use Tokio's utilities (like tokio::spawn) to run them concurrently with your server. This way, they can all operate within the same Tokio runtime.

Use a Multi-threaded Tokio Runtime: If your application structure allows, you can use a multi-threaded Tokio runtime. This would enable you to run multiple tasks concurrently within the same runtime. You can configure the runtime to have multiple threads by using #[tokio::main(flavor = "multi_thread")].

Separate Runtime in a Different Thread: If you need to keep the server running in its own thread without blocking the main thread, you can create a separate Tokio runtime in a new thread 
 */

pub struct Server;

// http://127.0.0.1:3030/hello/paul This work when it's not wrapped in a new thread
// Also works if I put the tokio::main macro above boot, and async before boot and remove the extra runtime - but that then blocks the rest of my main modules from loading
// #[tokio::main]
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
        let server_thread = thread::spawn(|| { boot(); });

        
        let _ = server_thread.join(); // Optionally, wait for the server thread to complete ( This is required for Tokio to startup )
    }
}

