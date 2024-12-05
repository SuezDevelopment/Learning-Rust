use actix_web::{
    dev::{ forward_ready, Service, ServiceRequest, ServiceResponse, Transform },
    Error,
};

use futures::future::LocalBoxFuture;
use futures::future::{ ready, Ready };

use crate::response::*;

use actix_session::{Session, SessionExt};

use crate::models::*;


pub struct Auth;

impl<S> Transform<S, ServiceRequest>
    for Auth
    where
        S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
        S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest>
    for AuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
        S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("requested path ::::: {}", req.path());

        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    if validate_token(token) {

                        let user = User::new("admin".to_string(), "admin".to_string());

                        let session = req.get_session();
                        session.insert("user_id", &user.id).expect("Failed to insert user_id into session");
                        
                        let fut = self.service.call(req);
                        return Box::pin(async move {
                            fut.await
                        });
                    }
                }
            }
        }

        Box::pin(async move {
            Ok(req.into_response(response_unauthorized("unauthorized: invalid or missing token")))
        })

    }}

pub fn validate_token(token: &str) -> bool {
    token == "valid_token"
}

pub fn get_user_from_session(session: &Session) -> Option<User> {
    match (
        session.get::<String>("user_id").ok()?,
        session.get::<String>("user_role").ok()?
    ) {
        (Some(id), Some(role)) => Some(User::new(
            id,
            role
        )),
        _ => None
    }
}