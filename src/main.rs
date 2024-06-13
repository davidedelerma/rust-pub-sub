use std::{thread, time::Duration};

use publisher::{listener, publisher, Packet};

mod publisher;

fn main() {
    let packet: Packet = Packet {
        sens_id: 1,
        loc_id: 1,
        tstamp: 1234,
        temp: 13,
    };
    let listener_thread = thread::spawn(|| {
        listener()
    });
    // Give the listener some time to start
    thread::sleep(Duration::from_secs(1));
    let writer_thread = thread::spawn(|| {
        match publisher(packet) {
            Ok(()) => println!("packet sent succesfully."),
            Err(e) => eprintln!("Error in sending packet: {}", e),
        }
    });
    listener_thread.join().expect("Listener thread panicked");
    writer_thread.join().expect("Listener thread panicked");
}
