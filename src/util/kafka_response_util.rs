use std::{io::Write, net::TcpStream};

pub fn send_response(stream:&mut TcpStream, buf:&mut Vec<u8>){
    let mut response_body: Vec<u8> = buf.len().to_be_bytes().to_vec();
    response_body.append(buf);
    let result = stream.write(&response_body);
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