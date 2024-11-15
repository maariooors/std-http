use std::fs;
use std::fmt::Display;
use std::collections::HashMap;

use crate::http::status::HttpStatusCode;
use crate::http::headers::ResponseHeaders;


/// Struct used to create an HTTP response with the needed parameters.
/// # EXAMPLE
/// ```
///     let response: Response = Response{
///         version: "HTTP/1.1".to_string(),
///         status: "200".to_string(),
///         headers: HashMap::from([
///             (ResponseHeaders::ContentType, "text/html".to_string())
///         ]),
///         body: None
///     };
/// ```
/// # NOTES
/// This struct has a custom build parser to convert the [Response] to a String
/// in order to be sent.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Response {
    pub version: String,
    pub status: HttpStatusCode,
    pub headers: HashMap<ResponseHeaders, String>,
    pub body: Option<String>,
}

/// Function used to convert a [Response] into a String following the standard structure
/// # SUCCESS
/// String containing all the response with the correct format according to the standard
/// # ERROR
/// Error type.
impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}\r\n", self.version, self.status as u16)?;
        for header in &self.headers {
            write!(f, "{}: {}\r\n", header.0.to_string(), header.1)?;
        }
        write!(f, "\r\n")?;
        match &self.body {
            Some(value) => write!(f, "{}\r\n", value)?,
            None => {}
        }
        Ok(())
    }
}


impl Response {

    pub fn default() -> Response {
        Response {
            version: "HTTP/1.1".to_string(),
            status: HttpStatusCode::default(),
            headers: Default::default(),
            body: None,
        }
    }

    pub fn ok() -> Response {

        let body: String = fs::read_to_string("./static/index.html".to_string()).unwrap();

        let mut response: Response = Response::default();
        response.headers.insert(ResponseHeaders::ContentType, "text/html".to_string());
        response.body = Some(body);
        response
    }

    pub fn bad_request() -> Response {
        let body: String = fs::read_to_string("./static/badRequest.html".to_string()).unwrap();

        let mut response: Response = Response::default();
        response.status = HttpStatusCode::BadRequest;
        response.headers.insert(ResponseHeaders::ContentType, "text/html".to_string());
        response.body = Some(body);
        response
    }

    pub fn not_found() -> Response {

        let body: String = fs::read_to_string("./static/notFound.html".to_string()).unwrap();

        let mut response: Response = Response::default();
        response.status = HttpStatusCode::NotFound;
        response.headers.insert(ResponseHeaders::ContentType, "text/html".to_string());
        response.body = Some(body);
        response
    }

    // TODO -> move this function to the server mod
    pub fn send(path: String) -> Response {

        let body = match Self::open_file("static/".to_string() + &path) {
            Ok(body) => body,
            Err(response) => return response,
        };

        let mut response: Response = Response::default();
        response.headers.insert(ResponseHeaders::ContentType, "text/html".to_string());
        response.body = Some(body);
        response
    }

    fn open_file(file: String) -> Result<String, Response> {

        let value = match fs::read_to_string(file) {
            Ok(value) => value,
            _ => return Err(Self::not_found())
        };
        Ok(value)
    }
}

// TODO -> document the functions