extern crate weather;

use weather::helper::determine_zip_code;

#[test]
fn test_get_zip() {
   let test_string = match determine_zip_code() {
     Ok(s) => s,
     Err(e) => format!("wrong")
   };
   assert_eq!("55024", test_string);
}
