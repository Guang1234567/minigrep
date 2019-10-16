use std::env;
use std::process;

mod server_demo;

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

    println!("dgsdgsdg");
    server_demo::demo_tcp_listener();
    println!("eryy");
}


