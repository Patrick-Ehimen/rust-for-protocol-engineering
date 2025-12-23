# Detailed To-Dos: 10-Month Rust Blockchain Journey

This document provides a week-by-week style breakdown of tasks to keep you on track.

## Month 1: Rust Beginner Bootcamp
- [ ] **Week 1: Setup & Syntax**
    - [ ] Install `rustup`, `cargo`, and VS Code (`rust-analyzer`).
    - [ ] Read Chapters 1-5 of [The Book](https://doc.rust-lang.org/book/).
    - [ ] Complete Rustlings: `intro`, `variables`, `functions`, `if`, `move_semantics`.
- [ ] **Week 2: Data Structures & Control Flow**
    - [ ] Read Chapters 6-9 of The Book.
    - [ ] Complete Rustlings: `structs`, `enums`, `strings`, `modules`, `error_handling`.
- [ ] **Week 3: Ownership & Lifetimes**
    - [ ] Read Chapter 10 and 15 of The Book.
    - [ ] Complete Rustlings: `generics`, `traits`, `lifetimes`.
- [ ] **Week 4: Project Zero**
    - [ ] Build a CLI that takes a string and prints it in Morse Code.
    - [ ] Complete all remaining Rustlings (up to `threads`/`macros`).

---

## Phase 1: Core & Data Tooling (Months 2-3)
### Project 1: Vault-CLI
- [ ] Initialize repo with `cargo init`.
- [ ] Add dependencies: `clap`, `aes-gcm`, `argon2`, `serde`, `rpassword`.
- [ ] Implement master password hashing with Argon2.
- [ ] Implement file-based storage for encrypted JSON.
- [ ] Implement CLI commands: `add`, `get`, `list`, `delete`.

### Project 2: Chain-Indexer (v1)
- [ ] Set up a local PostgreSQL instance (Docker is recommended).
- [ ] Use `alloy-rs` to connect to a public RPC (Infura/Alchemy).
- [ ] Create database migrations for `blocks` and `transactions`.
- [ ] Write a "worker" loop that fetches block `N` and saves it.
- [ ] Handle re-orgs/chain forks (basic logic: check block hash of `N-1`).

---

## Phase 2: Async & Networking (Months 4-5)
### Project 3: RPC Proxy
- [ ] Build a basic HTTP server using `axum`.
- [ ] Implement request forwarding to a real node.
- [ ] Add `moka` or `lru` crate for response caching.
- [ ] Configure `tower-governor` for rate limiting.

### Project 4: libp2p Gossip Node
- [ ] Spawn a libp2p Swarm with TCP transport.
- [ ] Implement `KeepAlive` and `mDNS` for local peer discovery.
- [ ] Integrate `Gossipsub` and create a "Chat" CLI to verify messages.

### Project 5: Mempool Watcher
- [ ] Connect to an Ethereum node via `WebSocket`.
- [ ] Subscribe to `newPendingTransactions`.
- [ ] Filter transactions in real-time by `gas_price` > `X`.

---

## Phase 3: Infrastructure & VMs (Months 6-7)
### Project 6: Block Explorer API
- [ ] Build sophisticated SQL queries for "Address Portfolio".
- [ ] Implement pagination for transaction lists.
- [ ] Add a caching layer (Redis) for high-traffic endpoints.

### Project 7: Rust-VM
- [ ] Define `Instruction` enum (Add, Sub, Push, Pop).
- [ ] Implement the `VirtualMachine` struct with `stack: Vec<u64>`.
- [ ] Add a `run()` loop that executes byte arrays.
- [ ] Implement `Gas` counter that decrements per instruction.

### Project 8: Consensus Engine
- [ ] Implement `Proposal` and `Vote` structs.
- [ ] Use `ed25519-dalek` to sign votes.
- [ ] Implement a simple "Pre-commit" -> "Commit" state machine.

---

## Phase 4: Scaling & L2s (Months 8-9)
### Project 9: Rollup Sequencer
- [ ] Implement a "World State" (HashMap of Address -> Balance).
- [ ] Create a `Transaction` type (From, To, Amount, Nonce).
- [ ] Build a `Batch` logic that computes the Merkle Root of 100 transactions.

### Project 10: Custom PoS Node
- [ ] Create the main `Node` entry point.
- [ ] Glue Project 4 (Networking) to Project 7 (VM).
- [ ] Ensure nodes can send each other Blocks and sync state.

---

## Month 10: Industry Readiness
- [ ] **GitHub Cleanup**: Add READMEs, Architects diagrams (Mermaid), and CI tests.
- [ ] **Open Source**: Submit a PR (even if just a doc fix) to `ethers-rs` or `solana`.
- [ ] **Mock Interviews**: Practice "System Design for a Distributed Sequencer".
