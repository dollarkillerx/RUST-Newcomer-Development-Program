use std::collections::HashMap;
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            // "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_owned());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            // 处理请求行
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            }else if line.contains(":") {
                // 处理header行
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
                // 空行
            }else{
                // 处理消息体
                parsed_msg_body = line;
            }
        }

        HttpRequest{
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_owned()
        }
    }
}

// 处理请求行
fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace(); // 空格切割
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    // println!("{}, {}, {}", method, resource, version);
    (
        method.into(),
        Resource::Path(resource.to_owned()),
        version.into()
    )
}

// 处理header行
fn process_header_line(s: &str) -> (String, String) {
    // key: localhost:3000
    let mut header_items = s.splitn(2, ":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = String::from(k);
    }
    if let Some(k) = header_items.next() {
        value = String::from(k.trim());
    }
    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_test() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn version_test() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }

    #[test]
    fn test_process_req_line() {
        let s = "GET /greeting HTTP/1.1";
        let (method, resource, version) = process_req_line(s);
        assert_eq!(method, Method::Get);
        assert_eq!(resource, Resource::Path("/greeting".to_owned()));
        assert_eq!(version, Version::V1_1);
    }

    #[test]
    fn test_process_header_line() {
        let s = "Accept: application/json";
        let (key, value) = process_header_line(s);
        assert_eq!(key, "Accept".to_owned());
        assert_eq!(value, "application/json".to_owned());
    }

    #[test]
    fn test_http_request() {
        let s = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.68.0\r\nAccept: */*\r\n\r\n");
        let r: HttpRequest = s.into();
        assert_eq!(r.method, Method::Get);
        assert_eq!(r.version, Version::V1_1);
        assert_eq!(r.resource, Resource::Path("/greeting".to_owned()));
        assert_eq!(r.headers["Host"], "localhost:3000".to_owned());
        assert_eq!(r.msg_body, "");
    }
}
