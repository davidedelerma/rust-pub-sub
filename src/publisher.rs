use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

const SOCKET_ADDRESS: &str = "127.0.0.1:8000";
pub struct Packet {
    pub sens_id: u32,
    pub loc_id: u16,
    pub tstamp: u32,
    pub temp: i16,
}

impl Packet {
    pub fn encode(&self) -> [u8; 12] {
        let mut array: [u8; 12] = [0; 12];

        let sens_id_bytes = self.sens_id.to_be_bytes();
        let loc_id_byte: [u8; 2] = self.loc_id.to_be_bytes();
        let tstamp_byte: [u8; 4] = self.tstamp.to_be_bytes();
        let temp_byte: [u8; 2] = self.temp.to_be_bytes();

        array[0..4].copy_from_slice(&sens_id_bytes);
        array[4..6].copy_from_slice(&loc_id_byte);
        array[6..10].copy_from_slice(&tstamp_byte);
        array[10..12].copy_from_slice(&temp_byte);
        return array;
    }
}

pub fn decode(array: [u8; 12]) -> Packet {
    let sens_id: u32 = u32::from_be_bytes(array[0..4].try_into().unwrap());
    let loc_id: u16 = u16::from_be_bytes(array[4..6].try_into().unwrap());
    let tstamp: u32 = u32::from_be_bytes(array[6..10].try_into().unwrap());
    let temp: i16 = i16::from_be_bytes(array[10..12].try_into().unwrap());
    return Packet {
        sens_id,
        loc_id,
        tstamp,
        temp,
    };
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 12];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection was closed
                break;
            }
            Ok(n) => {
                if n == 12 {
                    let mut array = [0u8; 12];
                    array.copy_from_slice(&buffer[..n]);
                    let received = decode(array);
                    println!(
                        "Received: sensor id {}, loc id {}, timestamp{}, temperature{}",
                        received.sens_id, received.loc_id, received.tstamp, received.temp
                    );
                } else {
                    eprintln!("Unexpected packet size: {}", n);
                }
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                break;
            }
        }
    }
}

pub fn listener() {
    let listener = TcpListener::bind(SOCKET_ADDRESS).expect("Could not bind to Socket address");
    println!("listening to: {}", SOCKET_ADDRESS);
    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

pub fn publisher(packet: Packet) -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect(SOCKET_ADDRESS).expect("Couldn't connect to the server...");
    println!("conected to: {}", SOCKET_ADDRESS);
    let encoded_data = packet.encode();
    stream.write(&encoded_data)?;
    Ok(())
}
