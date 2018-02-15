extern crate ctrlc;

use std::net::UdpSocket;
use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

struct Server {
    running: Arc<AtomicBool>,
}

impl Server {

    pub fn new() -> Self {
        Server {
			running: Arc::new(AtomicBool::new(false)),
		}
    }

    pub fn start(&mut self) {
	    println!("Starting server...");
		let running = self.running.clone();
		ctrlc::set_handler(move || {
            running.store(false, Ordering::SeqCst); 
        });
	    self.running.store(true, Ordering::SeqCst);
        self.start_listening();
		println!("Server closed.");
    }

    fn start_listening(&mut self) -> Result<(), Error> {
        while self.running.load(Ordering::SeqCst) {
            let mut socket = try!(UdpSocket::bind("0.0.0.0:34254"));
            // read from the socket
            let mut buf = [0; 10];
            let (amt, src) = try!(socket.recv_from(&mut buf));

            // send a reply to the socket we received data from
            let buf = &mut buf[..amt];
            let seb = String::from_utf8(buf.to_vec()).unwrap();
            println!("{}", seb);
            try!(socket.send_to(buf, &src));
        }
        Ok(())
    }

}


fn main() {
    Server::new().start();
}


// Server takes messages and immediately puts them on a queue, returning success to the sender
