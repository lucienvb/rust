use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Handle your HTTP request logic here
    let response = Response::new(Body::from("Hello, Rust HTTP server!"));

    Ok(response)
}

// #[tokio::main]
async fn server() {
    // Create a service to handle incoming requests
    let make_svc = make_service_fn(|_conn| {
        let handle_request = service_fn(handle_request);
        async { Ok::<_, Infallible>(handle_request) }
    });

    // Create a server bound to 127.0.0.1:8080
    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // Start the server
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

fn main() {
    server();
}
