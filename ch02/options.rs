// #![feature(inclusive_range_syntax)]
// #![feature(trace_macros)]
// trace_macros!(true);
use std::thread;
fn divide(dividend: u32, divisor: u32) -> Option<u32> {
	if divisor == 0u32 {
		None
	} else {
		Some(dividend / divisor)
	}
}

macro_rules! factorial {
	($x:expr) => {
		{
			let mut result = 1;
			for i in 1..($x + 1) {
				result = result * i;
			}
			result
		}
	};
}

fn main() {
	let arg = std::env::args().nth(1).expect("Please provide only one argument");
	println!("{}", arg);
	println!("{:?}", factorial!(arg.parse::<u64>().expect("Could not parse to an integer")));
	let result1 = divide(100, 0);

	match result1 {
			Some(expr) => println!("The result is {}", expr),
			None => println!("Error occurred"),
	}

	let result2 = divide(100, 2);
	println!("{:?}", result2.unwrap());

	let s = String::from("Test");
	heap_example(&s);
	heap_example2(s);

	let mut mut_s = String::from("Test");
	mut_heap_example(&mut mut_s);

	let _ = parse_int("1".to_owned());
	// let _ = parse_int("abcd".to_owned());

	let numbers = 1..5;
	for number in numbers {
		println!("{}", number);
	}
	println!("--------------------");

	let two_tuple: TwoTuple<u32> = TwoTuple {
		first: 4u32,
		second: 2u32,
	};
	let three_tuple: ThreeTuple<u64> = ThreeTuple {
		first: 6u64,
		second: 5u64,
		third: 10u64,
	};

	println!("{}", two_tuple.max());
	println!("{}", three_tuple.max());
	// let inclusive = 1..=5;
	// for number in inclusive {
	// 	println!("{}", number);
	// }

	for i in 1..10 {
		let handle = thread::spawn(move || {
			println!("Hello from thread number {}", i);
		});
		let _ = handle.join();
	}

	let num: u32 = 42;
	let p: *const u32 = &num;

	unsafe {
		assert_eq!(*p, num);
	}
}

// ownership-borrow.rs
fn heap_example(input: &String) {
	let mystr = input;
	let _otherstr = mystr;
	println!("{}", mystr);
}

fn heap_example2(input: String) {
	let mystr = input;
	let _otherstr = mystr;
	println!("{}", _otherstr);
}

fn mut_heap_example(input: &mut String) {
	let mystr = input;
	let _otherstr = mystr;
	println!("{}", _otherstr);
}

fn parse_int(s: String) -> u64 {
	return s.parse::<u64>().expect("Could not parse as integer")
}

trait Max<T> {
	fn max(&self) -> T;
}

struct ThreeTuple<T> {
	first: T,
	second: T,
	third: T,
}

impl<T: PartialOrd + Copy> Max<T> for ThreeTuple<T> {
	fn max(&self) -> T {
		if self.first >= self.second && self.first >= self.third {
			self. first
		} else if self.second >= self.first && self.second >= self.third {
			self.second
		} else {
			self.third
		}
	}
}

struct TwoTuple<T> {
	first: T,
	second: T,
}

impl<T: PartialOrd + Copy> Max<T> for TwoTuple<T> {
	fn max(&self) -> T {
		if self.first >= self.second {
			self.first
		} else {
			self.second
		}
	}
}

