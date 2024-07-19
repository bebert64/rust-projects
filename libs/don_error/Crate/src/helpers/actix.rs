use crate::DonError;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

impl ResponseError for DonError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        println!("{self:?}");
        HttpResponse::build(self.status_code()).body("Internal Server Error")
    }
}
