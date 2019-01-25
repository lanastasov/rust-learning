use std::io;
// Q3. Find number of digits in a number

fn main() {

	// first solution
	let mut st=String::new();
	io::stdin().read_line(&mut st).expect("Fail");
	//let n:i32=st.trim().parse().expect("Fail");
	println!("{}", st.trim().to_string().chars().count() );	

	// second solution
	let mut n=432;
	let mut c=0;

	while n!=0 {
		c+=1;
		n=n/10;
	}
	println!("{}",c );
}
