use std::{io::{BufRead, Write}, net::{TcpListener, TcpStream}};
use multithreaded_web_server::Threadpoll;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let threadpoll = Threadpoll::new(3);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        threadpoll.execute(|| {
            handle_connection(stream)
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = std::io::BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (filename,status) = if request_line == "GET / HTTP/1.1" {
        ("hello.html", "HTTP/1.1 200 OK")
    }else if request_line == "GET /sleep HTTP/1.1" {
        std::thread::sleep(std::time::Duration::from_secs(10));
        ("sleep.html", "HTTP/1.1 200 OK")
    }else {
        ("404.html", "HTTP/1.1 404 NOT FOUND")
    };
   
    let file_content = std::fs::read_to_string(String::from("scripts/") +filename).unwrap();
    let length = file_content.len();
    let response =
        format!("{status}\r\nContent-Length: {length}\r\n\r\n{file_content}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
