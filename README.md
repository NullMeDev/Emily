# Payment Processor - Rust Edition

A high-performance payment gateway testing system written in Rust, converted from Python.

## Features

- **7 Payment Gateways:**
  - Charge1: Blemart (Stripe PM + WooCommerce) - $4.99 USD
  - Charge2: District People (Stripe PM + Dynamic scraping) - €69.00 EUR
  - Charge3: Saint Vinson (GiveWP + Stripe Payment Intent) - $2.00 USD
  - Charge4: BGD Fresh Milk (Registration + Stripe PM) - $6.50 CAD
  - Charge5: Staleks Florida (Stripe Sources API) - $0.01 USD
  - Braintree: Braintree payment gateway
  - CC Foundation: Charitable WordPress plugin (Sky.py) - $1.00 USD

- **Core Features:**
  - ✅ Proper token generation and parsing
  - ✅ Clear response categorization (Charged/Declined/Insufficient Funds)
  - ✅ Session management and cookie handling
  - ✅ Dynamic field scraping
  - ✅ Batch processing support
  - ✅ Telegram bot integration (successful hits only)
  - ✅ BIN lookup
  - ✅ Luhn-valid card generation

## Project Structure

```
payment-processor-rust/
├── src/
│   ├── models/          # Data models (Card, Response, etc.)
│   ├── utils/           # Utilities (validators, generators, HTTP client)
│   ├── gateways/        # Payment gateway implementations
│   ├── services/        # Services (token manager, parser, BIN lookup)
│   └── bot/             # Telegram bot integration
├── Cargo.toml
└── README.md
```

## Development Phases

### ✅ Phase 1: Core Infrastructure (COMPLETE)
- Card models and validation
- Payment response types
- Luhn algorithm implementation
- User data generators
- HTTP client with session management

### 🚧 Phase 2: Payment Gateway Core (IN PROGRESS)
- Gateway trait definition
- Token manager (fresh token generation & parsing)
- Response parser (categorization logic)
- Dynamic field scraper

### ⏳ Phase 3: Gateway Implementations
- Implement all 7 payment gateways
- Each gateway as separate module
- Unified interface via trait

### ⏳ Phase 4: Telegram Bot Integration
- Command handlers
- File batch processing
- **Only successful hits sent to Telegram:**
  - ✅ Charged
  - ✅ CCN Live
  - 🔑 3DS/Action Required
  - 💰 Insufficient Funds
  - ❌ Declined (NOT sent)
  - ⚠️ Error (NOT sent)

## Building

```bash
cd payment-processor-rust
cargo build --release
```

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

## License

Private project - All rights reserved
