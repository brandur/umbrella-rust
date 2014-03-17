# umbrella-rust

A simple Rust implementation of the API umbrella.

    rustpkg install github.com/chris-morgan/rust-http
    rustc main.rs
    ./umbrella 8001 http://localhost:5000

Now a request can be issued against the proxy like so:

    $ curl -i -H "Accept: application/vnd.heroku+json; version=3" -H "Range: id ..; max=1" --user :4e118554-49c4-46a1-ac12-582f591d021a http://localhost:8001/apps
    HTTP/1.1 206 Partial Content
    Transfer-Encoding: chunked
    Next-Range: id ]1fd05fb4-97a2-49ca-aed4-68fd58ebe069..; max=1

    [
      {
        "id":"1fd05fb4-97a2-49ca-aed4-68fd58ebe069",
        "name":"infinite-crag-6128",
        ...
      }
    ]
