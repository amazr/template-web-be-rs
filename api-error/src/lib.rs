use axum::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize, utoipa::ToSchema, utoipa::IntoResponses)]
pub enum ErrorResponse {
    #[response(status = 400, description = "Bad Request")]
    BadRequest { error: String },
    #[response(status = 401, description = "Unauthorized")]
    Unauthorized { error: String },
    #[response(status = 402, description = "Payment Required")]
    PaymentRequired { error: String },
    #[response(status = 403, description = "Forbidden")]
    Forbidden { error: String },
    #[response(status = 404, description = "Not Found")]
    NotFound { error: String },
    #[response(status = 405, description = "Method Not Allowed")]
    MethodNotAllowed { error: String },
    #[response(status = 406, description = "Not Acceptable")]
    NotAcceptable { error: String },
    #[response(status = 407, description = "Proxy Authentication Required")]
    ProxyAuthenticationRequired { error: String },
    #[response(status = 408, description = "Request Timeout")]
    RequestTimeout { error: String },
    #[response(status = 409, description = "Conflict")]
    Conflict { error: String },
    #[response(status = 410, description = "Gone")]
    Gone { error: String },
    #[response(status = 411, description = "Length Required")]
    LengthRequired { error: String },
    #[response(status = 412, description = "Precondition Failed")]
    PreconditionFailed { error: String },
    #[response(status = 413, description = "Payload Too Large")]
    PayloadTooLarge { error: String },
    #[response(status = 414, description = "URI Too Long")]
    UriTooLong { error: String },
    #[response(status = 415, description = "Unsupported Media Type")]
    UnsupportedMediaType { error: String },
    #[response(status = 416, description = "Range Not Satisfiable")]
    RangeNotSatisfiable { error: String },
    #[response(status = 417, description = "Expectation Failed")]
    ExpectationFailed { error: String },
    #[response(status = 418, description = "I'm a teapot")]
    ImATeapot { error: String },
    #[response(status = 421, description = "Misdirected Request")]
    MisdirectedRequest { error: String },
    #[response(status = 422, description = "Unprocessable Entity")]
    UnprocessableEntity { error: String },
    #[response(status = 423, description = "Locked")]
    Locked { error: String },
    #[response(status = 424, description = "Failed Dependency")]
    FailedDependency { error: String },
    #[response(status = 425, description = "Too Early")]
    TooEarly { error: String },
    #[response(status = 426, description = "Upgrade Required")]
    UpgradeRequired { error: String },
    #[response(status = 428, description = "Precondition Required")]
    PreconditionRequired { error: String },
    #[response(status = 429, description = "Too Many Requests")]
    TooManyRequests { error: String },
    #[response(status = 431, description = "Request Header Fields Too Large")]
    RequestHeaderFieldsTooLarge { error: String },
    #[response(status = 451, description = "Unavailable For Legal Reasons")]
    UnavailableForLegalReasons { error: String },
    #[response(status = 500, description = "Internal Server Error")]
    InternalServerError { error: String },
    #[response(status = 501, description = "Not Implemented")]
    NotImplemented { error: String },
    #[response(status = 502, description = "Bad Gateway")]
    BadGateway { error: String },
    #[response(status = 503, description = "Service Unavailable")]
    ServiceUnavailable { error: String },
    #[response(status = 504, description = "Gateway Timeout")]
    GatewayTimeout { error: String },
    #[response(status = 505, description = "HTTP Version Not Supported")]
    HttpVersionNotSupported { error: String },
    #[response(status = 506, description = "Variant Also Negotiates")]
    VariantAlsoNegotiates { error: String },
    #[response(status = 507, description = "Insufficient Storage")]
    InsufficientStorage { error: String },
    #[response(status = 508, description = "Loop Detected")]
    LoopDetected { error: String },
    #[response(status = 510, description = "Not Extended")]
    NotExtended { error: String },
    #[response(status = 511, description = "Network Authentication Required")]
    NetworkAuthenticationRequired { error: String },
}

