# README.md

## Event-Driven Wallet Service

This is a Rust backend service simulating a token wallet with event-driven architecture using Axum, PostgreSQL, and RabbitMQ.

### Setup

1. Install Docker and Docker Compose.

2. Run `docker-compose up --build`.

3. Start Migration `sqlx migrate run`.

4.1 The API is available at `http://localhost:3000`.

### Endpoints

- POST /transfer: JSON body `{ "from": "0xabc", "to": "0xdef", "amount": 100 }` – queues transfer.

- GET /balance/{address}: Returns balance as text.

- GET /transfers/{address}: Returns JSON array of transfers.

### Notes

- Wallets are created implicitly with balance 0.

- Transfers are processed asynchronously; API returns 202 Accepted immediately.

- Error handling: Insufficient balance aborts transfer (logged).

- Stretch goals not implemented for brevity.

- Tests: Basic structure provided; expand as needed.

- Security: No auth; in production, add API keys or signatures.

- To init balances, you can manually insert into DB or extend API.