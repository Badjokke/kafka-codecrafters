use std::{io::{Read, Write}, mem, net::TcpStream, ptr::null};

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

pub fn stream_input_to_string(stream: &mut TcpStream) ->Option<String>{
    let payload_size: i32 = get_message_size(stream);
    read_stream_to_string(stream, payload_size as usize)
}

pub fn stream_input_to_bytes(stream: &mut TcpStream) -> Option<Vec<u8>>{
    let payload_size: i32 = get_message_size(stream);
    println!("payload size: {payload_size}");
    if payload_size == -1{
        return None;
    }
    read_stream_to_bytes(stream,payload_size as usize)
}

pub fn send_response(stream:&mut TcpStream, buf:&Vec<u8>){
    print!("Sending response: {:?}", buf);
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
    values.iter().flat_map(|item| item.to_bytes()).collect() 
}

fn get_message_size(stream: &mut TcpStream) -> i32{
    let mut buf: [u8;4] = [0;4];
    let result = stream.read(&mut buf);
    match result{
        Ok(_bytes) => i32::from_be_bytes(buf),
        Err(e) => {
            println!("{:?}",e);
            -1
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