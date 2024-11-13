// use regex:: Regex;
use std::path::Path;
// use std::error::Error;
use crate::response::Response;


#[allow(dead_code)]
#[derive(Debug)]
pub struct Route {
    path: String,
}


// impl Route {
//     pub fn validate(route: &str) -> Result<Regex, Box<dyn Error>> {
//         // Compile the regular expression
//         let regex = Regex::new(r"^/.(.html)/+")?; // Adjusted regex pattern to match routes ending with .html
//
//         if regex.is_match(route) {
//             println!("{}", regex.to_string());
//             Ok(regex)
//         } else {
//             Err("Route validation failed".into())
//         }
//     }
// }

impl Route {
    pub fn validate(route: &str) -> Result<String, Response> {
        if Path::new(&("static/".to_string() + route)).exists() == true {

            if route == "/" {
                return Ok("index.html".to_string())
            }
            return Ok(route.to_string());
        }
        Err(Response::not_found())
    }
}