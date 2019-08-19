#![feature(async_await)]

use async_std::{
    io::BufReader,
    net::{TcpListener, TcpStream},
    prelude::*,
    task,
};
use http::{Request, Response, Version};
use std::net::ToSocketAddrs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub struct Hidouki<A: ToSocketAddrs + Send> {
    address: A,
}

impl<A: ToSocketAddrs + Send> Hidouki<A> {
    pub fn new(address: A) -> Self {
        Self { address }
    }

    pub fn launch(self) {
        if let Err(e) = task::block_on(server(self.address)) {
            eprintln!("{}", e);
        }
    }
}

async fn server(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let mut stream = stream?;
        println!("Accepting from: {}", stream.peer_addr()?);
        let _req = request(&stream).await?;
        let res = response_to_bytes(Response::new(()));
        stream.write_all(&res).await?;
    }
    Ok(())
}
async fn request(stream: &TcpStream) -> Result<Request<String>> {
    let mut reader = BufReader::new(stream);
    let mut request = Vec::new();
    //let mut request_body = Vec::new();
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);

    loop {
        let bytes_read = reader.read_until(b'\n', &mut request).await?;
        let end = request.len() - 1;
        // bounds check then check for consecutive 'CRLF'
        if bytes_read == 0 || end >= 3 && request[end - 3..=end] == [13, 10, 13, 10] {
            break;
        }
    }
    if req.parse(&request)?.is_partial() {
        return Err("Malformed http header".into());
    }
    if let Some(header) = req.headers.iter().find(|h| h.name == "Content-Length") {
        let mut body = Vec::new();
        body.resize(std::str::from_utf8(&header.value)?.parse::<usize>()?, 0);
        reader.read_exact(&mut body).await?;
        make_request(req, Some(body))
    } else {
        make_request(req, None)
    }
}

fn response_to_bytes<T>(response: Response<T>) -> Vec<u8> {
    let mut res = Vec::new();
    let status = response.status();
    res.extend_from_slice(b"HTTP/1.1 ");
    res.extend_from_slice(status.as_str().as_bytes());
    res.extend_from_slice(b" ");
    res.extend_from_slice(status.canonical_reason().unwrap().as_bytes());
    res.extend_from_slice(b"\r\n\r\n");

    res
}
fn make_request(request: httparse::Request, body: Option<Vec<u8>>) -> Result<Request<String>> {
    let mut req = Request::builder();

    for header in request.headers {
        req.header(header.name, header.value);
    }

    if let Some(method) = request.method {
        req.method(method);
    }

    if let Some(path) = request.path {
        req.uri(path);
    }

    if let Some(version) = request.version {
        req.version(match version {
            1 => Version::HTTP_11,
            2 => Version::HTTP_2,
            // There's also 0.9 and 1.0 but I'm not sure how httparse handles that
            _ => unreachable!(),
        });
    }

    // clippy is mad about us returning String in both but they are fundamentally different
    #[allow(clippy::redundant_closure)]
    let req = req.body(body.map_or_else(|| Ok(String::new()), |body| String::from_utf8(body))?)?;
    Ok(req)
}
