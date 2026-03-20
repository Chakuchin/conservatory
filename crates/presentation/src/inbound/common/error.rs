use actix_web::{HttpResponse, HttpResponseBuilder};
use actix_web::http::StatusCode;
use serde::Serialize;
use crate::inbound::employee::dto::EmployeeDTO;

pub trait ConservatoryOptionExt<T: Serialize> {
        fn ok_or_not_found(self) -> HttpResponse;
        fn no_content_or_not_found(self) -> HttpResponse;
}

impl<T: Serialize> ConservatoryOptionExt<T> for Option<T> {
        fn ok_or_not_found(self) -> HttpResponse {
                match self {
                        Some(val) => HttpResponseBuilder::new(StatusCode::OK).json(val),
                        None => HttpResponseBuilder::new(StatusCode::NOT_FOUND).finish()
                }
        }
        fn no_content_or_not_found(self) -> HttpResponse {
                match self {
                        Some(val) => HttpResponseBuilder::new(StatusCode::NO_CONTENT).json(val),
                        None => HttpResponseBuilder::new(StatusCode::NOT_FOUND).finish()
                }
        }
}

pub trait ConservatoryResultExt<T, E: Serialize> {
        fn map_or_internal_server_error<F: FnOnce(T) -> HttpResponse>(self, map: F) -> HttpResponse;
}

impl<T, E: Serialize> ConservatoryResultExt<T, E> for Result<T, E> {
        fn map_or_internal_server_error<F: FnOnce(T) -> HttpResponse>(self, map: F) -> HttpResponse {
                match self {
                        Ok(val) => {
                                map(val)
                        }
                        Err(err) => {
                                HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(err)
                        }
                }
        }
}
