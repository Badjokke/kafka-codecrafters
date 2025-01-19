use core::error;
use std::net::TcpStream;
use uuid::uuid;
use crate::util::byte_util::{create_response, send_response, stream_input_to_bytes, ToBytes};
use crate::util::kafka_request_util::DescribeTopicPartitionsRequest;
use crate::{kafka_header_util, util};
use crate::kafka_constants;
use crate::kafka_response_util;
use crate::util::kafka_response_util::{ApiVersionsResponse, DescribeTopicPartitionsResponse, create_topic, create_partitions_topics_response};
use util::kafka_request_util;
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

fn describe_topic_partitions_response(req: DescribeTopicPartitionsRequest) -> DescribeTopicPartitionsResponse{
    println!("Sending response with topic name: {:?}", req.topics[0].name);
    let topic = create_topic(kafka_constants::UNKNOWN_TOPIC_OR_PARTITION, req.topics[0].name.clone(), uuid!("00000000-0000-0000-0000-000000000000"), true, Vec::new(),0x00000df8);
    create_partitions_topics_response(0, vec![topic], None)
}

fn create_kafka_response(buf: Vec<u8>, api_key: i16, error_code: i16) -> Option<Box<dyn ToBytes>>{
    match api_key{
        kafka_constants::KAFKA_API_VERSIONS_KEY => {
            Some(Box::new(kafka_response_util::create_api_version_response(error_code, 0)))
        },
        kafka_constants::KAFKA_DESCRIBE_TOPIC_PARTITIONS_KEY => {
            let thing = kafka_request_util::parse_describe_topics_request(buf);
            Some(Box::new(describe_topic_partitions_response(thing)))
        }
        _ => None
    }
}
fn handle_client_message(buf: Vec<u8>) -> Option<Vec<u8>>{
    println!("Handling client message: {:?}", buf); 
    let (api_key, api_version, correlation_id, client_id, header_offset)= kafka_header_util::parse_header(&buf);
    let error_code = get_kafka_error_code(api_key, api_version);
    if error_code == kafka_constants::UNSUPPORTED_API_VERSION_ERROR_CODE{
        println!("Unknown API version: {api_key}");
        return None;
    }
    let res = create_kafka_response(buf[header_offset..].to_vec(), api_key, error_code).expect("!!!");
    let mut items: Vec<Box<dyn ToBytes>> = Vec::new();
    items.push(Box::new(correlation_id));
    if api_key != kafka_constants::KAFKA_API_VERSIONS_KEY{
        items.push(Box::new(0 as u8));
    }

    items.push(res);
    Some(create_response(items))
}

fn get_kafka_error_code(api_key: i16, api_version: i16) -> i16{
    match api_key{
        kafka_constants::KAFKA_API_VERSIONS_KEY =>{
            if api_version < 3 || api_version > 18{
                return kafka_constants::UNSUPPORTED_API_VERSION_ERROR_CODE;
            }
            return kafka_constants::NO_ERROR;
        }
        kafka_constants::KAFKA_DESCRIBE_TOPIC_PARTITIONS_KEY => kafka_constants::NO_ERROR,
        _ => kafka_constants::UNSUPPORTED_API_VERSION_ERROR_CODE
    }
}
