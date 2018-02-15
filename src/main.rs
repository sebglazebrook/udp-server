extern crate ctrlc;

use std::net::UdpSocket;
use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

struct Server {
    running: Arc<AtomicBool>,
    inbound_queue: Vec<Vec<u8>>,
}

impl Server {

    pub fn new() -> Self {
        Server {
			running: Arc::new(AtomicBool::new(false)),
            inbound_queue: vec![],
		}
    }

    pub fn start(&mut self) {
        self.prepare_server();
        self.start_listening();
        self.close_server();
    }

    fn prepare_server(&self) {
	    println!("Starting server...");
		let running = self.running.clone();
		ctrlc::set_handler(move || {
            running.store(false, Ordering::SeqCst); 
        });
	    self.running.store(true, Ordering::SeqCst);
        // TODO start a thread pool to drain messages from inbound_queue
    }

    fn start_listening(&mut self) -> Result<(), Error> {
        let mut socket = try!(UdpSocket::bind("0.0.0.0:34254"));
        while self.running.load(Ordering::SeqCst) {
            // read from the socket
            let mut buf = [0; 10];
            let (amt, src) = try!(socket.recv_from(&mut buf)); // TODO this is block and means ctrl-c doesn't work
            self.inbound_queue.push(buf.to_vec());
            // send a reply to the socket we received data from
            let buf = &mut buf[..amt];
            try!(socket.send_to(buf, &src)); // TODO return some type of response code maybe??
        }
        Ok(())
    }

    fn close_server(&self) {
        // TODO close off any threads and make sure inbound_queue is empty
	    println!("Server closed.");
    }

}


fn main() {
    Server::new().start();
}


// Server takes messages and immediately puts them on a queue, returning success to the sender
