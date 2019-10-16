use std::net::TcpListener;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}

pub fn demo_tcp_listener() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let received = stream.unwrap();

        println!("Connection established!  received = {:?}", received)
    }
}