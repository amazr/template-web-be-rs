# Rust Backend Template

Axum backend template with:

- OpenAPI routing with `utoipa` + Swagger UI
- SeaORM stores/entities + migration crate
- OpenTelemetry tracing setup
- derive-driven API error mapping via `#[response(...)]`
- bootstrap split into `app`/`database`/`observability`/`server` components

## Create a new project from this template

1. Install cargo-generate (one time):
   ```sh
   cargo install cargo-generate
   ```
2. Generate a new project:
   ```sh
   cargo generate --git https://github.com/amazr/template-web-be-rs
   ```
3. Enter the generated folder and run it:
   ```sh
   cd <new-project>
   cp .env.example .env
   cargo run
   ```

During generation, this template replaces its own README with a generated-project README so the new repo starts with app-focused setup docs.

## Publish generated project to GitHub

After generating:

1. Create a new empty GitHub repository.
2. Initialize and push your generated project:
   ```sh
   git init
   git add .
   git commit -m "Initial project scaffold"
   git branch -M main
   git remote add origin https://github.com/<owner>/<repo>.git
   git push -u origin main
   ```

## Smoke test this template repo

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
