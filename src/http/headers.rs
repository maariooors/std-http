use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};


#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum RequestHeaders {
    Accept,
    AcceptEncoding,
    AcceptLanguage,
    UserAgent,
    Authorization,
    ContentType,
    Cookie,
    Connection,
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ResponseHeaders {
    ContentType,
    CacheControl,
    Location,
    SetCookie,
    Server,
    Expires,
    ContentLength,
    LastModified,
    KeepAlive,
}


impl Display for RequestHeaders {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestHeaders::Accept => write!(f, "Accept"),
            RequestHeaders::AcceptEncoding => write!(f, "Accept-Encoding"),
            RequestHeaders::AcceptLanguage => write!(f, "Accept-Language"),
            RequestHeaders::UserAgent => write!(f, "User-Agent"),
            RequestHeaders::Authorization => write!(f, "Authorization"),
            RequestHeaders::ContentType => write!(f, "Content-Type"),
            RequestHeaders::Cookie => write!(f, "Cookie"),
            RequestHeaders::Connection => write!(f, "Connection"),
        }
    }
}

impl Display for ResponseHeaders {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseHeaders::ContentType => write!(f, "Content-Type"),
            ResponseHeaders::CacheControl => write!(f, "Cache-Control"),
            ResponseHeaders::Location => write!(f, "Location"),
            ResponseHeaders::SetCookie => write!(f, "Set-Cookie"),
            ResponseHeaders::Server => write!(f, "Server"),
            ResponseHeaders::Expires => write!(f, "Expires"),
            ResponseHeaders::ContentLength => write!(f, "Content-Length"),
            ResponseHeaders::LastModified => write!(f, "Last-Modified"),
            ResponseHeaders::KeepAlive => write!(f, "Keep-Alive"),
        }
    }
}

#[allow(dead_code)]
impl RequestHeaders {

    /// This function takes a vector with vector inside. Each of these vectors correspond
    /// to a header that has been sent to the request.
    /// ```text
    /// String -> [
    ///             [String, String, ...] -> Vector
    ///             [String, String, ...] -> Vector
    ///           ] -> Vector
    /// ```
    /// The first element of the vector is the header in a String format, the rest
    /// of elements are the values of this header which had been previous separated
    /// when a blanc space was found. Therefore, this function introduces into the [Request]
    /// method HashMap, the header and the content associated with it.
    /// # NOTES
    /// When separating the header into multiple Strings when a blanc space is found,
    /// a problem occurs. The header name is separated but with the double dots ":"
    /// on it. This is a problem because when we go and analyze if the header is valid,
    /// we are comparing a "header:" with a "header". Therefore, previous to this
    /// validation the double dot is erased.
    pub fn validate(arr: Vec<Vec<String>>) -> Result
        <HashMap<RequestHeaders, String>, Box<dyn std::error::Error>> {

        let mut headers_hash: HashMap<RequestHeaders, String> = Default::default();

        for mut vec in arr{
            let headers:Vec<RequestHeaders> = RequestHeaders::headers();

            let mut chars = vec[0].chars();
            chars.next_back();
            let chars = chars.as_str();
            vec[0] = chars.parse().unwrap(); // To remove ":"

            for header in headers{
                if vec[0].to_string() ==  header.to_string(){
                    let mut value: String = String::new();
                    for i in 1..vec.len(){
                        value.push_str(vec[i].as_str());
                    }
                    headers_hash.insert(header, value);
                    break
                }
            }
        }
        Ok(headers_hash)
    }

    fn headers() -> Vec<RequestHeaders> {
        let headers: Vec<RequestHeaders> = Vec::from([
            RequestHeaders::Accept,
            RequestHeaders::AcceptEncoding,
            RequestHeaders::AcceptLanguage,
            RequestHeaders::UserAgent,
            RequestHeaders::Authorization,
            RequestHeaders::ContentType,
            RequestHeaders::Cookie,
            RequestHeaders::Connection,
        ]);
        headers
    }
}

// TODO -> return HTTP error