use std::{error, io::Write, net::TcpStream};
use crate::util::byte_util::{create_response, ToBytes};

pub struct ApiVersionsResponse{
    error_code: i16,
    api_key: i16,
    min_version:i16,
    max_version: i16,
    throttle_time: i32
}
impl ToBytes for ApiVersionsResponse{
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes:Vec<u8> = Vec::new();
        //error_code
        bytes.extend(&self.error_code.to_be_bytes());
        //api version count
        bytes.extend(&(1 as i32).to_be_bytes());
        //api version "struct"
        bytes.extend(&self.api_key.to_be_bytes());
        bytes.extend(&self.min_version.to_be_bytes());
        bytes.extend(&self.max_version.to_be_bytes());
        bytes.extend(&(0 as u8).to_be_bytes());
        //throttle
        bytes.extend(&self.throttle_time.to_be_bytes());
        //empty tag buffer
        bytes.extend(&(0 as u8).to_be_bytes()); 
        bytes
    }
}

pub fn create_api_version_response(error_code: i16, api_key: i16, min_version: i16, max_version: i16, throttle_time: i32) -> ApiVersionsResponse{
   ApiVersionsResponse{error_code,api_key, min_version, max_version, throttle_time}  
}
