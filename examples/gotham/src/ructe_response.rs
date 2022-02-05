use gotham::hyper::http::header::CONTENT_TYPE;
use gotham::hyper::{Body, Response, StatusCode};
use gotham::state::State;
use mime::TEXT_HTML_UTF_8;
use std::io;

pub trait RucteResponse: Sized {
    fn html<F>(self, do_render: F) -> (Self, Response<Body>)
    where
        F: FnOnce(&mut Vec<u8>) -> io::Result<()>;
}

impl RucteResponse for State {
    fn html<F>(self, do_render: F) -> (Self, Response<Body>)
    where
        F: FnOnce(&mut Vec<u8>) -> io::Result<()>,
    {
        let mut buf = Vec::new();
        let res = match do_render(&mut buf) {
            Ok(()) => Response::builder()
                .header(CONTENT_TYPE, TEXT_HTML_UTF_8.as_ref())
                .body(buf.into())
                .unwrap(),
            Err(e) => {
                println!("Rendering failed: {}", e);
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(CONTENT_TYPE, TEXT_HTML_UTF_8.as_ref())
                    .body(buf.into())
                    .unwrap()
            }
        };
        (self, res)
    }
}
