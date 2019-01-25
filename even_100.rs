// Q1. Write a program to print all even numbers between 1 to 100.
fn main() {
	for n in 1..101 {
		if n%2 == 0 {
			println!("{}",n );
		}
	}
}
