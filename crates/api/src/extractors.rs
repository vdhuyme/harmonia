use axum::extract::{FromRequest, FromRequestParts};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde::de::DeserializeOwned;
use std::fmt::Display;
use validator::Validate;

pub struct ValidatedJson<T>(pub T)
where
    T: Validate + DeserializeOwned + Send + 'static;

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: Validate + DeserializeOwned + Send + 'static,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(
        req: Request<Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let payload: T = axum::extract::Json::from_request(req, state)
            .await
            .map_err(|e| {
                (StatusCode::BAD_REQUEST, format!("Invalid JSON: {e}"))
            })?
            .0;

        payload
            .validate()
            .map(|_| ValidatedJson(payload))
            .map_err(|errors| {
                (StatusCode::BAD_REQUEST, format_validation_errors(&errors))
            })
    }
}

pub struct ValidatedPath<T>(pub T)
where
    T: Validate + DeserializeOwned + Send + 'static;

impl<S, T> FromRequestParts<S> for ValidatedPath<T>
where
    T: Validate + DeserializeOwned + Send + 'static,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let payload =
            axum::extract::Path::<T>::from_request_parts(parts, state)
                .await
                .map_err(|e| {
                    (StatusCode::BAD_REQUEST, format!("Invalid path: {e}"))
                })?
                .0;

        payload
            .validate()
            .map(|_| ValidatedPath(payload))
            .map_err(|errors| {
                (StatusCode::BAD_REQUEST, format_validation_errors(&errors))
            })
    }
}

fn format_validation_errors(errors: &validator::ValidationErrors) -> String {
    errors
        .field_errors()
        .iter()
        .map(|(field, err_vec)| {
            let messages = err_vec
                .iter()
                .map(format_field_error)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{field}: {messages}")
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn format_field_error(error: &validator::ValidationError) -> String {
    error
        .message
        .as_ref()
        .map_or_else(|| "invalid".to_string(), display_to_string)
}

fn display_to_string<T>(value: T) -> String
where
    T: Display,
{
    value.to_string()
}
