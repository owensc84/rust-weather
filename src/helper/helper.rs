use regex::Regex;

use std::io::Read;

use hyper::Client;
//use hyper::header::Connection;
//use hyper::header::ConnectionOption;


#[derive(Debug)]
pub enum Error { RegexError(String), HTTPGetError(String) }

#[derive(Debug)]
pub struct WeatherT {
  pub city : String,
  pub current_temp : String,
}

#[allow(unused)]
fn parse_weather_json(weather_json_str: String) -> Result<WeatherT, Error> {

  let mut temp_string = String::new();
  let mut city_string = String::new();

  // match temp_F
  
  let re_temp_f = match Regex::new(r"temp_f\W\W\d+\W\d+") {
    Ok(r) => r,
    Err(e) => return Err(Error::RegexError(format!("{}", e)))
  };

  let re_city = match Regex::new(r"city\W\W\W\w+") {
    Ok(r) => r,
    Err(e) => return Err(Error::RegexError(format!("{}", e)))
  };

  match re_temp_f.captures(weather_json_str.as_str()) {
    Some(cap) => {
      let v: Vec<&str> = cap.at(0).unwrap().split("\":").collect();
      temp_string = format!("{}", v[1]);
    },
    None => return Err(Error::RegexError(format!("temp_f: regex found no match")))
  };

  match re_city.captures(weather_json_str.as_str()) {
    Some(cap) => {
      let v: Vec<&str> = cap.at(0).unwrap().split("\":\"").collect();
      city_string = format!("{}", v[1]);
    }
    None => return Err(Error::RegexError(format!("city: regex found no match")))
  }

  let weather_struct = WeatherT{city: city_string, current_temp: temp_string};

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

  match parse_weather_json(weat_json) {
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

/*

http://api.wunderground.com/api/3d58504465810da1/conditions/q/55024.json



*/
