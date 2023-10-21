/////////////////////working code of server///////////////////////////////
// use std::net::{UdpSocket, SocketAddr};
// use std::str;
// use base64;

// fn main() {
//     // Create a socket to listen for incoming data
//     let socket = UdpSocket::bind("192.168.43.95:7878").expect("Could not bind socket");

//     // Create a socket to send data to the other server
//     let send_socket = UdpSocket::bind("192.168.43.95:7879").expect("Could not bind send socket");

//     // Create a buffer to store the data
//     let mut buf = [0; 65000];

//     // Define the IP address and port of the other server
//     let other_server_ip = "192.168.43.69:7878";
//     let other_server_addr: SocketAddr = other_server_ip.parse().expect("Invalid IP address or port");

//     // Loop to listen for clients
//     loop {
//         // Receive data from the client
//         let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Could not receive data");

//         // Convert the received data to a string
//         let received = str::from_utf8(&buf[0..number_of_bytes]).expect("Could not convert to string");

//         // Print the data received
//         println!("Received: {}", received);

//         // Decode the Base64 data (if it's base64 encoded)
//         //let decoded_data = base64::decode(received).expect("Failed to decode Base64 data");

//         // Optionally, you can process the decoded data as needed here

//         // Send a response to the client (if needed)
//        // let response = b"Received your data";
//         //socket.send_to(response, &src_addr).expect("Could not send data");

//         // Send a message to the other server
//         let message_to_other_server = b"Hello from the second server!";
//         send_socket.send_to(message_to_other_server, other_server_addr).expect("Failed to send message to the other server");

//         // Optionally, you can add a delay here before listening for the next chunk
//         //std::thread::sleep(std::time::Duration::from_secs(3));
//     }
// }

// use std::net::{UdpSocket, SocketAddr};
// use std::str;

// fn main() {
//     let local_addr = "192.168.43.95:7878";
//     let other_server_addr = "192.168.43.69:7879";

//     let self_addr: SocketAddr = local_addr.parse().expect("Invalid IP address or port");
//     let other_server: SocketAddr = other_server_addr.parse().expect("Invalid IP address or port");

//     let socket = UdpSocket::bind(self_addr).expect("Could not bind socket");

//     let mut buf = [0; 65000];

//     // Assume server 2 starts as a non-leader
//     let mut is_leader = false;

//     loop {
//         if is_leader {
//             // Server 2 sends a leader message
//             let message = b"I am the leader!";
//             socket.send_to(message, other_server).expect("Failed to send leader message");
//         } else {
//             // Server 2 sends a non-leader message
//             let message = b"Is anyone the leader?";
//             socket.send_to(message, other_server).expect("Failed to send non-leader message");
//         }

//         let (number_of_bytes, _) = socket.recv_from(&mut buf).expect("Could not receive data");
//         let received = str::from_utf8(&buf[0..number_of_bytes]).expect("Could not convert to string");
//         println!("Received: {}", received);

//         // Simulate leader election
//         // In a real scenario, you would implement the Chang and Roberts algorithm here.
//         // For simplicity, we'll toggle leader status in each iteration.
//         is_leader = !is_leader;
//     }
// }

use std::net::{UdpSocket, SocketAddr};
use std::str;

fn main() {
    let local_addr = "192.168.43.95:7878";
    let other_server_addr = "192.168.43.69:7879";

    let self_addr: SocketAddr = local_addr.parse().expect("Invalid IP address or port");
    let other_server: SocketAddr = other_server_addr.parse().expect("Invalid IP address or port");

    let socket = UdpSocket::bind(self_addr).expect("Could not bind socket");

    let mut buf = [0; 65000];

    // Server 2 starts as a candidate with a unique ID (e.g., a number).
    let candidate_id = 2;
    let mut is_leader = false;
    let mut leader_id = 0;  // 0 indicates no leader

    loop {
        // Send a message with the candidate ID to the other server
        let message:String = format!("Candidate {} - Election", candidate_id);
        socket.send_to(message.as_bytes(), other_server).expect("Failed to send message");

        // Receive a message from the other server
        let (number_of_bytes, _) = socket.recv_from(&mut buf).expect("Could not receive data");
        let received = str::from_utf8(&buf[0..number_of_bytes]).expect("Could not convert to string");
        println!("Received: {}", received);

        // Parse the message to determine the other server's candidate ID
        let other_candidate_id: u32 = match received
        .split(' ')
        .nth(1)
        .and_then(|s| s.trim_end_matches("Election").parse().ok()) {
        Some(id) => id,
        None => {
           // eprintln!("Receivedmmmmmmmmmmmmmmmmmmmmmm: {}", id);
            eprintln!("Received an invalid message format: {}", received);
            continue; // Skip processing this message
        }
        };


        if other_candidate_id < candidate_id {
            // The other server's candidate has a smaller ID, so it becomes the leader
            is_leader = false;
        } else if other_candidate_id > candidate_id {
            // The other server's candidate has a larger ID, so this server remains the leader
            is_leader = true;
        } else {
            // Both candidates have the same ID; compare IP addresses
            is_leader = self_addr.ip() < other_server.ip();
        }

        if is_leader && leader_id != candidate_id {
            // This server becomes the leader
            leader_id = candidate_id;
            println!("Server {} is the leader.", leader_id);
        }

        // Simulate a delay before starting a new round
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}