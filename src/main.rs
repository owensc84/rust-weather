extern crate weather;

use weather::helper::http_get;

fn main() {


	println!("{}", http_get(format!("http://api.wunderground.com/api/3d58504465810da1/conditions/q/55124.json")).unwrap());
	/*let mut client = Client::new();

	let mut res = client.get("http://api.wunderground.com/api/3d58504465810da1/features/settings/q/.json")
        // set a header
        .header(Connection(vec![ConnectionOption::Close]))
        // let 'er go!
        .send().unwrap();

		let mut body = String::new();
		res.read_to_string(&mut body).unwrap();

		println!("Response: {}", body); */

	/*match determine_zip_code() {
	   Ok(s) => println!("{}", s),
	   Err(e) => println!("{:?}", e)
	}*/
}

/*#[test]
fn test_get_zip() {
   let test_string = match determine_zip_code() {
     Ok(s) => s,
     Err(e) => format!("wrong")
   };
   assert_eq!("55024", test_string);
}*/
