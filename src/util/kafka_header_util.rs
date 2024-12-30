use crate::util::byte_util;

fn parse_correlation_id_v2(buffer:&Vec<u8>) -> i32{
   let byte_offset = 8;
   let correlation_id = i32::from_be_bytes(buffer[byte_offset..byte_offset+4].try_into().unwrap());
   correlation_id
}
fn parse_request_api_key(buffer:&Vec<u8>) -> i16{
    let byte_offset = 4;
    let request_api_key = i16::from_be_bytes(buffer[byte_offset..byte_offset+2].try_into().unwrap());
    request_api_key
}
fn parse_request_api_version(buffer:&Vec<u8>) -> i16{
    let byte_offset = 6;
    let request_api_version = i16::from_be_bytes(buffer[byte_offset..byte_offset+2].try_into().unwrap());
    request_api_version
}
pub fn parse_header(buffer:&Vec<u8>) -> (i16, i16, i32){
    println!("Parsing {:?}", buffer);
    let correlation_id = parse_correlation_id_v2(buffer);
    let request_api_key = parse_request_api_key(buffer);
    let request_api_version = parse_request_api_version(buffer);
    println!("Extracted {request_api_key} {request_api_version} {correlation_id}");
    (request_api_key, request_api_version, correlation_id)
}
