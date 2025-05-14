use pstd::anyhow::AnyResult;

use crate::Applet;

#[derive(Clone)]
pub struct DummyApp;

impl Applet for DummyApp {
    async fn init() -> AnyResult<Self>
    where
        Self: Sized,
    {
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
        app
    }
}
