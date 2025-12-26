# Ultimate To-Do List: 36 Projects to Rust Mastery

This is the hyper-granular breakdown for the combined roadmap.

## Month 1: The Gauntlet (16 Projects)
### Week 1: Foundations
- [ ] **1. Universal Converter**: Use `std::io` and `f64`.
- [ ] **2. Guessing Game**: Implement "Limited Tries" and "High Score" file.
- [ ] **3. Crypto Price Tracker**: Use `reqwest` and `serde_json` to fetch Binance API.
- [ ] **4. System Monitor TUI**: Use `sysinfo` and `ratatui` for a live graph.

### Week 2: Data & Files
- [ ] **5. Markdown Parser**: Regex-based conversion of `#` and `*`.
- [ ] **6. Todo CLI**: Implement `JSON` serialization for persistence.
- [ ] **7. Log Anonymizer**: Replace IP addresses and Emails in logs with `[MASKED]`.
- [ ] **8. Static Site Generator**: Watch a folder for `.md` files and output `.html`.

### Week 3: Abstractions
- [ ] **9. Character Generator**: Use a `Generate` trait for different classes (Mage, Warrior).
- [ ] **10. Custom Error Logger**: Use `thiserror` for domain-specific errors.
- [ ] **11. File Vault**: Use `chacha20poly1305` for secure local storage.
- [ ] **12. HTTP Health Checker**: Use `tokio` to ping 10 URLs concurrently.

### Week 4: Networking
- [ ] **13. TCP Echo Server**: Handle concurrent clients with `std::thread`.
- [ ] **14. Parallel Port Scanner**: Use `mpsc` to collect results from 100 threads.
- [ ] **15. TCP Load Balancer**: Implement Round-Robin distribution.
- [ ] **16. Backup Script**: Use `walkdir` to recursively copy modified files.

---

## Phase 1: Data Analytics (Projects 17-21)
- [ ] **17. Vault-CLI**: Master `clap` subcommands.
- [ ] **18. Vault-CLI Plus**: Integrate `keyring` crate for OS-level secret storage.
- [ ] **19. Chain-Indexer**: Use `alloy` to sync blocks from Eth Sepolia.
- [ ] **20. Multi-Chain Indexer**: Concurrent syncing of Eth and Solana into a single DB.
- [ ] **21. Portfolio CLI**: Build a "Tax Report" generator based on indexed trades.

---

## Phase 2: Networking & DeFi (Projects 22-27)
- [ ] **22. RPC Proxy**: Implement basic GET request forwarding.
- [ ] **23. High-QPS Proxy**: Add `redis` caching for `eth_getBalance`.
- [ ] **24. Gossip Node**: Build a "Join/Leave" peer discovery mechanism.
- [ ] **25. P2P File Sharer**: Implement piece-by-piece file transfer.
- [ ] **26. Mempool Watcher**: Detect "Large Whale Transfers" in real-time.
- [ ] **27. Arbitrage Dashboard**: Stream Uniswap vs Sushiswap prices via WebSockets.

---

## Phase 3: Core & VMs (Projects 28-33)
- [ ] **28. Block Explorer API**: Write complex SQL for "Token Holder Lists".
- [ ] **29. Explorer TUI**: Build a dashboard to view the API data in terminal.
- [ ] **30. Rust-VM**: Implement `Push`, `Add`, `Mul`, `Stop`.
- [ ] **31. Gas-VM**: Add memory limits and instruction costs.
- [ ] **32. Consensus Engine**: Implement a "Leader Election" timer.
- [ ] **33. BFT Governance**: Implement "Weighted Voting" based on mock stakes.

---

## Phase 4: Protocol Level (Projects 34-36)
- [ ] **34. Rollup Sequencer**: Implement a `state_root` calculation.
- [ ] **35. ZK-Verifier**: Implement a service that accepts Groth16 proofs.
- [ ] **36. PoS Node**: Link P2P messaging to the VM execution loop.
