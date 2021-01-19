use std::net::UdpSocket;
use rand::random;

pub fn max_packets(dst: &str, length: u16){		
	loop {
		let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
		socket.connect(dst).expect("connect function failed");
		socket.send(&vec![255; length.into()]).expect("couldn't send message");
	}
	
}

pub fn rand_packets(dst: &str, length: u16){
	loop {
		let mut data: Vec<u8> = Vec::with_capacity(length.into());
		for _ in 0..data.capacity() {
			data.push(random());
		};
		
		let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
		socket.connect(dst).expect("connect function failed");
		socket.send(&data).expect("couldn't send message");
	}
	
	
}

pub fn null_packets(dst: &str){		
	loop {
		let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
		socket.connect(dst).expect("connect function failed");
		socket.send(&[]).expect("couldn't send message");
	}
	
}