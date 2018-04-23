fn main() {
	let add = |a, b| a + b;
	assert_eq!(add(2,3), 5);

	println!("{:?}", (1..10).filter(|x| x % 2 == 0).collect::<Vec<u32>>());	
}