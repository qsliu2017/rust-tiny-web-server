use std::{
    io::{Read, Write},
    net,
};

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:2017").unwrap();
    match listener.accept() {
        Ok((mut stream, addr)) => {
            println!("Accept conn from {addr:?}");
            let mut buf = [0; 1024];

            stream.read(&mut buf).unwrap_or_else(|e| panic!("{e}"));
            println!("Got stream: {buf:?}");

            write!(&mut stream, "HTTP/1.0 200 OK\r\n\r\nHello World\r\n")
                .unwrap_or_else(|e| panic!("{e}"));
            stream.flush().unwrap_or_else(|e| panic! {"{e}"});
        }
        Err(e) => panic!("{}", e),
    }
}
