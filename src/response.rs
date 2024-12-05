use actix_web::{HttpResponse, http::StatusCode};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    status: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<T>,
}

pub fn response_no_content() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

pub fn response_ok<T>(message: &str, resource: T) -> HttpResponse 
where T: Serialize {
    HttpResponse::Ok().json(Response {
        status: true,
        message: message.to_string(),
        data: Some(resource),
        errors: None,
    })
}

pub fn response_created<T>(message: &str, resource: T) -> HttpResponse 
where T: Serialize {
    HttpResponse::Created().json(Response {
        status: true,
        message: message.to_string(),
        data: Some(resource),
        errors: None,
    })
}

pub fn response_bad_request(message: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(Response::<()> {
        status: false,
        message: message.to_string(),
        data: None,
        errors: None,
    })
}

pub fn response_unauthorized(message: &str) -> HttpResponse {
    HttpResponse::Unauthorized().json(Response::<()> {
        status: false,
        message: message.to_string(),
        data: None,
        errors: None,
    })
}

pub fn response_unprocessable_entity<T>(errors: T) -> HttpResponse 
where T: Serialize {
    HttpResponse::UnprocessableEntity().json(Response {
        status: false,
        message: "The given data was invalid".to_string(),
        data: None,
        errors: Some(errors),
    })
}

pub fn response_internal_server_error(message: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(Response::<()> {
        status: false,
        message: message.to_string(),
        data: None,
        errors: None,
    })
}

pub fn response_not_found(message: &str) -> HttpResponse {
    HttpResponse::NotFound().json(Response::<()> {
        status: false,
        message: message.to_string(),
        data: None,
        errors: None,
    })
}

pub fn response_forbidden(message: &str) -> HttpResponse {
    HttpResponse::Forbidden().json(Response::<()> {
        status: false,
        message: message.to_string(),
        data: None,
        errors: None,
    })
}

pub fn response_redirect(url: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", url))
        .finish()
}
