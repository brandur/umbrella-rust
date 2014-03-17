#[crate_id = "umbrella"];

extern mod extra;
extern mod http;

use std::io::net::ip::{SocketAddr, Ipv4Addr, Port};
use std::io::net::tcp::{TcpStream};
use std::io::Writer;
use std::os;

use http::client::{RequestWriter, ResponseReader};
use http::server::{Config, Server, Request, ResponseWriter};
use http::server::request::{AbsolutePath, AbsoluteUri};

#[deriving(Clone)]
struct UmbrellaServer {
    port: Port,
    upstream_url: ~str,
}

impl UmbrellaServer {
    fn write_extension(&self, response: &ResponseReader<TcpStream>, w: &mut ResponseWriter, header: ~str) {
        match response.headers.extensions.find(&header) {
            Some(val) => { w.headers.extensions.swap(header, (*val).clone()); },
            None      => {}
        };
    }
}

impl Server for UmbrellaServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr {
            ip: Ipv4Addr(127, 0, 0, 1),
            port: self.port
        } }
    }

    fn handle_request(&self, r: &Request, w: &mut ResponseWriter) {
        let path = match &r.request_uri {
            &AbsolutePath(ref path) => (*path).clone(),
            &AbsoluteUri(ref uri) => uri.path.clone(),
            _ => { ~"" }
        };
        let url: Option<extra::url::Url> = from_str(self.upstream_url + path);
        let mut request = RequestWriter::new(r.method.clone(), url.unwrap());

        request.headers.accept        = r.headers.accept.clone();
        request.headers.authorization = r.headers.authorization.clone();
        request.headers.range         = r.headers.range.clone();
        request.headers.user_agent    = r.headers.user_agent.clone();

        let mut response = match request.read_response() {
            Ok(response) => response,
            Err(_request) => {
                w.status = http::status::ServiceUnavailable;
                return;
            }
        };

        self.write_extension(&response, w, ~"Next-Range");
        self.write_extension(&response, w, ~"Prev-Range");

        w.status = http::status::Status::from_code_and_reason(
            response.status.code(),
            response.status.reason()
        );
        w.write(response.read_to_end());
    }
}

fn main() {
    let args = os::args();
    match args.len() {
        0 => unreachable!(),
        3 => serve(args[1].clone(), args[2].clone()),
        _ => {
            println!("Usage: {} PORT UPSTREAM_URL", args[0]);
            return;
        },
    };
}

fn serve(port: ~str, upstream_url: ~str) {
    let server = UmbrellaServer {
        port: from_str(port).unwrap(),
        upstream_url: upstream_url
    };
    server.serve_forever();
}
