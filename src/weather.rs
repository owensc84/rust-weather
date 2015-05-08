#![feature(plugin)]
#![feature(convert)]
#![feature(collections)]

extern crate hyper;
//extern crate regex;
extern crate rustc_serialize;

use rustc_serialize::json::Json;
use std::io::Read;
use hyper::Client;

#[derive(Debug)]
pub enum Error { RegexError(String), HTTPGetError(String), JsonError(String) }

#[derive(Debug, Clone, PartialEq)]
pub struct WeatherStruct {
  city: String,
  current_temp:  String,
  current_conditions: String,
  wind: String,
  zip: Option<String>,
  json_string: String,
  json_obj: Option<Json>,
}

impl WeatherStruct {
	pub fn new() -> WeatherStruct {
		WeatherStruct {
			city: String::from_str(""),
			current_temp: String::from_str(""),
			current_conditions: String::from_str(""),
			wind: String::from_str(""),
			zip: None,
			json_string: String::from_str(""),
			json_obj: None
		}
	}

	pub fn display_data(&self) {
		println!("City: {}", self.city);
		println!("Temp: {}", self.current_temp);
		println!("Cond: {}", self.current_conditions);
		println!("Wind: {}", self.wind);
	}

	pub fn update_weather_data(&mut self) -> Result<(), String> {
		try!(self.get_zip());

		try!(self.fetch_data_from_web());
		try!(self.parse_json_obj());

		Ok(())
	}

	/* 	look up zip code based on IP, sets the zipcode of the 
		WeatherStruct
		Returns the Zip code as a string or and Error
	*/
	pub fn geo_lookup(&mut self) -> Result<String, String> {
		try!(self.determine_zip_code());
		match self.get_zip() {
			Ok(z) => Ok(z),
			Err(e) => Err(format!("geo lookup error: {}", e))
		}
	}

	pub fn get_zip(&self) -> Result<String, String> {
		match self.zip.clone() {
			Some(z) => Ok(z),
			None => Err(format!("invalid zip code"))
		}
	}

	fn determine_zip_code(&mut self) -> Result<(), String> {
		let response = try!(http_get("http://ip-api.com/json"));
		let zip_json_obj = try!(Json::from_str(&response).map_err(|e| format!("zip json parse error: {}", e)));
		
		self.zip = match zip_json_obj.search("zip") {
			Some(z) => Some (
				format!("{}", z.as_string().ok_or("Zip object is not a string").unwrap())
			),
			None => return Err(format!("Couldnt find zip key"))
		};
		
		Ok(())
	}

	fn fetch_data_from_web(&mut self) -> Result<(), String> {
		let zip = try!(self.get_zip());
		let url = format!("http://api.wunderground.com/api/3d58504465810da1/conditions/q/{}.json", zip);
		self.json_string = try!(http_get(url.as_str()));
		self.json_obj = Some(try!(Json::from_str(&self.json_string).map_err(|e| format!("weather json parse error: {}", e))));
		Ok(())
	}

	fn parse_json_obj(&mut self) -> Result<(), String> {
		let tmp_json_obj = match self.json_obj.clone() {
			Some(o) => o,
			None => return Err(format!("json object is empty"))
		};

		self.city = match tmp_json_obj.search("full") {
			Some(c) => format!("{}", c.as_string().ok_or("city object is not a string").unwrap()),
			None => return Err(format!("could not find full key"))
		};

		self.current_temp = match tmp_json_obj.search("temp_f") {
			Some(t) => format!("{}", t),
			None => return Err(format!("could not find temp_f key"))
		};

		self.current_conditions = match tmp_json_obj.search("weather") {
			Some(w) => format!("{}", w.as_string().ok_or("weather object is not a string").unwrap()),
			None => return Err(format!("could not find weather key"))
		};

		self.wind = match tmp_json_obj.search("wind_string") {
			Some(w) => format!("{}", w.as_string().ok_or("wind object is not a string").unwrap()),
			None => return Err(format!("could not find wind_string key"))
		};

		Ok(())
	}

}



 fn http_get(url : &str) -> Result<String, String> {
   let mut client = Client::new();

   //match client.get(&*url).header(Connection(vec![ConnectionOption::Close])).send() {
   match client.get(&*url).send() {
    Ok(mut r) => {
	  	let mut body = String::new();
	  	r.read_to_string(&mut body).unwrap();
	  	return Ok(body)
	  },
	  Err(e)=> return Err(format!("{}", e))
   };
}

/*fn determine_zip_code() -> Result<String, Error>  {
	let url = format!("http://ip-api.com/json");

	let response = match http_get(url) {
	  Ok(s) => s,
	  Err(e) => return Err(e)
	};

  let zip_json = match Json::from_str(&response) {
    Ok(j) => j,
    Err(e) => return  Err(Error::JsonError(format!("{}", e)))
  };

  match zip_json.search("zip") {
    Some(zip) => return Ok(format!("{}", zip)),
    None => return Err(Error::JsonError(format!("couldn't find zip key")))
  };
}*/


#[test]
fn test_get_zip() {
   let mut test_struct = WeatherStruct::new();
   assert_eq!("55024", test_struct.geo_lookup().unwrap());
}
