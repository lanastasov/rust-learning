use std::io;

fn main() {
	let mut ch=String::new();
	println!("Are your friends coming ?");
	io::stdin().read_line(&mut ch).expect("Failed!");
	// trim
	ch=ch.trim().to_string();
	// trim
	if ch=="y" {
		println!("Yeah! Going for a Movie");
	} else {
		println!("Stay at home");
	}
}
