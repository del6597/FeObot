use std::io::BufferedReader;
use std::io::BufferedWriter;
use std::io::TcpStream;

struct IrcStream {
    inputstream: BufferedReader<TcpStream>,
    outputstream: BufferedWriter<TcpStream>,
    socket: TcpStream
}

impl IrcStream {
    pub fn new(host: &str, port: u16) -> IrcStream {
        let mut socket = TcpStream::connect((host, port)).unwrap();
        let mut reader = BufferedReader::new(socket.clone());
        let mut writer = BufferedWriter::new(socket.clone());        
        let mut stream = IrcStream{ inputstream: reader, outputstream: writer, socket: socket };        
        return stream;
    }
    
    pub fn read(&mut self) {
        loop {
            let next = self.inputstream.read_line();
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
                self.write(pong.deref());
            }
        }
    }
    
    pub fn write(&mut self, line: &str) {
        print!("Writing: {}", line);
        self.outputstream.write_line(line);
        self.outputstream.flush();
    }
    
}