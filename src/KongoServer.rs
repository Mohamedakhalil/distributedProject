use std::net::{UdpSocket, SocketAddr};
use std::str;

fn main() {
    let local_addr = "192.168.43.69:7879";
    let other_server_addr = "192.168.43.95:7878";

    let self_addr: SocketAddr = local_addr.parse().expect("Invalid IP address or port");
    let other_server: SocketAddr = other_server_addr.parse().expect("Invalid IP address or port");

    let socket = UdpSocket::bind(self_addr).expect("Could not bind socket");

    let mut buf = [0; 65000];

    // Server 1 starts as a candidate with a unique ID (e.g., a number).
    let candidate_id = 1;
    let mut is_leader = false;
    let mut leader_id = 0;  // 0 indicates no leader

    loop {
        // Send a message with the candidate ID to the other server
        let message = format!("Candidate {} - Election", candidate_id);
        socket.send_to(message.as_bytes(), other_server).expect("Failed to send message");

        // Receive a message from the other server
        let (number_of_bytes, _) = socket.recv_from(&mut buf).expect("Could not receive data");
        let received = str::from_utf8(&buf[0..number_of_bytes]).expect("Could not convert to string");
        println!("Received: {}", received);

        // Parse the message to determine the other server's candidate ID
        let other_candidate_id: u32 = received
        .split(' ')
        .nth(1)
        .expect("Invalid message format")
        .trim() // Remove leading/trailing whitespace
        .parse()
        .expect("Invalid candidate ID");

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
