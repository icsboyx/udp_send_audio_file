use std::{fs::File, io::Read, net::UdpSocket};

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:54321").unwrap();
    let mut payload_file = File::open("audio.mp3").unwrap();
    let metadata = payload_file.metadata().unwrap();
    let file_size = metadata.len();
    println!("Sending total of {} bytes", file_size);
    let mut payload = [0; 1024];

    while let Ok(n) = payload_file.read(&mut payload[..1024]) {
        match n {
            0 => {
                println!("EOF");
                break;
            }
            _ => {
                udp_socket
                    .send_to(&payload[..n], "127.0.0.1:12345")
                    .unwrap();
            }
        }
    }
}
