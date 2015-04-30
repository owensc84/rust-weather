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
enum Error { RegexError(String), HTTPGetError(String) }


fn http_get(url : String) -> Result<String, Error> {
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


fn determine_zip_code() -> Result<String, Error>  {
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
	   Some(_) => (), 
	   None => return Err(Error::RegexError(format!("regex found no match")))
	}


	let mut zip_code = format!("{}", re.captures(response.as_str()).unwrap().at(0).unwrap());
	
	zip_code.remove(0);	// remove the leading "
	
	Ok(zip_code)
}

/*fn url_builder(city : String) -> String {
	let wunder_key = format!("3d58504465810da1");
	
} */

fn main() {

	/*let mut client = Client::new();
	
	let mut res = client.get("http://api.wunderground.com/api/3d58504465810da1/features/settings/q/.json")
        // set a header
        .header(Connection(vec![ConnectionOption::Close]))
        // let 'er go!
        .send().unwrap();
		
		let mut body = String::new();
		res.read_to_string(&mut body).unwrap(); 
		
		println!("Response: {}", body); */
	
	match determine_zip_code() {
	   Ok(s) => println!("{}", s),
	   Err(e) => println!("{:?}", e)
	}
}
