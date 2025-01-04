#![allow(unused_imports)]
use std::{io::{Read, Write}, net::TcpListener};
pub mod util;
pub mod client;
use util::kafka_constants;
use util::kafka_header_util;
use util::byte_util::{stream_input_to_bytes, create_response, send_response};
use util::kafka_response_util;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
     for stream in listener.incoming() {
         match stream {
             Ok(mut stream) => {
                client::tcp_client::handle_client(&mut stream);    
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
     println!("main exit!");
}
