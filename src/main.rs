#![allow(unused_imports)]
use std::{io::{Read, Write}, net::TcpListener};
pub mod util;
use util::kafka_constants;
use util::kafka_header_util;
use util::byte_util::{stream_input_to_bytes, create_response, send_response};
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
                let (api_key, api_version, correlation_id )= kafka_header_util::parse_header(&buf);
                let mut items: Vec<Box<dyn util::byte_util::ToBytes>> = Vec::new();
                items.push(Box::new(correlation_id));
                
                if api_key == kafka_constants::KAFKA_API_VERSIONS_KEY{
                    let error_code:i16 = if api_version == 4{kafka_constants::NO_ERROR} else{kafka_constants::UNSUPPORTED_API_VERSION_ERROR_CODE};
                    let api_versions_body = 
                    kafka_response_util::create_api_version_response( error_code,
                         kafka_constants::KAFKA_API_VERSIONS_KEY,
                         1, 4, 500);
                         items.push(Box::new(api_versions_body));
                    send_response(&mut stream, &create_response(items));
                }
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
