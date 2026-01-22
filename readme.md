# Solana Raffle Smart Contract

A fully functional raffle application built on the [Solana blockchain](https://solana.com/) using Rust and Anchor.

## Features
- Participants send SOL to join the raffle.
- Admin draws a winner randomly based on blockhash entropy.
- Winner receives all collected funds (pot).

## Requirements
- [Rust](https://www.rust-lang.org/)
- [Anchor CLI](https://book.anchor-lang.com/)

## Setup

1. **Install dependencies**:
```bash
cargo install --git https://github.com/coral-xyz/anchor.git --branch v0.34.0 anchor-cli
