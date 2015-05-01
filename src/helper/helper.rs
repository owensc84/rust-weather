#![feature(plugin)]
#![feature(convert)]
//#![plugin(regex_macros)]

extern crate hyper;
extern crate regex;

use regex::Regex;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;
use hyper::header::ConnectionOption;


#[derive(Debug)]
pub enum Error { RegexError(String), HTTPGetError(String) }


 pub fn http_get(url : String) -> Result<String, Error> {
   let mut client = Client::new();

   match client.get(&*url).header(Connection(vec![ConnectionOption::Close])).send() {
	  Ok(mut r) => {
	  	let mut body = String::new();
	  	r.read_to_string(&mut body).unwrap();
	  	return Ok(body)
	  },
	  Err(e)=> return Err(Error::HTTPGetError(format!("{}", e)))
   };
}

fn get_weather_json(url: String) -> Result<String, Error> {
  let response = match http_get(url) {
    Ok(s) => return Ok(s),
    Err(e) => return Err(e)
  };
}

pub fn determine_zip_code() -> Result<String, Error>  {
	let url = format!("http://ip-api.com/json");

	let response = match http_get(url) {
	  Ok(s) => s,
	  Err(e) => return Err(e)
	};

	let re = match Regex::new(r"\W\d{5}") {
	   Ok(r) => r,
	   Err(e) => return Err(Error::RegexError(format!("{}", e)))
	};

	match re.captures(response.as_str()) {
	   Some(cap) => {
	   	  let mut zip_code = format!("{}", cap.at(0).unwrap());
	   	  zip_code.remove(0); // remove the leading "
	   	  return Ok(zip_code)
	   },
	   None => return Err(Error::RegexError(format!("regex found no match")))
	};
}

pub fn simple_print() {
  println!("it worked!");
}
/*fn url_builder(city : String) -> String {
	let wunder_key = format!("3d58504465810da1");

} */
