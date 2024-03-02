mod pool;

use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use pool::ThreadPool;

fn main() {
    let mut port = 8080;

    while TcpListener::bind(format!("127.0.0.1:{port}")).is_err() {
        port += 1
    }
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    println!("Listener bound to 127.0.0.1:{port}");

    let pool = ThreadPool::new(4).unwrap();
    listener.incoming().for_each(|stream| {
        if let Ok(stream) = stream {
            pool.execute(|| handle_connection(stream));
        }
    });
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK\r\n\r\n", "public/index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(2));
            ("HTTP/1.1 200 Ok\r\n\r\n", "public/index.html")
        }
        _ => ("HTTP/1.1 404 Not found\r\n\r\n", "public/404.html"),
    };

    create_response(status_line, filename, stream);
}

fn create_response(status_line: &str, html_path: &str, mut stream: TcpStream) {
    let contents = fs::read_to_string(html_path).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
