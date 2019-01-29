fn main() {
	let mut a=[22,34,65,3];
	let b:[i32;4]=[1,2,3,4];
	let c:[i32;5]=[0;5];
	a[0]=1;
	println!("{:?}",a );
	println!("{:?}",b );
	println!("{:?}",c );
	print(c);
}

fn print(x:[i32;5]) {
	for n in 0 .. 5 {
		println!("{}",x[n]);
	}
}

fn print2(x:[i32;5]) {
	println!("{}",x.len());
	for n in x.iter() {
		println!("{}",n );
	}
}
