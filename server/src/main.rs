use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::BufReader;
use std::path::Path;
use std::io::ErrorKind;
use std::fs::File;
extern crate time;

struct Request{
    request_file: String,
    request_time: String,
    stream: TcpStream,
}

impl Request{
    fn new(&mut self, stream: TcpStream, time: String) -> Self{
        // get path of requested file
        self.request_time = time;
        let mut request_header = Strng::new();
        let mut stream_reader = BufReader::new(stream);
        let mut request_parameters = Vec<&str>::new();
        match stream_reader.read_line(&mut request_header).unwrap() > 0{
            true => {
                request_parameters = request_header.split_whitespace().collect();
            }
            false => {
                println!("Server can not parse request!");
            }
        }
        self.request_file = request_parameters[1].to_string();
    }


}
struct Handler {
    request: Request,
    response: Response,
}
impl Handler {
    fn new(&mut self, req: Request) -> Self{
        self.request = req;
        self.response = Response::new();
    }

    fn parse_path(&mut self){
        match self.request.request_file.ends_with("/"){
            true => self.parse_file(),
            false => self.parse_dir(),
        }
    }

    fn parse_file(&mut self){
        let path = Path::new(&self.request.request_file);
        let mut f = try!(File::open(path));
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => self.set_response(200, s),
            Err(e) => {
                //match e.kind()
            },
        }
    }

    fn parse_dir(&mut self) {
        unimplemented!()
    }
    fn get_response(&mut self) -> Response{
        unimplemented!()
    }
}
struct Response {
    status_code: usize,
    status_info: String,
    file_content: Option<String>,
}
fn handle_client(stream: TcpStream){
    // Record current time for log file
    let current_time = time::now().ctime().to_string();
    let mut req = Request::new(stream,current_time);
    let mut handler = Handler::new(req);
    let mut res = handler.get_response();

}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server started to accept request!");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                handle_client(stream)
            });
            }
            Err(e) => {println!("Stream error: {:?}", e);}
        }
    }
    drop(listener);
}