impl ErrorResponse {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ErrorResponse::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ErrorResponse::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            ErrorResponse::PaymentRequired { .. } => StatusCode::PAYMENT_REQUIRED,
            ErrorResponse::Forbidden { .. } => StatusCode::FORBIDDEN,
            ErrorResponse::NotFound { .. } => StatusCode::NOT_FOUND,
            ErrorResponse::MethodNotAllowed { .. } => StatusCode::METHOD_NOT_ALLOWED,
            ErrorResponse::NotAcceptable { .. } => StatusCode::NOT_ACCEPTABLE,
            ErrorResponse::ProxyAuthenticationRequired { .. } => {
                StatusCode::PROXY_AUTHENTICATION_REQUIRED
            }
            ErrorResponse::RequestTimeout { .. } => StatusCode::REQUEST_TIMEOUT,
            ErrorResponse::Conflict { .. } => StatusCode::CONFLICT,
            ErrorResponse::Gone { .. } => StatusCode::GONE,
            ErrorResponse::LengthRequired { .. } => StatusCode::LENGTH_REQUIRED,
            ErrorResponse::PreconditionFailed { .. } => StatusCode::PRECONDITION_FAILED,
            ErrorResponse::PayloadTooLarge { .. } => StatusCode::PAYLOAD_TOO_LARGE,
            ErrorResponse::UriTooLong { .. } => StatusCode::URI_TOO_LONG,
            ErrorResponse::UnsupportedMediaType { .. } => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            ErrorResponse::RangeNotSatisfiable { .. } => StatusCode::RANGE_NOT_SATISFIABLE,
            ErrorResponse::ExpectationFailed { .. } => StatusCode::EXPECTATION_FAILED,
            ErrorResponse::ImATeapot { .. } => StatusCode::IM_A_TEAPOT,
            ErrorResponse::MisdirectedRequest { .. } => StatusCode::MISDIRECTED_REQUEST,
            ErrorResponse::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorResponse::Locked { .. } => StatusCode::LOCKED,
            ErrorResponse::FailedDependency { .. } => StatusCode::FAILED_DEPENDENCY,
            ErrorResponse::TooEarly { .. } => StatusCode::TOO_EARLY,
            ErrorResponse::UpgradeRequired { .. } => StatusCode::UPGRADE_REQUIRED,
            ErrorResponse::PreconditionRequired { .. } => StatusCode::PRECONDITION_REQUIRED,
            ErrorResponse::TooManyRequests { .. } => StatusCode::TOO_MANY_REQUESTS,
            ErrorResponse::RequestHeaderFieldsTooLarge { .. } => {
                StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
            }
            ErrorResponse::UnavailableForLegalReasons { .. } => {
                StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS
            }
            ErrorResponse::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::NotImplemented { .. } => StatusCode::NOT_IMPLEMENTED,
            ErrorResponse::BadGateway { .. } => StatusCode::BAD_GATEWAY,
            ErrorResponse::ServiceUnavailable { .. } => StatusCode::SERVICE_UNAVAILABLE,
            ErrorResponse::GatewayTimeout { .. } => StatusCode::GATEWAY_TIMEOUT,
            ErrorResponse::HttpVersionNotSupported { .. } => StatusCode::HTTP_VERSION_NOT_SUPPORTED,
            ErrorResponse::VariantAlsoNegotiates { .. } => StatusCode::VARIANT_ALSO_NEGOTIATES,
            ErrorResponse::InsufficientStorage { .. } => StatusCode::INSUFFICIENT_STORAGE,
            ErrorResponse::LoopDetected { .. } => StatusCode::LOOP_DETECTED,
            ErrorResponse::NotExtended { .. } => StatusCode::NOT_EXTENDED,
            ErrorResponse::NetworkAuthenticationRequired { .. } => {
                StatusCode::NETWORK_AUTHENTICATION_REQUIRED
            }
        }
    }
}
