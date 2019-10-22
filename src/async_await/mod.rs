use {
    futures::{
        // Extension trait for futures 0.1 futures, adding the `.compat()` method
        // which allows us to use `.await` on 0.1 futures.
        compat::Future01CompatExt,
        // Extension traits providing additional methods on futures.
        // `FutureExt` adds methods that work for all futures, whereas
        // `TryFutureExt` adds methods to futures that return `Result` types.
        future::{FutureExt, TryFutureExt},
    },
    hyper::{
        // Miscellaneous types from Hyper for working with HTTP.
        Body, Client, Request, Response, rt::run, Server,

        // This function turns a closure which returns a future into an
        // implementation of the the Hyper `Service` trait, which is an
        // asynchronous function from a generic `Request` to a `Response`.
        service::service_fn,

        // A function which runs a future to completion using the Hyper runtime.
        Uri,
    },
    std::net::SocketAddr,
};

async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Got request at {:?}", req.uri());

    let url_str = "http://www.rust-lang.org/en-US/";
    let url = url_str.parse::<Uri>().expect("failed to parse URL");

    let res = Client::new().get(url).compat().await;
    // Return the result of the request directly to the user
    println!("request finished --returning response");
    res

    //Ok(Response::new(Body::from("hello, world!")))
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    let serve_future = Server::bind(&addr)
        .serve(|| service_fn(|req| serve_req(req).boxed().compat()));

    if let Err(e) = serve_future.compat().await {
        eprintln!("server error: {}", e);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, 2);

        // Set the address to run our socket on.
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

        // Call our `run_server` function, which returns a future.
        // As with every `async fn`, for `run_server` to do anything,
        // the returned future needs to be run. Additionally,
        // we need to convert the returned future from a futures 0.3 future into a
        // futures 0.1 future.
        let futures_03_future = run_server(addr);
        let futures_01_future = futures_03_future.unit_error().boxed().compat();

        // Finally, we can run the future to completion using the `run` function
        // provided by Hyper.
        run(futures_01_future);
    }
}