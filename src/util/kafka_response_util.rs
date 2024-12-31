use std::{io::Write, net::TcpStream};
use crate::util::byte_util::{create_response, ToBytes};
pub fn create_api_version_response(error_code: i16, api_key: i16, min_version: i16, max_version: i16) -> Vec<Box<dyn ToBytes>>{
    let mut items: Vec<Box<dyn ToBytes>> = Vec::new();
    items.push(Box::new(error_code));
    items.push(Box::new(api_key));
    items.push(Box::new(min_version));
    items.push(Box::new(max_version));
    items
}
