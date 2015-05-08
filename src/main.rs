extern crate weather;

use weather::*;
//use weather::get_current_conditions;

fn main() {
/*	match get_current_conditions() {
		Ok(z) => println!("The Weather for:\t {}\n\
				    Current Conditions:\t {}\n\
				    Temperature:\t\t {}F\n\
				    Wind:\t\t\t {}\n\
				    ", z.city, z.current_conditions, z.current_temp, z.wind_string),
		Err(e) => println!("{:?}",e)
	};*/

	let mut weat = WeatherStruct::new();
	weat.geo_lookup();
	weat.update_weather_data();

	weat.display_data();
	


}
