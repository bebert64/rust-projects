pub use {actix_cors, actix_web, don_error};

use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
};

pub mod prelude {
    pub use crate::services;

    pub use {
        actix_web::{
            get, post,
            web::{Json, Path},
            Responder,
        },
        don_error::DonResult,
    };
}

pub trait DonActixApp:
    ServiceFactory<
    ServiceRequest,
    Config = (),
    Response = ServiceResponse<EitherBody<BoxBody>>,
    Error = actix_web::Error,
    InitError = (),
>
{
}

impl<T> DonActixApp for T where
    T: ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<EitherBody<BoxBody>>,
        Error = actix_web::Error,
        InitError = (),
    >
{
}

#[macro_export]
macro_rules! services {
    ($($service:ident),* $(,)?) => {
        pub(super) fn add_services<T>(app: $crate::actix_web::App<T>) -> $crate::actix_web::App<T>
        where
            T: $crate::DonActixApp,
        {
            $(let app = app.service($service);)*
            app
        }
    };
}

#[macro_export]
macro_rules! units {
    ($($unit:ident),* $(,)?) => {
        $(mod $unit;)*

        pub fn app_with_services() -> $crate::actix_web::App<impl $crate::DonActixApp> {
            let app = $crate::actix_web::App::new().wrap($crate::actix_cors::Cors::permissive());
            $(let app = $unit::add_services(app);)*
            app
        }
    };
}

#[macro_export]
macro_rules! build_server {
    ($crate_back:ident) => {
        build_server!($crate_back, port: 8080);
    };
    ($crate_back:ident, port: $port:literal) => {
        #[$crate::actix_web::main]
        async fn main() -> std::io::Result<()> {
            $crate::actix_web::HttpServer::new($crate_back::rest::app_with_services)
                .bind(("0.0.0.0", $port))?
                .run()
                .await
        }
    };
}
