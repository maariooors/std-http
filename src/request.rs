use std::fmt::Display;
use std::collections::HashMap;
use crate::http::headers::RequestHeaders;

use crate::routes::Route;
use crate::response::Response;
use crate::http::methods::HttpMethod;


/// Struct used to create an HTTP request with the needed parameters.
/// # EXAMPLE
/// ```
///     let request: Request = Request {
///         method: HttpMethod::GET,
///         url: "/".to_string(),
///         headers: HashMap::from([
///             (RequestHeaders::ContentType,"text/html".to_string())
///         ]),
///         body: None,
///     };
/// ```
/// # NOTES
/// This struct has a custom build parser to convert the [Request] to a String
/// in order to be sent.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Request {
    pub method: HttpMethod,
    pub url: String,
    pub headers: HashMap<RequestHeaders, String>,
    pub body: Option<String>,
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} HTTP/1.1\r\n", self.method, self.url)?;
        for header in &self.headers {
            write!(f, "{}: {}\r\n", header.0.to_string(), header.1)?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
impl Request {


    /// This function validates the request that the server receives and returns a
    /// [Request] struct containing all the information in a structured way.
    ///
    /// The request is sent as a [String] and it is divided into different vectors
    /// for further validations.
    ///
    /// First we extract the first line and store it in a vector. This line
    /// contains the following attributes: [HttpMethod Path HttpVersion].
    /// ```text
    /// String -> [HttpMethod, Path, HttpVersion]
    /// ```
    /// This vector is then sent to the [HttpMethod::validate] function in order
    /// to validate the content inside. If everything goes correct, a [Request] struct
    /// is returned containing the HttpMethod, Path and HttpVersion.
    ///
    /// Then the headers are structure in a way that a vector is produced containing
    /// inside another vector for each header. This vector stores the HttpHeader
    /// and the values associated with it, all in a different position inside.
    /// ```text
    /// String -> [
    ///             [String, String, ...] -> Vector
    ///             [String, String, ...] -> Vector
    ///           ] -> Vector
    /// ```
    /// This vector is then sent to the [RequestHeaders::validate] function
    /// witch validates the headers and stores them with the corresponded values
    /// inside the request method HashMap and returns the [Request] struct.
    /// # NOTES
    /// When the request is parsed to a String, it ends with \r\n\r\n. Therefore,
    /// when storing the values inside their correspondent vector, we have to check
    /// if we are introducing this "\r\n" to skip it.
    pub fn validate(text: String) -> Response {

        let (first_line, vec_headers) = Self::split_request(text);

        if first_line.len() != 3 { return Response::bad_request(); }

        HttpMethod::validate(&first_line[0]).unwrap();

        let route = match Route::validate(&first_line[1]) {
            Ok(route) => route ,
            Err(response) => return  response ,
        };

        RequestHeaders::validate(vec_headers).unwrap();

        let response = Response::send(route);
        response
    }

    fn split_request(text: String) -> (Vec<String>, Vec<Vec<String>>) {
        let mut first_line: Vec<String> = Vec::new();
        let mut vec_headers: Vec<Vec<String>> = Vec::new();

        let mut i = 0;
        for line in text.lines() {
            if line == "" { break }; // For the last line
            let mut aux: Vec<String> = Vec::new();

            for item in line.split_whitespace() { aux.push(item.to_string()) }
            if i == 0 { first_line = aux } else { vec_headers.push(aux) }
            i += 1;
        }
        (first_line, vec_headers)
    }
}
