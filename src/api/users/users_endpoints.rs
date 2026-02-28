use axum::{
    Json,
    extract::{Path, State},
};
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    api::{created::Created, users::user::{NewUser, User}},
    errors::{Error, ErrorResponse, Result},
    state::AppState,
    store::user_store::StoredUser,
};

#[utoipa::path(
    post,
    path = "/create",
    request_body = NewUser,
    responses(
        (status = 201, description = "User created", body = User),
        ErrorResponse,
    ),
    tag = "users"
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(new_user): Json<NewUser>,
) -> Result<Created<Json<User>>> {
    let StoredUser { id, name, email } = state.user_store().store_user(new_user).await?;
    Ok(Created(Json(User { id, name, email })))
}

#[utoipa::path(
    get,
    path = "/list",
    responses(
        (status = 200, description = "List users", body = Vec<User>),
        ErrorResponse,
    ),
    tag = "users"
)]
pub async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<User>>> {
    let users = state
        .user_store()
        .list_users()
        .await?
        .into_iter()
        .map(|u| User {
            id: u.id,
            name: u.name,
            email: u.email,
        })
        .collect::<Vec<_>>();
    Ok(Json(users))
}

#[utoipa::path(
    delete,
    path = "/delete/{id}",
    responses(
        (status = 200, description = "User deleted"),
        ErrorResponse,
    ),
    tag = "users"
)]
pub async fn delete_user(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<()> {
    let store = state.user_store();
    if store.get_user(id).await?.is_none() {
        return Err(Error::UserNotFound);
    }
    store.delete_user(id).await
}

pub fn router() -> OpenApiRouter<AppState> {
    info!("Registering user routes");
    OpenApiRouter::new()
        .routes(routes!(create_user))
        .routes(routes!(list_users))
        .routes(routes!(delete_user))
}
