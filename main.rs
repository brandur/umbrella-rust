#[crate_id = "umbrella"];

extern mod extra;
extern mod http;

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::Writer;

use http::client::RequestWriter;
use http::server::{Config, Server, Request, ResponseWriter};
use http::server::request::{AbsolutePath, AbsoluteUri};

#[deriving(Clone)]
struct UmbrellaServer;

impl Server for UmbrellaServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr {
            ip: Ipv4Addr(127, 0, 0, 1),
            port: 8001
        } }
    }

    fn handle_request(&self, r: &Request, w: &mut ResponseWriter) {
        let downstream_url = "http://localhost:5000/";

        let path = match &r.request_uri {
            &AbsolutePath(ref path) => (*path).clone(),
            &AbsoluteUri(ref uri) => uri.path.clone(),
            _ => { ~"" }
        };
        let url: Option<extra::url::Url> = from_str(downstream_url + path);
        let request = RequestWriter::new(r.method.clone(), url.unwrap());

        let mut response = match request.read_response() {
            Ok(response) => response,
            Err(_request) => {
                w.status = http::status::ServiceUnavailable;
                return;
            }
        };

        w.status = http::status::Status::from_code_and_reason(
            response.status.code(),
            response.status.reason()
        );
        w.write(response.read_to_end());
    }
}

fn main() {
    UmbrellaServer.serve_forever();
}