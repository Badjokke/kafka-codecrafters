#![allow(unused_imports)]
use std::{io::{Read, Write}, net::TcpListener};
pub mod util;
use util::kafka_constants;
use util::kafka_header_util;
use util::byte_util::stream_input_to_bytes;
use util::byte_util::create_response;
use util::kafka_response_util;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
     for stream in listener.incoming() {
         match stream {
             Ok(mut stream) => {
                let buf = match stream_input_to_bytes(&mut stream){
                    Some(buffer) => buffer,
                    None => panic!("No input!")
                };
                let (api_key, _api_version, correlation_id )= kafka_header_util::parse_header(&buf);
                let mut items: Vec<Box<dyn util::byte_util::ToBytes>> = Vec::new();
                items.push(Box::new(6 as i32));
                items.push(Box::new(correlation_id));
                items.push(Box::new(kafka_constants::UNSUPPORTED_API_VERSION_ERROR_CODE));
                
                if api_key == kafka_constants::KAFKA_API_VERSIONS_KEY{
                    kafka_response_util::send_response(&mut stream, &create_response(items));
                }
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
