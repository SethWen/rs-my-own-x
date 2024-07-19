use std::time::Duration;

use reqwest::header::HeaderMap;
use reqwest::redirect::Policy;
use reqwest::{Client, Method, RequestBuilder};

// const DEFAULT_HEADERS: HeaderMap = HeaderMap::new();

async fn request_cmd(
    url: &str,
    method: Method,
    headers: HeaderMap,
    timeout: Duration,
) -> Result<Option<(u16, HeaderMap)>, Box<dyn std::error::Error>> {
    if url.is_empty() {
        return Ok(None);
    }

    // let url = match Url::parse(url_string) {
    //     Ok(url) => url,
    //     Err(_) => return Ok(None),
    // };

    let client = Client::builder()
        .timeout(timeout)
        .connection_verbose(true)
        .danger_accept_invalid_certs(true)
        .redirect(Policy::none())
        .build()?;

    let request = client.request(method, url).headers(headers).build()?;
    let response = client.execute(request).await?;

    println!("Response: {} {:?}", response.status(), response.headers());

    if response.status().is_success() {
        let status_code = response.status().as_u16();
        let headers = response.headers().clone();

        // let body = response.bytes().await?;
        // println!("Body: {}", body.len());

        Ok(Some((status_code, headers)))
    } else {
        Ok(None)
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    // let url = "https://httpbin.org/delay/5";
    let url = "https://github.com/oceanbase/oceanbase/releases/download/v4.2.4_CE/oceanbase-ce-utils-4.2.4.0-100000082024070810.el8.x86_64.rpm";
    let method = Method::GET;
    let headers = HeaderMap::new();
    let timeout = Duration::from_secs(10);
    println!("Requesting: {} {}", method, url);

    match request_cmd(url, method, headers, timeout).await? {
        Some((status_code, headers)) => {
            println!("Status Code: {}", status_code);
            for (name, value) in headers.iter() {
                println!("{}: {:?}", name, value);
            }
        }
        None => println!("Request failed or no response"),
    }

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
