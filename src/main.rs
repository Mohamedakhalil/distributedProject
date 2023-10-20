use std::io::{Read, Write};
use std::net::UdpSocket;
use base64;
use std::fs;

const MAX_PACKET_SIZE: usize = 60000; // Maximum UDP payload size

fn main() {
    let socket = UdpSocket::bind("192.168.43.237:7878").expect("Failed to bind socket"); // Bind to any available local port
    let server_address = "192.168.43.69:7878";
    //let request = "GET / HTTP/1.1\r\nHost: 192.168.43.69:7878\r\n\r\n";
    let image_path = "/home/khalil/Pictures/test.png";
    let image_data = fs::read(image_path).expect("Failed to read image file");

    // Encode image data to base64
    let encoded_data = base64::encode(&image_data);

    let chunks = encoded_data.as_bytes().chunks(MAX_PACKET_SIZE);
    let mut counter = 1;
    // Send each chunk over UDP
    for chunk in chunks {
        println!("{}",counter);
        counter = counter+1;
        socket.send_to(chunk, server_address).expect("Failed to send data");
    }
    let mut response = [0; 1024];
    let (size, _) = socket.recv_from(&mut response).expect("Failed to receive response");
    let response = String::from_utf8_lossy(&response[..size]);
    println!("Response:\n{}", response);
    
    
}