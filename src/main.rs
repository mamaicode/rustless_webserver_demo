#![deny(warnings)]

// Import necessary modules from the hyper crate
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
// Import `Infallible` for handling errors
use std::convert::Infallible;
// Import `SocketAddr` for defining the server address
use std::net::SocketAddr;

// Define an asynchronous function `rustless` which handles HTTP requests
async fn rustless(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Respond with "Hello, Rustless!"
    Ok(Response::new("Hello, Rustless!".into()))
}

// Define the main function using the Tokio runtime
#[tokio::main]
async fn main() {
    // Define the address to bind the server to (in this case, 127.0.0.1:8080)
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    // Create a service factory using `make_service_fn`
    // This will create a new `Service` for each incoming connection
    let make_svc = make_service_fn(|_conn| async {
        // Convert the `rustless` function into a `Service` using `service_fn`
        Ok::<_, Infallible>(service_fn(rustless))
    });

    // Create a new HTTP server that listens on the specified address
    let server = Server::bind(&addr).serve(make_svc);

    // Start the server and handle any errors that might occur
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}