use std::io::BufferedReader;
use std::io::BufferedWriter;
use std::io::TcpStream;

struct IrcConnection {
    inputstream: BufferedReader<TcpStream>,
    outputstream: BufferedWriter<TcpStream>,
}

impl IrcConnection {
    fn new() -> IrcConnection {
        let socket = TcpStream::connect("do1.stiny.org:6667").unwrap();
        let reader = BufferedReader::new(socket.clone());
        let writer = BufferedWriter::new(socket.clone());
        IrcConnection { inputstream:reader, outputstream:writer }
    }
}