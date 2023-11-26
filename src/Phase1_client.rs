use std::net::{UdpSocket, SocketAddr};
use std::{thread, default};
use std::str;   
use std::time::Duration;
use serde::{Serialize, Deserialize};
use base64;
use std::fs;

const MAX_PACKET_SIZE: usize = 60000; // Maximum UDP payload size

// Define your custom struct
#[derive(Serialize, Deserialize)]
struct ImageWithIP {
    id: usize,
    image_data: Option<Vec<u8>>,
}

fn main() {
    // Specify the server's IP addresses and ports
    let server_addrs = vec![
        "192.168.43.69:8081",   //Kongo IP
        "192.168.43.237:8081",  //Khalil IP
       "192.168.43.159:8081",     //Khalil2 IP
    ];

    let num_clients = 3;
    let num_requests_per_client = 100;

    let mut handles = vec![];

    // Spawn a thread for each client
    for i in 1..=num_clients {
        let server_addrs_clone = server_addrs.clone();
        let client_thread_id = i;

        // Spawn a new thread for each client
        let handle = thread::spawn(move || {
            let client_socket = UdpSocket::bind("192.168.43.95:0").expect("Failed to bind client socket");
            client_socket.set_read_timeout(Some(Duration::new(10, 0))).expect("Failed to set read timeout");
            
            // Load your image data from a file
            let image_path = "download.jpeg";
            let image_data = fs::read(image_path).expect("Failed to read image file");
            for request_number in 0..num_requests_per_client {
            // Encode the image data to base64
            let mut encoded_data = base64::encode(&image_data);
            let mut encoded_data_clone = encoded_data.clone();
            // for i in image_data.iter() {
            //     print!("{}",i);
            // }
            //println!("Image data: {}",image_data);
            //println!("encoded data: {}",encoded_data);

    
            // Serialize the struct to JSON
            let json_data = serde_json::to_string(&encoded_data).expect("Failed to serialize to JSON");
            //print image data
            //println!("Image data: {}", image_data);
    
            // Split the JSON data into chunks
            let chunks: Vec<&[u8]> = json_data.as_bytes().chunks(MAX_PACKET_SIZE).collect();
            // Send a handshake request to the server
            //let chunks_clone =  chunks.clone();
            // for chunk in chunks_clone{
            //     for sh in chunk{
            //    // print!("{}",sh);
            //     }
            // }
            let handshake_request: String = "ENCRYPT,".to_string() + &chunks.len().to_string();
            for server_addr_str in &server_addrs_clone {
                let server_addr: SocketAddr = server_addr_str.parse().expect("Invalid server address");
                client_socket.send_to(handshake_request.as_bytes(), server_addr).expect("Failed to send handshake request");
                thread::sleep(Duration::new(1, 0));


                // Receive handshake confirmation from the server
                let mut buffer = [0; 6500];
                match client_socket.recv_from(&mut buffer) {
                    Ok((size, addressssss)) => {
                        let handshake_confirmation = String::from_utf8_lossy(&buffer[..size]);
                        println!("Client {} received handshake confirmation: {}, from server address: {}", client_thread_id, handshake_confirmation,addressssss);

                        // Check if the server confirmed
                        if handshake_confirmation.trim() == "YES" {

                            // Send the image data to the server
                            // for (i, chunk) in chunks.iter().enumerate() {
                            //     let image_with_id = ImageWithIP {
                            //         id: i,
                            //         image_data: Some(chunk.to_vec()),};
                            //         let json_data = serde_json::to_string(&image_with_id).expect("Failed to serialize to JSON");
                            //         //print id 
                            //         //println!("{}",image_with_id.id);

                            //     client_socket.send_to(chunk, addressssss).expect("Failed to send image data");
                            //     //thread::sleep(Duration::new(1, 0));
                            // }
                            encoded_data_clone = encoded_data.clone();
                            let default_image_with_ip = ImageWithIP {   
                                id: request_number,
                                image_data: Some(encoded_data_clone.into_bytes()),
                            };
                            let default_json_data = serde_json::to_string(&default_image_with_ip).expect("Failed to serialize to JSON");
                            let default_chunks: Vec<&[u8]> = default_json_data.as_bytes().chunks(MAX_PACKET_SIZE).collect();
                            for chunk in &default_chunks {
                                client_socket.send_to(chunk, addressssss).expect("Failed to send image data");
                                //thread::sleep(Duration::new(1, 0));
                            }
                            // Receive the image data from the server
                            let mut response_buffer= Vec::new();
                            loop{
                                let mut chunk = vec![0;MAX_PACKET_SIZE];
                                let(bytes_recieved,_)=client_socket.recv_from(&mut chunk).expect("Failed to recieve data");
                                response_buffer.extend_from_slice(&chunk[0..bytes_recieved]);

                                if bytes_recieved<MAX_PACKET_SIZE{
                                    break;
                                }

                            }
                            let filename = format!(
                                "client{}_request{}_encoded_image.jpeg",
                                client_thread_id, request_number
                            );
                            let response = str::from_utf8(&response_buffer).expect("Failed to convert to string");
                            let response_image:ImageWithIP = serde_json::from_str(&response).expect("Failed to convert to json");
                            if let Some(image_data) = response_image.image_data
                            {
                                let decoded_image_data = base64::decode(&image_data).expect("Failed to decode image data");

                                std::fs::write(&filename, decoded_image_data).expect("Failed to write image file");
                                println!("Image saved to {}", filename);

                            }
                            else {
                                println!("No image data received");}

                        } else {
                            println!("Handshake failed for Client {}", client_thread_id);
                        }
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::WouldBlock {
                            // Timeout occurred
                            println!("Timeout occurred for Client {}", client_thread_id);
                        } else {
                            // Handle other errors
                            eprintln!("Client {} error receiving handshake confirmation: {:?}", client_thread_id, e);
                        }
                    }
                  }
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
