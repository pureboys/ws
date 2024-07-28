use std::fmt::{Display, Formatter};
use actix_web::{error, HttpResponse};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use serde::{ Serialize};

#[allow(dead_code)]
#[derive(Serialize, Debug)]
pub enum MyError {
    TeraError(String),
    ActixError(String),
    NotFound(String),
}

impl std::error::Error for MyError {}

impl MyError {
    fn error_reponse(&self) -> String {
        match self {
            MyError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            MyError::TeraError(msg) => {
                println!("Error in rendering the template {:?}", msg);
                msg.into()
            }
            MyError::NotFound(msg) => {
                println!("Not Found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::ActixError(_) | MyError::TeraError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .json(MyErrorResponse {
                error_message: self.error_reponse(),
            })
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}

impl From<tera::Error> for MyError {
    fn from(err: tera::Error) -> Self {
        MyError::TeraError(err.to_string())
    }
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}


