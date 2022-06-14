use std::{
    io::{self, Read, Write},
    net::{self, TcpStream},
    thread::sleep,
    time::Duration,
};

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:2017").unwrap();
    listener.incoming().for_each(|stream| match stream {
        Ok(mut stream) => handle_connection(&mut stream).unwrap(),
        Err(e) => println!("{e:?}"),
    });
}

const CONTENT: &str = "<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
";

fn handle_connection(stream: &mut TcpStream) -> Result<(), io::Error> {
    let mut buf = [0; 1024];

    println!("Stream:");
    stream.read(&mut buf).unwrap();
    print!("{:?}", String::from_utf8_lossy(&buf));

    heavy_job(stream);

    stream.write(
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            CONTENT.len(),
            CONTENT
        )
        .as_bytes(),
    )?;
    stream.flush()
}

fn heavy_job(_: &mut TcpStream) {
    sleep(Duration::from_secs(5))
}
