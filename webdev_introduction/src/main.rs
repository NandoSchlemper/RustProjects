use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:1478") {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Ocorreu um erro ao fazer o vinculo: {}", e);
            return;
        }
    };

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Ocorreu um erro: {}", e);
                return;
            }
        };

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = reader
        .lines()
        .map(|resultado| resultado.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}
