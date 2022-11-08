use proxy_server_rs::ThreadPool;
use itertools::Itertools;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).unwrap();

    let (verb, path, protocol) = request_line
        .splitn(3, " ")
        .collect_tuple()
        .unwrap_or_else(|| panic!("Invalid request line: {}", request_line));


    let response = match path {
        "/" => match verb {
            "GET" => {
                let status_line = "HTTP/1.1 200 OK";
                let contents = fs::read_to_string("hello.html").unwrap();
                let length = contents.len();
                format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
            }
            _ => {
                "HTTP/1.1 405 Method Not Allowed\r\n\r\n".to_string()
            }
        }
        "/sleep" => match verb {
            "GET" => {
                thread::sleep(Duration::from_secs(5));
                let status_line = "HTTP/1.1 200 OK";
                let contents = fs::read_to_string("hello.html").unwrap();
                let length = contents.len();
                format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
            }
            _ => {
                "HTTP/1.1 405 Method Not Allowed\r\n\r\n".to_string()
            }
        }
        _ => {
            let status_line = "HTTP/1.1 404 Not Found";
            let contents = fs::read_to_string("404.html").unwrap();
            let length = contents.len();
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
        }
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
