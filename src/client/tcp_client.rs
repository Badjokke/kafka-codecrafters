use std::net::TcpStream;
use crate::util::byte_util::{create_response, send_response, stream_input_to_bytes, ToBytes};
use crate::kafka_header_util;
use crate::kafka_constants;
use crate::kafka_response_util;

pub fn handle_client(stream: &mut TcpStream){
    loop {
           match stream_input_to_bytes( stream){
            Some(message) =>{
                match handle_client_message(message){
                    Some(response) => send_response(stream, response),
                    None => println!("Send some default message")
                }
            },
            None => {
                println!("Socket closed!");
                break;
            }
            }  
    }
}

fn handle_client_message(buf: Vec<u8>) -> Option<Vec<u8>>{
    println!("Handling client message: {:?}", buf); 
    let (api_key, api_version, correlation_id )= kafka_header_util::parse_header(&buf);
    let mut items: Vec<Box<dyn ToBytes>> = Vec::new();
    items.push(Box::new(correlation_id));
    if api_key != kafka_constants::KAFKA_API_VERSIONS_KEY {
        println!("Unknown API key: {api_key}");
        return None;
    }
    let error_code = get_kafka_error_code(api_version);             
    let api_versions_body =kafka_response_util::create_api_version_response(
        error_code,
        kafka_constants::KAFKA_API_VERSIONS_KEY,
        1, 4, 0);
    items.push(Box::new(api_versions_body));
    return Some(create_response(items));
}

fn get_kafka_error_code(api_version: i16) -> i16{
    if api_version == kafka_constants::API_VERSIONS {kafka_constants::NO_ERROR} else {kafka_constants::UNSUPPORTED_API_VERSION_ERROR_CODE} 
}
