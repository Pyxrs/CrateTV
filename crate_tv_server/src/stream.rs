use actix_web::{get, web, HttpResponse, Responder};
use pstd::anyhow::AnyResult;
use serde::Serialize;

use crate::Applet;

#[derive(Clone)]
pub struct StreamApp;

impl Applet for StreamApp {
    async fn init() -> AnyResult<Self> {
        Ok(Self)
    }

    fn build<T>(self, app: actix_web::App<T>) -> actix_web::App<T>
    where
        T: actix_web::dev::ServiceFactory<
            actix_web::dev::ServiceRequest,
            Config = (),
            Error = actix_web::error::Error,
            InitError = (),
        >,
    {
        app.service(stream_info)
    }
}

#[derive(Serialize)]
struct StreamInfo {
    url: String,
}

/// Returns the HLS stream URL for the given streamer name.
/// TODO: Look up the stream in the database and return 404 when the stream is not live.
#[get("/stream/{name}")]
async fn stream_info(_name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(StreamInfo {
        url: "https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8".to_string(),
    })
}
