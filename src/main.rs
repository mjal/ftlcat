extern crate clap;

use std::io;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use clap::{Arg, App};

fn sending_mode(address: &str, port: &str) {
    let mut stream = TcpStream::connect(format!("{}:{}", address, port));
}

fn receiving_mode(address: &str, port: &str) {

    let listener = TcpListener::bind(format!("{}:{}", address, port)).unwrap();

    /*
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    */

    match listener.accept() {
        Ok((_socket, addr)) => println!("new client: {:?}", addr),
        Err(e) => println!("couldn't get client: {:?}", e),
    }

}

/*
fn handle_client(stream: TcpStream) {
    let _data = stream.read(&mut [0; 128]);
}
*/

fn main() -> io::Result<()> {

    let matches = App::new("ftlcat")
        .version("0.1")
        .about("Transfer data faster than the speed of light")
        .author("Maxime L. <maxime.lalisse@gmail.com>")
        .arg(Arg::with_name("l").short("l")
             .required(false).takes_value(false).help("Listening"))
        .arg(Arg::with_name("address").help("Address to connect or listen from").required(true).index(1))
        .arg(Arg::with_name("port").help("Port to connect or listen from").required(true).index(2))
        .get_matches();
    
    let address = matches.value_of("address").unwrap();
    let port    = matches.value_of("port").unwrap();
    println!("Address: {}, Port: {}", address, port);

    if matches.is_present("l") == true
    {
        receiving_mode(address, port);
    }
    else
    {
        sending_mode(address, port);
    }

    Ok(())
}
