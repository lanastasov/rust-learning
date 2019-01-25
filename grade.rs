use std::io;
/* Q3. Find Grade from percentage
	90% >= A Grade
	80% >= && <90% B
	70% >= && <80% C
	60% >= && <70% D

	else Fail
*/
fn main() {
	let mut per=String::new();
	println!("Enter your percentage");
	io::stdin().read_line(&mut per).expect("Fail");
	let per:i32=per.trim().parse().expect("Fail");
	if per>=90 {
		println!("A Grade");
	} else if per>=80 && per<90  {
		println!("B Grade");
	} else if per>=70 && per<80 {
		println!("C Grade");
	}else if per>=60 && per<70 {
		println!("D Grade");
	} else {
		println!("Fail");
	}
	
}
