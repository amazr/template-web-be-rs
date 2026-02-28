use axum::{http::StatusCode, response::IntoResponse};

pub struct Created<T>(pub T);

impl<T: IntoResponse> IntoResponse for Created<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, self.0).into_response()
    }
}
