/* Q2. Create a quiz application. Ask a question from user and give 3
	chances to attempt the question. If he answer the question correctly
	then quit the application.
*/

use std::io;
fn main() {
	let mut ans=String::new();
	println!("Which is the longest river in world");
	for n in 1 .. 4 {
		ans.clear();
		io::stdin().read_line(&mut ans).expect("Fail");
		ans=ans.trim().to_string();
		if ans=="Nile" {
			println!("You guessed Correctly");
			break;
		}else {
			if n!=3 {
				println!("Try Again");
			}else {
				println!("You are Failed");
			}
		}

	}
}
