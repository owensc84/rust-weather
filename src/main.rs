#![feature(plugin)]
#![plugin(regex_macros)]

extern crate hyper;
extern crate regex;

use regex::Regex;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;
use hyper::header::ConnectionOption;


fn determine_zip_code() -> String {
	let url = format!("http://ip-api.com/json");
	let mut client = Client::new();
	let mut res = client.get(url).header(Connection(vec![ConnectionOption::Close])).send().unwrap();
	let re = Regex::new(r"(?<=zip":")\d{5}").unwrap();
}

/*fn url_builder(city : String) -> String {
	let wunder_key = format!("3d58504465810da1");
	
} */

fn main() {

	let mut client = Client::new();
	
	let mut res = client.get("http://api.wunderground.com/api/3d58504465810da1/features/settings/q/.json")
        // set a header
        .header(Connection(vec![ConnectionOption::Close]))
        // let 'er go!
        .send().unwrap();
		
		let mut body = String::new();
		res.read_to_string(&mut body).unwrap();
		
		println!("Response: {}", body);
}
