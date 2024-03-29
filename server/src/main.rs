use server::ThreadPool;
use std::time::Duration;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7777").unwrap();

    let pool = ThreadPool::new(4);

    // Exit after receiving two requests.
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down")
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // lines is actually an iterator
    let mut lines = buf_reader.lines();

    let request_line = lines.next().unwrap().unwrap();

    // match a slice here. Pass reference to the match
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" if request_line.len() < 100 => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" | "GET /sleep/ HTTP/1.1" if request_line.len() < 100 => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}
