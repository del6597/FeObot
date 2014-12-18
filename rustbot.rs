mod irclib;
use std::io::BufferedReader;
use std::io::BufferedWriter;
use std::io::TcpStream;

fn main() {
    let mut irc = IrcStream::new("do1.stiny.org", 6667u16);
    irc.write("NICK RustBucket\nUSER rusty 8 * :Rust Bucket\n");
    irc.read();
}