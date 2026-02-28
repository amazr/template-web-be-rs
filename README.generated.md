# {{project-name}}

Axum backend scaffold with:

- OpenAPI routes (`utoipa` + Swagger UI)
- SeaORM entities/store layer
- SeaORM migration crate wired at startup
- OpenTelemetry tracing + request trace IDs
- derive-driven error-to-response mapping

## Quick start

1. Copy env template:
   ```sh
   cp .env.example .env
   ```
2. Run the app:
   ```sh
   cargo run
   ```
3. Open Swagger UI:
   - http://127.0.0.1:3000/swagger-ui

## Project structure

- `src/main.rs`: top-level orchestration
- `src/bootstrap`: app/database/middleware/observability/server setup
- `src/api`: route modules and API schemas
- `src/store`: data access layer
- `src/entities`: SeaORM entities
- `src/errors.rs`: app error enum using `#[response(...)]`
- `migration`: SeaORM migration crate and CLI
- `api-error`: shared `ErrorResponse` type
- `api-error-derive`: `ApiError` derive macro

## Error mapping pattern

```rust
#[derive(Debug, derive_more::From, api_error_derive::ApiError)]
pub enum Error {
    #[from]
    #[response(InternalServerError)]
    Db(sea_orm::DbErr),

    #[response(NotFound)]
    UserNotFound,
}
```

Each variant must have `#[response(...)]`. The derive generates `Display`, `std::error::Error`, and `IntoResponse`.
