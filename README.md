# mkvss

> A high-concurrency, persistent REST API server built from scratch in Rust

This is an educational-purpose implementation of a multi-threaded web server.
This project implements the core HTTP protocol (parts only needed for REST API),
routing logic, and thread management manually using Rust's standard library.

It uses SQLite for persistence, running in WAL (Write-Ahead Logging) mode to support high-concurrency access.

## Specification

- SQL setup

```sql
CREATE TABLE IF NOT EXISTS store (
    key TEXT PRIMARY KEY,
    value TEXT,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

## API reference

| Method | Endpoint      | Description         | Request Body                        |
|--------|--------------|---------------------|-------------------------------------|
| GET    | /keys/{id}   | Retrieve a value    | N/A                                 |
| POST   | /keys        | Create a new key    | `{"key": "...", "value": "..."}`   |
| PUT    | /keys/{id}   | Update existing key | Raw Text or JSON                    |
| DELETE | /keys/{id}   | Delete a key        | N/A                                 |

## Usage examples

1. Create a Key (JSON)

```bash
curl -X POST -H "Content-Type: application/json" \
     -d '{"key": "rust", "value": "fast_and_safe"}' \
     http://localhost:8080/keys
```

2. Retrieve a key

```bash
curl http://localhost:8080/keys/rust

# Output: fast_and_safe
```

3. Update a key (Raw Text)

```bash
curl -X PUT -d "new_value" http://localhost:8080/keys/rust
```

## Performance

Benchmarked using `oha` (Oha HTTP Load Generator) on my PC:

- Concurrency: 50
- Duration: 10 sec
- Database: SQLite in WAL mode

Result:
- Average: 2.0562ms/sec
- P99.00: 4.9463ms
- P99.90: 6.0222ms
- P99.99: 7.0840ms
- Success rate: approx. 99.99093%

<img width="821" height="1153" alt="image" src="https://github.com/user-attachments/assets/49dfeac4-1820-474e-a910-690b421abb99" />
