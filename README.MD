# Event-Driven Wallet Service

This is a **Rust backend service** simulating a token wallet with an **event-driven architecture** using **Axum**, **PostgreSQL**, and **RabbitMQ**. It supports asynchronous transfer processing, balance queries, and transfer history tracking.

---

## Quick Start

Get the service running and test it with a few commands.

### Prerequisites
- **Docker** and **Docker Compose** installed.
- **Rust** and **Cargo** for building and running migrations (if not using Docker).
- **SQLx CLI** for migrations: `cargo install sqlx-cli`.

### Steps
```bash
# 1. Build and start services (PostgreSQL, RabbitMQ, and the Rust app)

docker-compose up --build

```

# 2. Run database migrations
```bash

sqlx migrate run

```

# 3. Queue a transfer
```bash

curl -X POST http://localhost:3000/transfer \
  -H "Content-Type: application/json" \
  -d '{"from": "0x1234567890abcdef1234567890abcdef12345678", "to": "0xabcdef1234567890abcdef1234567890abcdef12", "amount": 100}'

```

# 4. Check 
```bash

curl http://localhost:3000/balance/0x1234567890abcdef1234567890abcdef12345678


```

# 5. View transfer history
```bash

curl http://localhost:3000/transfers/0x1234567890abcdef1234567890abcdef12345678


```


The API is available at `http://localhost:3000`.

### Endpoints
| Method | Endpoint                              | Description                                                                 | Request Body (if applicable)                     | Response                             |
|--------|---------------------------------------|-----------------------------------------------------------------------------|------------------------------------------------|--------------------------------------|
| POST   | `/transfer`                          | Queues a token transfer (processed asynchronously).                         | `{ "from": "0xabc...", "to": "0xdef...", "amount": 100 }` | 202 Accepted or error message       |
| GET    | `/balance/{address}`                  | Returns the balance for the specified wallet address as plain text.         | -                                              | Balance (e.g., `100`)               |
| GET    | `/transfers/{address}`               | Returns a JSON array of transfer history for the specified wallet address.  | -                                              | JSON array (e.g., `[{...}, {...}]`) |

### Notes
- **Wallet Creation**: Wallets are created implicitly with a balance of `0` upon first transfer or query.
- **Asynchronous Processing**: Transfers are queued in RabbitMQ and processed asynchronously. The API returns `202 Accepted` immediately upon successful queuing.
- **Error Handling**: Insufficient balance results in an aborted transfer, logged in the system.
- **Security**: This implementation lacks authentication. In production, add API keys, signatures, or other authentication mechanisms.
- **Database Initialization**: Run `sqlx migrate run` to initialize the database schema. To seed initial balances, modify the migration scripts or manually insert data into the database.
- **Testing**: Basic test cases are included. Expand them to cover edge cases like concurrent transfers or invalid inputs.
- **Stretch Goals**: Features like batch transfers or advanced error reporting are not implemented for brevity.

---

## Setup Details
- **Tech Stack**:
    - **Rust**: Backend logic using the Axum web framework.
    - **PostgreSQL**: Stores wallet balances and transfer history.
    - **RabbitMQ**: Handles event-driven transfer processing.
- **Dependencies**:
    - Managed via `Cargo.toml` for Rust dependencies.
    - External services (PostgreSQL, RabbitMQ) are configured in `docker-compose.yml`.
- **Configuration**:
    - Environment variables (e.g., database URL, RabbitMQ connection) are set in `.env` or `docker-compose.yml`.
    - Example `.env`:
      ```bash
      DATABASE_URL=postgres://user:password@localhost:5432/wallet
      RABBITMQ_URL=amqp://guest:guest@localhost:5672
      ```

## Running Without Docker
1. Ensure PostgreSQL and RabbitMQ are running locally or on a remote server.
2. Set environment variables in a `.env` file or your shell.
3. Install dependencies: `cargo build`.
4. Run migrations: `sqlx migrate run`.
5. Start the server: `cargo run`.

## Testing
- Run unit and integration tests: `cargo test`.
- Expand tests to cover edge cases (e.g., negative amounts, invalid addresses).

## Security Considerations
- **Authentication**: Add API keys, JWT, or blockchain-style signatures for production.
- **Input Validation**: Ensure addresses are valid (e.g., 40-character hex strings prefixed with `0x`).
- **Rate Limiting**: Implement to prevent abuse.
- **Logging**: Sensitive data (e.g., addresses) should be masked in logs.

## Troubleshooting
- **Service Not Starting**: Check Docker logs (`docker-compose logs`) for errors.
- **Migration Fails**: Verify the `DATABASE_URL` and PostgreSQL connectivity.
- **Transfers Not Processing**: Ensure RabbitMQ is running and the `RABBITMQ_URL` is correct.
- **API Errors**: Check server logs for detailed error messages.

---

## Contributing
- Fork the repository.
- Submit pull requests with clear descriptions of changes.
- Follow Rust coding conventions and include tests for new features.

## License
MIT License. See `LICENSE` for details.