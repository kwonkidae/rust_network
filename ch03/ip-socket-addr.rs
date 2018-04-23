#![feature(ip)]
#![feature(lookup_host)]

use std::net::{IpAddr, SocketAddr};
use std::env;
use std::net::lookup_host;


fn main() {
	let args: Vec<_> = env::args().collect();
	println!("{:?}", args);
	if args.len() != 2 {
		eprintln!("Please provide only one host name");
		std::process::exit(1);
	} else {
		let addresses = lookup_host(&args[1]).unwrap();
		for address in addresses {
			println!("{}", address.ip());
		}
	}


	let local: IpAddr = "127.0.0.1".parse().unwrap();
	assert!(local.is_loopback());

	let global: IpAddr = IpAddr::from([0,0, 0x1c9, 0, 0, 0xafc8, 0, 0x1]);
	println!("{}", global.is_global());

	let local_sa: SocketAddr = "127.0.0.1:80".parse().unwrap();
	println!("{}", local_sa.is_ipv4());
	println!("{}", local_sa);

	let global_sa = SocketAddr::new(global, 80u16);
	println!("{}", global_sa);
	assert!(global_sa.is_ipv6());
}