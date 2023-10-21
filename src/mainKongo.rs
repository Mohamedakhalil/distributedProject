use std::net::UdpSocket;
use std::str;
use base64;

fn main() {
    // Create a socket
    let socket = UdpSocket::bind("192.168.43.69:7878").expect("Could not bind socket");

    // Create a buffer to store the data
    let mut buf = [0; 65000];

    // Loop to listen for clients
    loop {
        // Receive data from the client
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Could not receive data");

        // Convert the received data to a string
        let received = str::from_utf8(&buf[0..number_of_bytes]).expect("Could not convert to string");

        // Print the data received
        print!("{}", received);

        // Decode the Base64 data
        let decoded_data = base64::decode(received).expect("Failed to decode Base64 data");
        //store the image recieved locally
        let file_name = "received_image.png";
        let mut file = std::fs::File::create(file_name).expect("Failed to create file");
        std::io::Write::write_all(&mut file, &decoded_data).expect("Failed to write to file");
        // Handle the decoded data (e.g., save it to a file or process it)
        // For example, you can save the image data to a file.
        // Note: You may need to handle image chunks and reconstruct the complete image on the server side.

        // Send a response to the client (if needed)
        let response = b"Received your image data";
        socket.send_to(response, &src_addr).expect("Could not send data");

        // Optionally, you can add a delay here before listening for the next chunk
        //std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
