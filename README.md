# Solana On-Chain Calculator (Anchor)

A simple **Solana** smart contract (program) built with the **Anchor** framework in **Rust**.  
It allows basic arithmetic operations (addition and subtraction) on-chain and stores the last result in a persistent account.

This project was created as a beginner-friendly way to learn:
- Anchor framework basics
- Solana program development in Rust
- Account initialization and mutation
- Basic instruction handling

## Features

- `initialize` — Creates the calculator account and sets initial result to 0
- `add(a: u8, b: u8)` — Computes `a + b` and updates the stored result
- `sub(a: u8, b: u8)` — Computes `a - b` and updates the stored result
- Stores the payer's public key and the latest result on-chain

**Note:** Operations use `u8` (0–255). Overflow/underflow wraps around (e.g. 255 + 1 = 0, 0 - 1 = 255).

## Program ID
LEX2HNNLUG5v6XY7BHyga25HjpfPteQrqB5Df421N7B


## Tech Stack

- **Rust** + **Anchor** (~0.29 / 0.30+)
- **Solana** blockchain
- TypeScript + Mocha for tests
- Yarn (for client-side / test setup)

## Project Structure
solana-1/
├── Anchor.toml               # Anchor configuration
├── Cargo.toml                # Rust workspace config
├── Cargo.lock
├── programs/
│   └── solana-1/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs        # ← The actual Solana program (instructions + accounts)
├── tests/
│   └── solana-1.ts           # Mocha tests using Anchor TS client
├── migrations/
│   └── deploy.ts             # Deployment script (optional)
├── package.json
├── tsconfig.json
├── yarn.lock
└── README.md


## Getting Started

### Prerequisites

1. Install **Solana CLI** → https://docs.solana.com/cli/install-solana-cli-tools
2. Install **Anchor** → https://www.anchor-lang.com/docs/installation
3. Rust (via rustup)
4. Node.js + Yarn

```bash
# Quick install (if not already done)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
