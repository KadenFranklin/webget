use std::{io, env};
use std::io::{Write,Read};
use std::fs::File;
use std::ffi::CString;
use nix::unistd::{fork, ForkResult, execvp};
use nix::sys::wait::waitpid;
use std::net::TcpStream;

struct Thisstruct {
    host: String,
    file: String,
    port: usize
}

impl Thisstruct{
    fn new(line: String) -> Self {
        Thisstruct{
            host: get_host(line.clone()),
            file: get_file(line.clone()),
            port: get_port(line.clone())
        }
    }
}

fn main() -> io::Result<()> {
    loop {
        let path = env::current_dir()?;
        println!("{}", path.display());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("not valid input.");
        if input.contains("webget") {
            let this = Thisstruct::new(input.split("webget ").collect());
            let host_port = this.host.clone() + ":" + this.port.to_string().as_str();
            let mut socket = TcpStream::connect(host_port)?;
            let message = format!
            ("GET {} HTTP/1.1
Host: {}
Connection: Close
", this.file, this.host);
            socket.write(message.as_bytes())?;
            let mut reply = String::new();
            socket.read_to_string(&mut reply)?;
            let new_filename = format!("{}.txt", this.file);
            let mut buffer = File::create(new_filename)?;
            println!("reply: {}",reply);
            if reply.trim() == "" {
                buffer.write_all(reply.as_bytes())?;
                println!("done!");
            }
        }
        else {
            match unsafe { fork() }.unwrap() {
                ForkResult::Parent { child } => {
                    waitpid(child, None).expect("incorrect input");
                }
                ForkResult::Child => {
                    let input = input.trim();
                    let c_input = CString::new(input).unwrap();
                    let externalized =  externalize(input);
                    execvp(c_input.as_c_str(), &externalized).unwrap();
                }
            }
        }
    }
}

fn get_host (s: String) -> String {
    let url: String = s.split("/").skip(2).take(1).collect();
    let mut this: String = url ;
    if this.contains(":"){
         this = this.split(":").take(1).collect();
        return this
    }
    return this;
}

fn get_file (s: String) -> String {
    let mut this: String ="".to_owned();
    for x in s.split("/").skip(3).into_iter() {
        this = this + "/" + &x;
    }
    this = this.trim().parse().unwrap();
    return this
}

fn get_port (s :String ) -> usize {
    let mut split_port: String = s.split(":").skip(2).collect();
    split_port = split_port.split("/").take(1).collect();
    for char in split_port.chars() {
        if char.is_alphabetic() {
            break
        }
        if char.is_numeric() {
            let this = split_port.parse::<usize>().unwrap();
            return this
        }
    }
    let split_http: String = s.split("http").collect();
    if split_http.starts_with("s") {
        let this = 443 as usize;
        return this
    }
    return 80 as usize
}

fn externalize(command: &str) -> Box<[CString]> {
    let converted = command.split_whitespace()
        .map(|s| CString::new(s).unwrap())
        .collect::<Vec<_>>();
    converted.into_boxed_slice()
}
