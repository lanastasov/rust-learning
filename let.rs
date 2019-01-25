fn main() {
	let num= if 3>2 {
		println!("if block");
		()
	}else {
		println!("else");
		()
	};
	println!("{:?}", num );
}
