use std::io;
// Q2. Create Basic Calculator which support +,-,/,*.
fn main() {
	let mut o=String::new();
	let a=10;
	let b=20;
	println!("Choose operation to be performed");
	println!("1. +\n2. -\n3. *\n4. /");
	io::stdin().read_line(&mut o).expect("Failed");
	let o:char=o.trim().parse().expect("Failed");
	// o=o.trim().to_string();
	// o=="+" ...etc
	
	if o=='+' {
		println!("{}",a+b);
	} else if o=='-' {
		println!("{}",a-b);
	} else if o=='*' {
		println!("{}}",a*b );
	} else if o=='/' {
		println!("{}",a/b );
	} else {
		println!("Wrong Choice");
	}
}
