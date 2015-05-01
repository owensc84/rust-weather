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

pub mod helper;
