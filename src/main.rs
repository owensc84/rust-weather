
fn main() {
	
	let str = format!("abc123dddddddd");
	let c : Vec<char> = str.chars().collect();
	println!("{:?}", c);
}
