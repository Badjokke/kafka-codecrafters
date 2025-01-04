use core::panic;
use std::{io::{Read, Write, ErrorKind}, mem, net::TcpStream, ptr::null};

pub trait ToBytes{
    fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for i32{
    fn to_bytes(&self) -> Vec<u8> {
        return self.to_be_bytes().to_vec();
    }
}
impl ToBytes for i16{
    fn to_bytes(&self) -> Vec<u8> {
        return self.to_be_bytes().to_vec();
    }
}
impl ToBytes for u8{
    fn to_bytes(&self) -> Vec<u8> {
        return self.to_be_bytes().to_vec();       
    }
}

impl ToBytes for String{
    fn to_bytes(&self) -> Vec<u8> {
        self.clone().into_bytes()
    }
}


pub fn stream_input_to_bytes(stream: &mut TcpStream) -> Option<Vec<u8>>{
    match get_message_size(stream){
        Some(bytes) => read_stream_to_bytes(stream, bytes as usize),
        None => None
    }
}

pub fn send_response(stream:&mut TcpStream, buf:Vec<u8>){
    println!("Sending response: {:?}", buf);
    let result = stream.write(&buf);
    match result{
        Ok(bytes_send) => {
            println!("{bytes_send} bytes send");
            let _flush_result = stream.flush();
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
pub fn create_response(values:Vec<Box<dyn ToBytes>>) -> Vec<u8>{
    let mut bytes:Vec<u8> = values.iter().flat_map(|item| item.to_bytes()).collect();
    let mut response: Vec<u8>  =Vec::new();
    let msg_size = bytes.len() as i32;
    response.append(&mut msg_size.to_be_bytes().to_vec());
    response.append(&mut bytes);
    response
}

fn get_message_size(stream: &mut TcpStream) -> Option<i32>{
    let mut buf: [u8;4] = [0;4];
    let result = stream.read(&mut buf);
    match result{
        Ok(_bytes) => Some(i32::from_be_bytes(buf)),
        Err(e) if e.kind() == ErrorKind::ConnectionAborted => {
            None
        }
        Err(e) =>{
            println!("Error: {:?}", e);
            panic!("i am panicking!");
        }
    }
}

fn read_stream_to_string(stream: &mut TcpStream, bytes: usize)->Option<String>{
    let mut buf:Vec<u8> = vec![0;bytes];
    let result = stream.read(&mut buf[..]);
    match result {
        Ok(bytes_retrieved)=>{
            assert_eq!(bytes, bytes_retrieved);
           Some(u8_to_string(buf))
        },
        Err(e) => {
            println!("{:?}",e);
            None 
        }
    }
}

fn read_stream_to_bytes(stream: &mut TcpStream, bytes: usize) -> Option<Vec<u8>>{
    let mut buf: Vec<u8> = vec![0; bytes];
    let result = stream.read(&mut buf[..]);
    match result {
        Ok(bytes_read) =>{
            assert_eq!(bytes, bytes_read);
            Some(buf)
        },
        Err(e) =>{
            println!("{:?}", e);
            None
        }
    }
}

fn u8_to_string(vec_in:Vec<u8>) -> String{
    String::from_utf8(vec_in).expect("Failed to convert from utf8")
}