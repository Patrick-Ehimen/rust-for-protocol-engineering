# Ultimate To-Do List: 36 Projects to Rust Mastery

This is the hyper-granular breakdown for the combined roadmap.

## Month 1: The Gauntlet (16 Projects)
### Week 1: Foundations
- [x] **1. Universal Converter**: Use `std::io` and `f64`.
- [x] **2. Guessing Game**: Implement "Limited Tries" and "High Score" file.
- [ ] **3. Crypto Price Tracker**: Use `reqwest` and `serde_json` to fetch Binance API.
- [ ] **4. System Monitor TUI**: Use `sysinfo` and `ratatui` for a live graph.
- [ ] **Bonus (Ethereum):** Read the [Ethereum Whitepaper](https://ethereum.org/en/whitepaper/) and summarize the Account Model vs UTXO.

### Week 2: Data & Files
- [ ] **5. Markdown Parser**: Regex-based conversion of `#` and `*`.
- [ ] **6. Todo CLI**: Implement `JSON` serialization for persistence.
- [ ] **7. Log Anonymizer**: Replace IP addresses and Emails in logs with `[MASKED]`.
- [ ] **8. Static Site Generator**: Watch a folder for `.md` files and output `.html`.
- [ ] **Bonus (Ethereum):** Implement a basic **RLP Serializer/Deserializer** for a mock Ethereum transaction.

### Week 3: Abstractions
- [ ] **9. Character Generator**: Use a `Generate` trait for different classes (Mage, Warrior).
- [ ] **10. Custom Error Logger**: Use `thiserror` for domain-specific errors.
- [ ] **11. File Vault**: Use `chacha20poly1305` for secure local storage.
- [ ] **12. HTTP Health Checker**: Use `tokio` to ping 10 URLs concurrently.
- [ ] **Bonus (Ethereum):** Research **Binary Merkle Trees vs Patricia Tries**; implement a 2-level mock Trie.

### Week 4: Networking
- [ ] **13. TCP Echo Server**: Handle concurrent clients with `std::thread`.
- [ ] **14. Parallel Port Scanner**: Use `mpsc` to collect results from 100 threads.
- [ ] **15. TCP Load Balancer**: Implement Round-Robin distribution.
- [ ] **16. Backup Script**: Use `walkdir` to recursively copy modified files.
- [ ] **Bonus (Ethereum):** Study `discv5` specs and simulate a DHT node join/leave event.

---

## Phase 1: Data Analytics (Projects 17-21)
- [ ] **17. Vault-CLI**: Master `clap` subcommands.
- [ ] **18. Vault-CLI Plus**: Integrate `keyring` crate for OS-level secret storage.
- [ ] **19. Chain-Indexer**: Use `alloy` to sync blocks from Eth Sepolia.
- [ ] **20. Multi-Chain Indexer**: Concurrent syncing into a single DB. Implement MPT proof verification for state access.
- [ ] **21. Portfolio CLI**: Build a "Tax Report" generator. Study the **Engine API** for CL-EL sync.

---

## Phase 2: Networking & DeFi (Projects 22-27)
- [ ] **22. RPC Proxy**: Implement basic GET request forwarding.
- [ ] **23. High-QPS Proxy**: Add `redis` caching. Implement the `eth_getProof` endpoint verification.
- [ ] **24. Gossip Node**: Build a "Join/Leave" peer discovery mechanism.
- [ ] **25. P2P File Sharer**: Implement piece-by-piece file transfer. Compare with `devp2p`'s RLPx transport.
- [ ] **26. Mempool Watcher**: Detect "Large Whale Transfers". Study **EIP-1559** gas dynamics.
- [ ] **27. Arbitrage Dashboard**: Stream prices via WebSockets.

---

## Phase 3: Core & VMs (Projects 28-33)
- [ ] **28. Block Explorer API**: Write complex SQL for "Token Holder Lists".
- [ ] **29. Explorer TUI**: Build a dashboard to view the API data in terminal.
- [ ] **30. Rust-VM**: Implement `Push`, `Add`, `Mul`, `Stop`. Add `gas` costs for each.
- [ ] **31. Gas-VM**: Implement **SSZ Merkleization** for state roots.
- [ ] **32. Consensus Engine**: Implement a "Leader Election" timer. Study the **Beacon Chain** lifecycle.
- [ ] **33. BFT Governance**: Implement "Weighted Voting". Deep-dive into **Casper FFG** finalization rules.

---

## Phase 4: Protocol Level (Projects 34-36)
- [ ] **34. Rollup Sequencer**: Implement a `state_root` calculation. Integrate with **Ethereum as a Data Availability layer**.
- [ ] **35. ZK-Verifier**: Implement a service that accepts Groth16 proofs.
- [ ] **36. PoS Node**: Final integration. Model the **LMD GHOST** fork choice rule.
