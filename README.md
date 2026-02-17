# CryptoWrap

CryptoWrap is a payment gateway/processor architected for secure, reliable, and seamless cryptocurrency transactions.
Providing a unified interface for both inbound and outbound payments. The project prioritizes simplicity and ease of integration, offering a lightweight and fast solution that operates as an extensible wrapper (API layer) for various blockchains.

### Coins
- Monero
- Litecoin (planned, not yet implemented)

#### Features
- HTML page (customizable template) for accepting crypto payments in 3 different modes:
1. WebSocket (fastest, default)
2. HTTPS polling (more reliable for unstable connections, fallback)
3. No-JavaScript

- Accept and store coins in isolated `virtual` accounts. (allows track of `dynamic` amounts)
Send/withdraw without locks or delays of other users' coins. (e.g. `output` management)
Ideal for systems with multiple users, where funds must be safely separated.


## Technology Stack

This project is built using a robust and modern technology stack, orchestrated within Docker containers for easy deployment and scalability:

- **Reverse Proxy/Load Balancer:** Nginx (default) / Nginx + HAProxy
- **Backend:** Rust (Axum Web framework)
- **Database Interaction & Migrations:** Sea-ORM
- **Database:** PostgreSQL

## License

This project is open-source and licensed under the Affero General Public License (AGPL), promoting freedom and encouraging the sharing of modifications or extended versions, even when used as a web service.
