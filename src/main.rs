use std::{net::{TcpListener, TcpStream}, io::{Write, BufReader, BufRead}, collections::HashMap, error::Error};

struct ReqHead {
    proto: String,
    method: String,
    path: String,
}

fn handle_connection(conn: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let buf_read = BufReader::new(conn.try_clone()?);
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut req_head: ReqHead;

    for (idx, line) in buf_read.lines().enumerate() {
        let line = line.expect("Couldn't read line from conn");

        if line.is_empty() {
            break;
        }

        if idx == 0 {
            // this is the req line
            let parts: Vec<&str> = line.split(' ').collect();
            assert_eq!(parts.len(), 3);
            req_head = ReqHead {
                method: parts.get(0).unwrap().to_string(),
                path: parts.get(1).unwrap().to_string(),
                proto: parts.get(2).unwrap().to_string(),
            };
            println!("{}: {} on path {}", req_head.proto, req_head.method, req_head.path);
            continue;
        }
        // parse a header
        let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
        headers.insert(
            parts.get(0).unwrap().to_ascii_lowercase(),
            parts.get(1).unwrap().to_string());

    }

    println!("Received {} headers", headers.len());

    let mut message: String = String::from("Hello! This is a demo Rust app that responds to HTTP requests. Just for fun, here are your headers:<ul>");
    for (key, value) in headers {
        message.push_str(format!("<li>{}: {}</li>", key, value).as_str());
    }
    message.push_str("</ul>Thank you!");
    conn.write(
            format!("HTTP/1.1 200 OK\nContent-Type: text/html\nContent-Length: {}\n\n{}\n", message.len(), message).as_bytes()
        )
        ?;

    conn.flush()?;
    // conn.shutdown(std::net::Shutdown::Both).unwrap();
    println!("<<<<< Connection terminated");
    return Ok(());
}

fn main() {
    let listener = 
        TcpListener::bind("127.0.0.1:1235")
        .expect("Could not open port");

    println!("Opened port 1235");
    for conn in listener.incoming() {
        let mut conn = conn.expect("Error opening connection");
        println!(">>>>> Handling connection");
        match handle_connection(&mut conn) {
            Ok(v) => v,
            Err(err) => println!("<<<<<! Error handling connection: {}", err),
        }
    }
}
