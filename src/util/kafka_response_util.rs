use std::{error, io::Write, net::TcpStream};
use crate::util::byte_util::{create_response, ToBytes};

pub struct ApiVersionsResponse{
    error_code: i16,
    api_key: i16,
    min_version:i16,
    max_version: i16,
    throttle_time: i32
}

impl ApiVersionsResponse{
    fn new(error_code: i16, api_key: i16, min_version: i16, max_version:i16, throttle_time: i32)->Self{
        ApiVersionsResponse{error_code, api_key, min_version,max_version, throttle_time}
    }
}

impl ToBytes for ApiVersionsResponse{
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes:Vec<u8> = Vec::new();
        let mut error_code_bytes = self.error_code.to_be_bytes().to_vec();
        let mut api_key_bytes = self.api_key.to_be_bytes().to_vec();
        let mut min_version_bytes = self.min_version.to_be_bytes().to_vec();
        let mut max_version_bytes = self.max_version.to_be_bytes().to_vec();
        let mut throttle_time_bytes = self.throttle_time.to_be_bytes().to_vec();
        
        bytes.append(&mut error_code_bytes);
        bytes.append(&mut api_key_bytes);
        bytes.append(&mut min_version_bytes);
        bytes.append(&mut max_version_bytes);
        bytes.append(&mut throttle_time_bytes);
        
        bytes
    }
}

pub fn create_api_version_response(error_code: i16, api_key: i16, min_version: i16, max_version: i16, throttle_time: i32) -> ApiVersionsResponse{
   ApiVersionsResponse{error_code: error_code,api_key: api_key, min_version: min_version,max_version: max_version, throttle_time: throttle_time}  
}
