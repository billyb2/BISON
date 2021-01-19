use std::env;
mod udp;

fn main(){
	let args: Vec<String> = env::args().collect();
	
	if args.len() <= 1 {
		println!("Need more than one arguments!!");
		std::process::exit(1);
	} else if args.len() == 2 {
		println!("No attack type specified!");
		std::process::exit(1);
	} else if args.len() == 3 {
		let mut attack_type = "";
		
		if args[2] == "--flood" || args[2] == "-f" {
			attack_type = "flood";
		}
		
		println!("No {} type specified!", attack_type);
		std::process::exit(1);
	} else if args.len() == 4 {
		println!("No ip address specified!");
		std::process::exit(1);
	}
	
	if args[1] == "udp" {
		if args[2] == "--flood" || args[2] == "-f" {
			if args[3] == "--max" || args[3] == "-m" {
				udp::flood::max_packets(&args[4], args[5].parse::<u16>().unwrap());
				
			} else if args[3] == "--random" || args[3] == "-r" {
				udp::flood::rand_packets(&args[4], args[5].parse::<u16>().unwrap());
				
			} else if args[3] == "--null" || args[3] == "-n" { 
				udp::flood::null_packets(&args[4]);
			} else {
				println!("Invalid UDP flood attack type!");
			}
			
			
		} else {
			println!("Invalid UDP attack type!");
		}
	} else {
		println!("Invalid attack type!");
	}
}