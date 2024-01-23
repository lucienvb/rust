use hyper::{service::make_service_fn, Body, Request, Response, Server};

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Your request handling logic here
    let response = Response::new(Body::from("Hello, Hyper!"));
    Ok(response)
}

#[tokio::main]
async fn main() {
    // Create a make_service_fn that returns a service for each incoming connection
    let make_svc = make_service_fn(|_| {
        async {
            Ok::<_, hyper::Error>(hyper::service::service_fn(handle_request))
        }
    });

    // Build a server and run it
    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
