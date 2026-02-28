# {{project-name}}

Axum backend template with:

- structured error handling and API result types
- OpenAPI generation with `utoipa` + Swagger UI
- SeaORM stores + entities layout
- SeaORM migration crate wired into app startup
- shared application state and background task hook
- derive-driven API error mapping via `#[response(...)]`

## Project structure

- `src/main.rs`: server bootstrap, middleware, OpenAPI, migration startup
- `src/bootstrap`: app wiring split by concern (app, db, middleware, observability, server)
- `src/api`: route modules and API schemas
- `src/store`: data access/store layer
- `src/entities`: SeaORM entities
- `migration`: SeaORM migration crate and CLI
- `api-error`: shared `ErrorResponse` type and status-code mapping (4xx/5xx)
- `api-error-derive`: `ApiError` derive macro for `IntoResponse`/`Display`/`std::error::Error`

## Error mapping workflow

`Error` variants are annotated once and converted automatically into HTTP responses.

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

The derive macro enforces `#[response(...)]` on every variant and generates:

- `impl core::fmt::Display` (debug style)
- `impl std::error::Error`
- `impl axum::response::IntoResponse` mapped through `api_error::ErrorResponse`

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

## Template usage

This repository is set up for `cargo-generate` so project-specific names are rendered automatically.

Generate a project from this template:

```sh
cargo generate --git https://github.com/{{github_owner}}/{{project-name}}
```

During generation, `cargo-generate` will prompt for `github_owner` and render project metadata like `Cargo.toml` repository URL.

## Smoke test the template

You can verify template changes still produce a compilable project by generating a temporary project and running `cargo check`:

```sh
./scripts/smoke-test-template.sh
```

Optional inputs:

- positional arg 1: generated project name
- env var `GITHUB_OWNER`: value used for the template placeholder

Example:

```sh
GITHUB_OWNER=acme ./scripts/smoke-test-template.sh acme-api
```

Notable templated behavior:

- `.env.example` uses `{{crate_name}}` in `RUST_LOG`, so log filters match the generated crate module path.
- OpenTelemetry tracer name uses `env!("CARGO_PKG_NAME")` in `src/main.rs`, so it always follows the generated package name at compile time.

After generation, replace the `users` API/store/entity/migration modules with your domain modules while keeping the same layering pattern.
