rust-chat

- Rust-based chat/auth starter composed of separate crates: Rocket+SQLite backend, Raylib desktop client, shared DTOs, proc-macros, and small UI helpers.
- Run each crate directly (no top-level Cargo workspace).

Project Map
- server/: Rocket API on :3000; auth (signup/login/refresh) and user search; runs SQLite migrations at startup.
- client/: Raylib desktop UI; simple login/signup screens; points to http://localhost:3000.
- shared/: DTOs, response envelope, and auth/search payloads reused by server/client.
- macros/: Procedural macros for lightweight error handling and requests guards with db pool.
- ui/: Custom raylib based UI library.


Environment (server .env)
- DATABASE_URL: SQLite connection string, e.g. sqlite://db.sqlite
- JWT_ACCESS_KEY: base64-encoded secret for access tokens.
- JWT_REFRESH_KEY: base64-encoded secret for refresh tokens.
Generate secrets: `openssl rand -base64 32`

Run the Server
```bash
cd server
cargo run
```

Run the Client
```bash
cd client
cargo run
```