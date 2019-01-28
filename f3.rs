fn main() {
	let sum=add(2,3);
	println!("{}",sum );
	println!("{}",add(3,4) );
}

fn add(a:i32,b:i32)-> i32 {
	// return can be omitted
	return a+b
}
