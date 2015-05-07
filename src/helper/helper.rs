//use regex::Regex;
use rustc_serialize::json::Json;

use std::io::Read;

use hyper::Client;
//use hyper::header::Connection;
//use hyper::header::ConnectionOption;


#[derive(Debug)]
pub enum Error { RegexError(String), HTTPGetError(String), JsonError(String) }

#[derive(Debug)]
pub struct WeatherT {
  pub city: String,
  pub current_temp:  String,
  pub current_conditions: String,
  pub wind_string: String,
}

#[allow(unused)]
fn parse_weather_json(weather_json_str: &str) -> Result<WeatherT, Error> {

  let mut temp_string = String::new();
  let mut wind_string = String::new();
  let mut city_string = String::new();
  let mut current_string = String::new();

  let weather_json = match Json::from_str(&weather_json_str) {
    Ok(j) => j,
    Err(e) => return  Err(Error::JsonError(format!("{}", e)))
  };

  match weather_json.search("temp_f") {
    Some(temp) => temp_string = format!("{}", temp),
    None => return Err(Error::JsonError(format!("couldn't find temp_f key")))
  };

  match weather_json.search("full") {
    Some(city) => city_string = format!("{}", city),
    None => return Err(Error::JsonError(format!("couldn't find full key")))
  };
 
   match weather_json.search("wind_string") {
    Some(wind) => wind_string = format!("{}", wind),
    None => return Err(Error::JsonError(format!("couldn't find wind_string key")))
  };
  
   match weather_json.search("weather") {
    Some(current) => current_string = format!("{}", current),
    None => return Err(Error::JsonError(format!("couldn't find weather key")))
  };

  let weather_struct = WeatherT{city: city_string, 
                                current_temp: temp_string,
                                wind_string: wind_string,
                                current_conditions: current_string,


                                                  };

  Ok(weather_struct)

}

pub fn get_current_conditions() -> Result<WeatherT, Error> {
  let zip = match determine_zip_code() {
    Ok(s) => s,
    Err(e) => return Err(e)
  };

  let url = format!("http://api.wunderground.com/api/3d58504465810da1/conditions/q/{}.json", zip);
  //let url = format!("http://api.wunderground.com/api/3d58504465810da1/conditions/q/CA/San_Francisco.json");
  //println!("{}", url);
  let weat_json = match http_get(url) {
    Ok(s) => s,
    Err(e) => return Err(e)
  };

  match parse_weather_json(weat_json.as_str()) {
    Ok(weat_struct) => return Ok(weat_struct),
    Err(e) => return Err(e)
  };

}


 fn http_get(url : String) -> Result<String, Error> {
   let mut client = Client::new();

   //match client.get(&*url).header(Connection(vec![ConnectionOption::Close])).send() {
   match client.get(&*url).send() {
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

  let zip_json = match Json::from_str(&response) {
    Ok(j) => j,
    Err(e) => return  Err(Error::JsonError(format!("{}", e)))
  };

  match zip_json.search("zip") {
    Some(zip) => return Ok(format!("{}", zip)),
    None => return Err(Error::JsonError(format!("couldn't find zip key")))
  };
}

/*

http://api.wunderground.com/api/3d58504465810da1/conditions/q/55024.json



*/
