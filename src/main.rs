
use std::env;
use std::process;
use minigrep::server_demo;


fn main() {

    /*let config = Config::new(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {:?}", err);
            process::exit(1)
        });

    if let Err(e) = config.run() {
        eprintln!("Application error: {:?}", e);

        process::exit(1);
    }*/

    server_demo::demo_tcp_listener();
}


