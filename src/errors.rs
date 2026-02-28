pub use api_error::ErrorResponse;
use api_error_derive::ApiError;
use derive_more::From;
use sea_orm::DbErr;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, ApiError)]
#[allow(dead_code)]
pub enum Error {
    #[response(NotFound)]
    UserNotFound,
    #[response(BadRequest)]
    RowsAffectedUnexpected { expected: u64, affected: u64 },

    // EXTERNAL ERRORS
    #[from]
    #[response(InternalServerError)]
    Db(DbErr),
}
