use anyhow::Error as AnyErr;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

/// An internal error enum for representing all the possible failure states
#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("{0} not found")]
    NotFound(&'static str),
    #[error("already exists")]
    AlreadyExists(anyhow::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}
impl warp::reject::Reject for ServiceError {}
impl From<ServiceError> for warp::reject::Rejection {
    fn from(e: ServiceError) -> Self {
        warp::reject::custom(e)
    }
}
// impl From<ServiceError> for StatusCode {
//     fn from(e: ServiceError) -> Self {
//         match e{
//             Unauthorized =>
//         }
//     }
// }
impl From<r2d2::Error> for ServiceError {
    fn from(e: r2d2::Error) -> Self {
        dbg!("r2d2 error", &e);
        ServiceError::Other(e.into())
    }
}
impl From<argonautica::Error> for ServiceError {
    fn from(e: argonautica::Error) -> Self {
        match e.kind() {
            _ => {
                dbg!(e.kind());
                ServiceError::Unauthorized
            }
        }
    }
}
// impl From<secp256k1::Error> for ServiceError {
//     fn from(e: secp256k1::Error) -> Self {
//         dbg!("secp error", &e);
//         ServiceError::Other(e.into())
//     }
// }
impl From<base64::DecodeError> for ServiceError {
    fn from(e: base64::DecodeError) -> Self {
        dbg!("base64 error", &e);
        ServiceError::Other(e.into())
    }
}
impl From<diesel::result::Error> for ServiceError {
    fn from(e: diesel::result::Error) -> Self {
        use diesel::result::{DatabaseErrorKind, Error as DieselErr};
        match &e {
            DieselErr::DatabaseError(kind, info) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    ServiceError::AlreadyExists(AnyErr::msg(info.message().to_owned()))
                }
                _ => ServiceError::Other(e.into()),
            },
            _ => ServiceError::Other(e.into()),
        }
    }
}

/// An API error serializable to JSON responses
struct ErrMsg {
    statuscode: StatusCode,
    message: String,
}
impl ErrMsg {
    pub fn new(code: StatusCode, msg: &str) -> Self {
        ErrMsg {
            statuscode: code,
            message: msg.into(),
        }
    }
    pub fn into_reply(&self) -> impl warp::Reply {
        warp::reply::with_status(warp::reply::json(&self), self.statuscode)
    }
}
impl Serialize for ErrMsg {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ErrMsg", 3)?;
        state.serialize_field("code", &self.statuscode.as_u16())?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("status", "error")?;
        state.end()
    }
}
impl From<Rejection> for ErrMsg {
    fn from(r: Rejection) -> Self {
        if r.is_not_found() {
            return ErrMsg::new(StatusCode::NOT_FOUND, "Not found");
        }
        if let Some(sErr) = r.find::<ServiceError>() {
            return ErrMsg::from(sErr);
        }
        if let Some(_) = r.find::<warp::reject::MethodNotAllowed>() {
            return ErrMsg::from(StatusCode::METHOD_NOT_ALLOWED);
        }
        ErrMsg::new(StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_REJECTION")
    }
}
impl From<&ServiceError> for ErrMsg {
    fn from(e: &ServiceError) -> Self {
        match e {
            ServiceError::Unauthorized => ErrMsg::new(StatusCode::UNAUTHORIZED, "Unauthorized"),
            ServiceError::AlreadyExists(_) => ErrMsg::new(StatusCode::CONFLICT, "Already exists"),
            _ => ErrMsg::new(StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_REJECTION"),
        }
    }
}
impl From<StatusCode> for ErrMsg {
    fn from(sc: StatusCode) -> Self {
        match sc {
            StatusCode::UNAUTHORIZED => ErrMsg::new(sc, "Unauthorized"),
            StatusCode::METHOD_NOT_ALLOWED => ErrMsg::new(sc, "Method not Allowed"),
            _ => ErrMsg::new(StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_REJECTION"),
        }
    }
}

pub async fn handle_rejection(r: Rejection) -> Result<impl Reply, Infallible> {
    dbg!(&r);
    Ok(ErrMsg::from(r).into_reply())
}
