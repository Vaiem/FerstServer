use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread::{self, Thread},
    time::Duration,
};
use FerstServer::ServerThread::ThreadPool;

fn main() {
    
    let tcpConect = TcpListener::bind("127.0.0.1:7878").unwrap();

    

    let pool = ThreadPool::new(4);
    for stream in tcpConect.incoming() {
        
        let mut streamCurrent = stream.unwrap();

        pool.execut(|| {
            http_Response(streamCurrent);
        });
        
        
    }
    
    
    println!("Hello, world!");
}

fn http_Response(mut Stream : TcpStream){

    //thread::sleep(Duration::from_secs(10));
    let buffer_read = BufReader::new(&mut Stream);

    let hader_requst = buffer_read.lines().next().unwrap().unwrap();

    let (status_requst, filename) = if hader_requst == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "Views/home.html" )
    }
    else{
        ("HTTP/1.1 200 OK", "Views/err.html" )
    };

    let status_line = status_requst;
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    Stream.write_all(response.as_bytes()).unwrap();

    
}



fn print_requst(stream : &mut TcpStream) {

    let buffer_read = BufReader::new(stream);
    
    let requst : Vec<_> = buffer_read
                                .lines()
                                .map(|result| result.unwrap())
                                .take_while(|line| !line.is_empty())
                                .collect();
    
    println!("Requst {:#?}", requst);

}


