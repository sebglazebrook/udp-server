use std::net::UdpSocket;
use std::io::Error;

fn start_server() -> Result<(), Error> {
	{
		let mut socket = try!(UdpSocket::bind("0.0.0.0:34254"));

		// read from the socket
		let mut buf = [0; 10];
		let (amt, src) = try!(socket.recv_from(&mut buf));

		// send a reply to the socket we received data from
		let buf = &mut buf[..amt];
		buf.reverse();
		let seb = String::from_utf8(buf.to_vec()).unwrap();
		println!("{}", seb);
		try!(socket.send_to(buf, &src));
		Ok(())
	} 

}

struct Server;

impl Server {

    pub fn new() -> Self {
        Server {}
    }

    pub fn start(&self) {
	    println!("Starting server...");
	    start_server();
    }

}


fn main() {
    Server::new().start();
}


// Server takes messages and immediately puts them on a queue, returning success to the sender
