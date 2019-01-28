fn main() {
	fn fact(n:i32)-> i32 {
		if n == 0 {
			return 1;
		} else {
			return n*fact(n-1)
		}

	}	
	println!("fact(6) = {:?}", fact(6) );
	// Prints 720

//------------------------------

	fn fact2(n:i32)-> i32 {
		if n >= 1 {
			return n*fact2(n-1)
		}else {
			return 1
		}
	}

	println!("fact2(6) = {:?}", fact2(6) );
	// Prints 720

//------------------------------

	fn fact3(a:i32)->(String,i32) {
		let mut fact=1;
		for n in 1 .. a+1 {
			fact=fact*n;
		}
		(String::from("Factorial of 6:"), fact)
	}

	let x=fact3(6);
	println!("fact3(6) = {:?}",x);
}

