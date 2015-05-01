extern crate weather;

use weather::helper::determine_zip_code;

fn main() {
	println!("{}", determine_zip_code().unwrap());
}
