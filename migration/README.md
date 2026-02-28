# Running migrator CLI

- Generate a new migration file
  ```sh
  cargo run -- generate MIGRATION_NAME
  ```
- Apply all pending migrations
  ```sh
  cargo run -- up
  ```
- Roll back last migration
  ```sh
  cargo run -- down
  ```
- Show migration status
  ```sh
  cargo run -- status
  ```
