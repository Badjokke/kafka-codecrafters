use crate::util::byte_util;

fn parse_correlation_id_v2(buffer:&[u8]) -> i32{
   let byte_offset = 4;
   i32::from_be_bytes(buffer[byte_offset..byte_offset+4].try_into().unwrap())
}
fn parse_request_api_key(buffer:&[u8]) -> i16{
    let byte_offset = 0;
    i16::from_be_bytes(buffer[byte_offset..byte_offset+2].try_into().unwrap())
}
fn parse_request_api_version(buffer:&[u8]) -> i16{
    let byte_offset = 2;
    i16::from_be_bytes(buffer[byte_offset..byte_offset+2].try_into().unwrap())
}
fn parse_client_id(buffer:&[u8]) -> String {
    let byte_offset = 8;
    let client_id_size: usize = u16::from_be_bytes(buffer[byte_offset..byte_offset+2].try_into().unwrap()) as usize;
    let client_id_byte_offset = 10;
    String::from_utf8(buffer[client_id_byte_offset..client_id_byte_offset+client_id_size].try_into().unwrap()).expect("Failed to decode client id string!")
}
pub fn parse_header(buffer:&Vec<u8>) -> (i16, i16, i32, String, usize){
    println!("Parsing {:?}", buffer);
    let correlation_id = parse_correlation_id_v2(buffer);
    let request_api_key = parse_request_api_key(buffer);
    let request_api_version = parse_request_api_version(buffer);
    let client_id = parse_client_id(buffer);
    println!("Extracted {request_api_key} {request_api_version} {correlation_id} {client_id}");
    //expceting tag buffer to be always empty
    (request_api_key, request_api_version, correlation_id, parse_client_id(buffer), (10+client_id.len()+1))
}
