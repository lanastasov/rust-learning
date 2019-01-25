use std::io;

fn main() {
	let mut ch=String::new();
	println!("Are your friends coming ?");
	io::stdin().read_line(&mut ch).expect("Failed!");
	if ch=="y" {
		println!("Yeah! Going for a Movie");
	} else {
		println!("Stay at home");
	}
}
