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
    
    fn connect(self) {
        //spawn(proc() { self.read() });
        let mut writer = self.outputstream;
        let _ = writer.write(b"NICK RustBucket\nUSER rusty 8 * :Rust Bucket\n");
    }
    
    fn read(self) {
        let mut reader = self.inputstream;
        let mut writer = self.outputstream;
        loop {
            let next = reader.read_line();
            if next.is_err() {
                break;
            }
            let text = next.unwrap();
            print!("{}", text);
            if text.starts_with("ERROR") {
                break;
            }
            if text.starts_with("PING") {
                let pong = text.replace("PING ", "PONG ");
                let _ = writer.write(pong.as_bytes());
            }
        }
    }
}

fn main() {
    let connection = IrcConnection::new();
    connection.connect();
    connection.read();
}