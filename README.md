# DAO Vote Verifier 🔐

A privacy-preserving voting system using Zama's Fully Homomorphic Encryption (FHE). Votes remain encrypted during tallying, ensuring voter privacy while providing accurate results.

## Features
- Vote encryption using TFHE-rs
- Homomorphic vote tallying
- Vote storage in encrypted format
- Individual votes never decrypted
- CLI interface for easy operation

## Tech Stack
- Rust
- Zama tfhe-rs (v0.4.0)
- FHE parameters: `PARAM_MESSAGE_2_CARRY_2_COMPACT_PK`

## Installation

1. Install Rust nightly:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup default nightly
