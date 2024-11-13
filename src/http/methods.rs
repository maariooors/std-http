use std::fmt::{Display, Formatter};


#[allow(dead_code)]
#[derive(Debug, Default, Eq, PartialEq, Hash)]
pub enum HttpMethod {
    #[default]
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::HEAD => write!(f, "HEAD"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::CONNECT => write!(f, "CONNECT"),
            HttpMethod::OPTIONS => write!(f, "OPTIONS"),
            HttpMethod::TRACE => write!(f, "TRACE"),
            HttpMethod::PATCH => write!(f, "PATCH"),
        }
    }
}

impl HttpMethod {

    pub fn validate(value: &str) -> Result<String, Box<dyn std::error::Error>> {
        let methods: Vec<HttpMethod> = HttpMethod::methods();

        for method in methods {
            if method.to_string() == value {
                return Ok(value.to_string());
            }
        }
        Err("Invalid HTTP method".into())
    }


    fn methods() -> Vec<HttpMethod> {
        let methods: Vec<HttpMethod> = Vec::from([
            HttpMethod::GET,
            HttpMethod::HEAD,
            HttpMethod::POST,
            HttpMethod::PUT,
            HttpMethod::DELETE,
            HttpMethod::CONNECT,
            HttpMethod::OPTIONS,
            HttpMethod::TRACE,
            HttpMethod::PATCH,
        ]);
        methods
    }
}

// TODO -> document the function
// TODO -> return an HTTP error