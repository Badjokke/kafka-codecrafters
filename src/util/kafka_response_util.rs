use std::{error, io::Write, net::TcpStream};
use crate::util::byte_util::{create_response, ToBytes};
use crate::kafka_constants;
struct ApiKeys{
    api_key: i16,
    min_version: i16,
    max_version: i16,
    tag_buffer: Vec<u8>
}
pub struct ApiVersionsResponse{
    error_code: i16,
    api_keys: Vec<ApiKeys>,
    throttle_time: i32
}
pub struct DescribeTopicPartitionsResponse{
    throttle_time: i32,
}
impl ToBytes for ApiKeys{
    fn to_bytes(&self) -> Vec<u8> {    
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.api_key.to_be_bytes());
        bytes.extend(self.min_version.to_be_bytes());
        bytes.extend(self.max_version.to_be_bytes());
        bytes.append(&mut self.tag_buffer.clone());
        bytes
    }

}
impl ToBytes for ApiVersionsResponse{
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes:Vec<u8> = Vec::new();
        //error_code
        bytes.extend(&self.error_code.to_be_bytes());
        //api version count
        bytes.extend(&(self.api_keys.len() as u8 + 1).to_be_bytes());
        for i in 0..self.api_keys.len(){
            bytes.extend(self.api_keys[i].to_bytes());
        }
        //throttle
        bytes.extend(&self.throttle_time.to_be_bytes());
        //empty tag buffer
        bytes.extend(&(0 as u8).to_be_bytes()); 
        bytes
    }
}
fn create_api_versions_body() -> Vec<ApiKeys>{
    vec![
        ApiKeys{api_key: kafka_constants::KAFKA_API_VERSIONS_KEY, min_version: 0, max_version: 4, tag_buffer: vec![0]},
        ApiKeys{api_key: kafka_constants::KAFKA_DESCRIBE_TOPIC_PARTITIONS_KEY, min_version: 0, max_version:0, tag_buffer: vec![0]}
    ]
}
pub fn create_api_version_response(error_code: i16, throttle_time: i32) -> ApiVersionsResponse{
   ApiVersionsResponse{error_code,api_keys:create_api_versions_body() ,throttle_time}  
}
