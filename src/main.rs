use std::io;
use std::net::{TcpListener, TcpStream};

extern crate clap;
use clap::{Arg, App, SubCommand};

fn handle_client(stream: TcpStream) {
    // ...
}

fn main() -> io::Result<()> {
    let matches = App::new("ftlcat")
        .version("0.1")
        .about("Transfer data faster than the speed of light")
        .args_from_usage("-l 'Listening mode'
            <INPUT>              'Sets the input file to use'
            -v...                'Sets the level of verbosity'")
        .subcommand(SubCommand::with_name("test")
                    .about("controls testing features")
                    .version("1.3")
                    .author("Someone E. <someone_else@other.com>")
                    .arg_from_usage("-d, --debug 'Print debug information'"))
        .get_matches();

    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
