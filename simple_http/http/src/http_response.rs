use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        let mut response: HttpResponse = Default::default();
        if status_code != "200" {
            response.status_code = status_code;
        }

        response.headers = headers.or_else(|| {
            let mut h = HashMap::new();
            h.insert("Content-Type", "text/html");
            Some(h)
        });

        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };

        response.body = body;
        response
    }

    pub async fn send_response(&self, stream: Arc<Mutex<TcpStream>>) -> Result<(), Box<dyn std::error::Error>> {
        let response_string: String = self.clone().into();
        stream.lock().await.write_all(response_string.as_bytes()).await?;
        stream.lock().await.flush().await?;
        Ok(())
    }

    pub fn version(&self) -> &str {
        self.version
    }

    pub fn status_code(&self) -> &str {
        self.status_code
    }

    pub fn headers(&self) -> String {
        let mut headers = String::new();
        if let Some(h) = &self.headers {
            for (key, value) in h {
                headers.push_str(format!("{}: {}\r\n", key, value).as_str());
            }
        }
        headers
    }

    pub fn status_text(&self) -> &str {
        self.status_text
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse<'a>) -> Self {
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
        let response = HttpResponse::new("404", None, None);
        assert_eq!(response.status_code(), "404");
    }

    #[tokio::test]
    async fn test_http_response_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type", "text/html");
        let response = HttpResponse::new("200", Some(headers), None);
        assert_eq!(response.headers(), "Content-Type: text/html\r\n");
    }

    #[tokio::test]
    async fn test_http_response_with_body() {
        let response = HttpResponse::new("200", None, Some("Hello World".to_string()));
        assert_eq!(response.body(), "Hello World");
    }

    #[tokio::test]
    async fn test_http_response_with_headers_and_body() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type", "text/html");
        let response = HttpResponse::new("200", Some(headers), Some("Hello World".to_string()));
        assert_eq!(response.headers(), "Content-Type: text/html\r\n");
        assert_eq!(response.body(), "Hello World");
    }

    #[tokio::test]
    async fn test_http_response_to_string() {
        let response = HttpResponse::new("200", None, Some("Hello World".to_string()));
        assert_eq!(
            String::from(response),
            "HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\nHello World"
        );
    }

    #[tokio::test]
    async fn test_http_response_to_string_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type", "text/html");
        let response = HttpResponse::new("200", Some(headers), Some("Hello World".to_string()));
        assert_eq!(
            String::from(response),
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 11\r\n\r\nHello World"
        );
    }
}
