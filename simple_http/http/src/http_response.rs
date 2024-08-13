use std::collections::HashMap;
use tokio::io::{ AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse {
    version: String,
    status_code: String,
    status_text: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: "200".to_string(),
            status_text: "OK".to_string(),
            headers: None,
            body: None,
        }
    }
}

impl HttpResponse {
    pub fn new(
        status_code: String,
        headers: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Self {
        let mut response: HttpResponse = Default::default();
        if status_code != "200" {
            response.status_code = status_code;
        }

        response.headers = headers.or_else(|| {
            let mut h = HashMap::new();
            h.insert("Content-Type".to_string(), "text/html".to_string());
            Some(h)
        });

        response.status_text = match response.status_code.as_str() {
            "200" => "OK".to_string(),
            "400" => "Bad Request".to_string(),
            "404" => "Not Found".to_string(),
            "500" => "Internal Server Error".to_string(),
            _ => "Not Found".to_string(),
        };

        response.body = body;
        response
    }

    pub async fn send_response(&self, stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let response_string: String = self.clone().into();
        stream.write_all(response_string.as_bytes()).await?;
        Ok(())
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn status_code(&self) -> &str {
        &self.status_code
    }

    pub fn headers(&self) -> String {
        let mut headers = String::new();
        if let Some(h) = &self.headers {
            for (key, value) in h {
                headers.push_str(&format!("{}: {}\r\n", key, value));
            }
        }
        headers
    }

    pub fn status_text(&self) -> &str {
        &self.status_text
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

impl From<HttpResponse> for String {
    fn from(res: HttpResponse) -> Self {
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            res.version(),
            res.status_code(),
            res.status_text(),
            res.headers(),
            res.body().len(),
            res.body()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_http_response() {
        let response = HttpResponse::new("404".to_string(), None, None);
        assert_eq!(response.status_code(), "404");
    }

    #[tokio::test]
    async fn test_http_response_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        let response = HttpResponse::new("200".to_string(), Some(headers), None);
        assert_eq!(response.headers(), "Content-Type: text/html\r\n");
    }

    #[tokio::test]
    async fn test_http_response_with_body() {
        let response = HttpResponse::new("200".to_string(), None, Some("Hello World".to_string()));
        assert_eq!(response.body(), "Hello World");
    }

    #[tokio::test]
    async fn test_http_response_with_headers_and_body() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        let response = HttpResponse::new("200".to_string(), Some(headers), Some("Hello World".to_string()));
        assert_eq!(response.headers(), "Content-Type: text/html\r\n");
        assert_eq!(response.body(), "Hello World");
    }

    #[tokio::test]
    async fn test_http_response_to_string() {
        let response = HttpResponse::new("200".to_string(), None, Some("Hello World".to_string()));
        assert_eq!(
            String::from(response),
            "HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\nHello World"
        );
    }

    #[tokio::test]
    async fn test_http_response_to_string_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        let response = HttpResponse::new("200".to_string(), Some(headers), Some("Hello World".to_string()));
        assert_eq!(
            String::from(response),
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 11\r\n\r\nHello World"
        );
    }
}
