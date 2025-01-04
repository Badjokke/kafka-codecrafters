#![allow(unused_imports)]
use std::{io::{Read, Write}, net::TcpListener};
pub mod util;
pub mod client;
pub mod threads;
use util::kafka_constants;
use util::kafka_header_util;
use util::byte_util::{stream_input_to_bytes, create_response, send_response};
use util::kafka_response_util;

fn main() {
    let thread_pool = threads::thread_pool::ThreadPool::new(8);
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
     for stream in listener.incoming() {
         match stream {
             Ok(mut stream) => {
                thread_pool.execute(move||{client::tcp_client::handle_client(&mut stream)});    
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
