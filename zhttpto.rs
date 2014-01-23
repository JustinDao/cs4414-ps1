//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut visits:  int = 0;

fn main() 
{
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() 
    {
        // Spawn a task to handle the connection
        do spawn 
        {
            let mut stream = stream;
            
            match stream 
            {
                Some(ref mut s) => 
                {
                    match s.peer_name() 
                    {
                    Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                    None => ()
                    }
               },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));
            
            let mut arr = request_str.split('\n');
            let mut flag = false;
            let mut file_path = "";

            for line in arr
            {
                for word in line.split(' ')
                {
                    if flag
                    {
                        file_path = word.slice_from(1);
                        flag = false;
                    }

                    if word == "GET"
                    {
                        flag = true;
                    }
                }
            }

            let mut response: ~str;

            let str_length = file_path.len();

            if file_path != "" && file_path != "favicon.ico" && file_path.slice(str_length-5, str_length) == ".html"
            {
                let path = Path::new(file_path.clone());

                if !path.exists()
                {
                    response = 
                        ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                        <doctype !html><html><head><title>Hello, Rust!</title>
                        <style>body { background-color: #111; color: #FFEEAA }
                            h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                            h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                        </style></head>
                        <body>
                        <h1>File not found.</h1>
                        </body></html>\r\n";
                }
                else
                {
                    match (File::open(&path)) 
                    {
                        Some(mut msg) => 
                        {   
                            response = 
                                ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n"
                                + msg.read_to_str()
                                + "\r\n";
                        },
                        None => 
                        {
                            response = 
                                ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                                <doctype !html><html><head><title>Hello, Rust!</title>
                                <style>body { background-color: #111; color: #FFEEAA }
                                    h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                                    h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                                </style></head>
                                <body>
                                <h1>File not found.</h1>
                                </body></html>\r\n";
                        }
                    }
                }
            }

            else if file_path == ""
            {
                response = 
                    ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                     <doctype !html><html><head><title>Hello, Rust!</title>
                     <style>body { background-color: #111; color: #FFEEAA }
                            h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                            h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                     </style></head>
                     <body>
                     <h1>Greetings, Krusty!</h1>
                     </body></html>\r\n";
            }
            else
            {
                response =
                    ~"HTTP/1.1 403 ERROR\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                     <doctype !html><html><head><title>Hello, Rust!</title>
                     <style>body { background-color: #111; color: #FFEEAA }
                            h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                            h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                     </style></head>
                     <body>
                     <h1>ERROR</h1>
                     </body></html>\r\n";
            }
            

            stream.write(response.as_bytes());
            unsafe 
            {
                visits += 1;
                let val = visits;
                println!("Visits: {}", val);
            }
            println!("Connection terminates.");
            
        }
    }
}
