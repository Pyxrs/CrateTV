use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    middleware, App, HttpServer,
};
use pstd::anyhow::AnyResult;
use std::net::Ipv4Addr;
use tokio::try_join;

mod account;
mod dummy;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    let _ = pstd::dotenv::to_env();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Rate limiting configuration
    let governor_conf = GovernorConfigBuilder::default()
        .requests_per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();

    let (account, dummy) = try_join!(account::AccountApp::init(), dummy::DummyApp::init()).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .wrap(Governor::new(&governor_conf))
            .install(&account)
            .install(&dummy)
    })
    .bind((Ipv4Addr::LOCALHOST, 3000))?
    .run()
    .await
}

trait Applet: Clone {
    async fn init() -> AnyResult<Self>
    where
        Self: Sized;
    fn build<T>(self, app: App<T>) -> App<T>
    where
        T: ServiceFactory<
            ServiceRequest,
            Config = (),
            Error = actix_web::error::Error,
            InitError = (),
        >;
}

trait AppletInstaller<
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::error::Error, InitError = ()>,
>
{
    fn install(self, applet: &impl Applet) -> Self;
}

impl<T> AppletInstaller<T> for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::error::Error, InitError = ()>,
{
    fn install(self, applet: &impl Applet) -> Self {
        applet.clone().build(self)
    }
}
