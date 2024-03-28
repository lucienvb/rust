// test with: curl -p http://127.0.0.1:8080

// FUNCTION DESCRIPTIONS
// - make_service_fn is used to create a service for each incoming connection. It returns a MakeService instance.
// - service_fn is used to convert the handle_request function into a Service instance.
// - Server::bind(&addr) sets up the server to listen on the specified address.
// - serve(make_svc) starts serving HTTP requests using the provided make_service implementation.
// - server.await is used to wait for the server to complete serving.

use hyper::{service::make_service_fn, Body, Request, Response, Server};

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let response = Response::new(Body::from("Hello, Hyper!"));
    Ok(response)
}

#[tokio::main]
async fn main() {
    let make_svc = make_service_fn(|_| {
        async {
            Ok::<_, hyper::Error>(hyper::service::service_fn(handle_request))
        }
    });

    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
