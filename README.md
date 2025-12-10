# Acme Data Dash

A data quality dashboard for monitoring data sources and running data checks.

## Features

- **Data Quality Checks**: Define and run data quality checks against your data sources
- **Data Sources Management**: Configure connections, secrets, and bind them into data sources
- **Dashboard UI**: Web-based interface for running checks and viewing results
- **Multiple Connection Types**: Support for databases (ODBC), APIs (OpenAPI), and file-based sources
- **Secret Management**: Secure storage of credentials with environment variables and file-based options
- **History Tracking**: View historical check results and statuses

## Prerequisites

- Rust 1.70 or later (install from [rustup.rs](https://rustup.rs/))
- Node.js and npm (for UI development)
- SQLite (bundled with the project)

## Setup

### 1. Database Setup

Initialize the database with the schema and seed data:

```bash
./scripts/reset_db.sh
```

### 2. Prepare SQLx Query Cache

SQLx requires either a database connection or cached query metadata for compile-time verification. After setting up the database, prepare the query cache:

```bash
# Install sqlx-cli if not already installed
cargo install sqlx-cli --no-default-features --features sqlite

# Set DATABASE_URL and prepare queries
export DATABASE_URL=sqlite:data_dash.db
cargo sqlx prepare
```

This generates a `.sqlx` directory with cached query metadata that should be committed to version control.

### 3. Build the UI

```bash
cd ui
npm install
npm run build
cd ..
```

## Using Just (Command Runner)

This project uses [`just`](https://github.com/casey/just) as a command runner for common tasks. If you don't have it installed:

```bash
# macOS
brew install just

# Other platforms: see https://github.com/casey/just#installation
```

### Available Commands

```bash
# Run the application in development mode (resets DB, starts backend and frontend)
just dev

# Run benchmarks and generate performance graph
just benchmark
```

See the `Justfile` for all available commands.

## Development Workflow

### Before Committing

Always run these commands before committing to ensure code quality:

```bash
# 1. Format your code
cargo fmt

# 2. Check for compilation errors
cargo build

# 3. Run clippy for linting (with warnings as errors)
cargo clippy --all-targets --all-features -- -D warnings

# 4. Run all tests
cargo test --all-features

# 5. Verify formatting is correct
cargo fmt -- --check
```

### Complete Pre-Commit Check

Run all checks at once:

```bash
cargo fmt && \
cargo build && \
cargo clippy --all-targets --all-features -- -D warnings && \
cargo test --all-features && \
cargo fmt -- --check
```

## Running the Application

### Development Mode

**Using Just (Recommended):**

```bash
just dev
```

This automatically:
- Resets the database
- Starts the backend server on `http://localhost:3000`
- Starts the UI dev server on `http://localhost:5173`

**Manual Mode:**

```bash
# Start the backend server
cargo run

# In another terminal, start the UI development server
cd ui
npm run dev
```

The API will be available at `http://localhost:3000` and the UI at `http://localhost:5173`

### Production Mode

```bash
# Build the UI
cd ui
npm run build
cd ..

# Run the application (serves both API and UI)
cargo run --release
```

## Cargo Commands Reference

### Check for compile errors

```bash
cargo check
```

### Format code

```bash
# Format all code
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check
```

### Build binaries

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Run tests

```bash
# Run all tests
cargo test

# Run tests with all features enabled
cargo test --all-features

# Run specific test
cargo test test_name
```

### Run benchmarks

Uses the `criterion` library:

```bash
cargo bench
```

### Build documentation

```bash
cargo doc --open
```

### Linting with Clippy

```bash
# Run clippy with warnings as errors
cargo clippy --all-targets --all-features -- -D warnings
```

## Database Management

### Reset Database

To reset the database to its initial state:

```bash
./scripts/reset_db.sh
```

This will:
1. Remove the existing `data_dash.db`
2. Create a fresh database with the schema from `scripts/schema.sql`
3. Seed initial data from `scripts/seed_data.sql`

### Update Query Cache

After modifying database queries in the code, update the SQLx query cache:

```bash
export DATABASE_URL=sqlite:data_dash.db
cargo sqlx prepare
```

## Project Structure

```
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library exports
│   ├── api/              # REST API endpoints
│   ├── checks/           # Data quality check definitions
│   ├── connections.rs    # Connection management
│   ├── db.rs            # Database layer
│   └── secrets.rs       # Secret storage implementations
├── ui/                   # Svelte frontend application
├── scripts/             # Database scripts
├── benches/             # Performance benchmarks
└── tests/               # Integration tests
```

## Troubleshooting

### SQLx Compilation Errors

If you see errors like "set `DATABASE_URL` to use query macros online":

1. Ensure the database exists: `./scripts/reset_db.sh`
2. Set the DATABASE_URL: `export DATABASE_URL=sqlite:data_dash.db`
3. Prepare queries: `cargo sqlx prepare`

### UI Not Loading

If the UI doesn't load:

1. Build the UI: `cd ui && npm run build`
2. Ensure `ui/dist` directory exists
3. Restart the backend server

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines and workflow.
