mod don_error;
mod don_error_ctx;
mod helpers;
mod option_extensions;
mod result_extensions;

pub use {
    crate::{don_error::*, don_error_ctx::*, option_extensions::*, result_extensions::*},
    anyhow,
    don_error_derive::ResponseError,
};

#[cfg(any(feature = "actix", feature = "yew"))]
#[allow(unused)]
pub use helpers::*;

#[cfg(feature = "actix")]
pub type DonHttpResult<T> = DonResult<actix_web::web::Json<T>>;
#[cfg(feature = "actix")]
pub type DonHttpResult2<T, F> = DonResult<Result<actix_web::web::Json<T>, F>>;

pub type DonResult<T> = std::result::Result<T, DonError>;

/// Execute lambda and report in case of error
pub fn try_or_report(lambda: impl FnOnce() -> DonResult<()>) {
    if let Err(err) = lambda() {
        err.report();
    }
}

#[macro_export]
macro_rules! err_msg(
    ($($args:tt)*) => {
        $crate::DonError::from($crate::anyhow::anyhow!($($args)*))
    }
);

#[macro_export]
macro_rules! bail(
    ($($args:tt)*) => {
        return ::std::result::Result::Err($crate::err_msg!($($args)*))
    }
);
