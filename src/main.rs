extern crate weather;

use weather::helper::get_current_conditions;

fn main() {
	match get_current_conditions() {
		Ok(z) => println!("The Weather for: {}\nTemperature: {}F", z.city, z.current_temp),
		Err(e) => println!("{:?}",e)
	};


}
