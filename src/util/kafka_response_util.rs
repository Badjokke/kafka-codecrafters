use std::{io::Write, net::TcpStream};

pub fn send_response(stream:&mut TcpStream, buf: &Vec<u8>){
    println!("Sending response: {:?}", buf);
    let result = stream.write(buf);
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