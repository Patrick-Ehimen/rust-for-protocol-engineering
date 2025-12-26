# Workspace Structure

This document outlines the recommended directory structure for your 10-month journey. Organizing your work from Day 1 will help you track progress and maintain a clean portfolio.

```text
.
├── README.md               # Project Hub
├── docs/                   # Roadmap & Checklists
│   ├── walkthrough.md      # Goal overview
│   ├── implementation.md   # Technical descriptions
│   ├── to-dos.md           # Weekly tasks
│   ├── STRUCTURE.md        # This file
│   └── task.md             # High-level tracker
│
├── learning/               # Study Notes & Small Exercises
│   ├── month-01/           # Month 1 Beginner notes
│   │   ├── notes.md        # Chapter summaries from "The Book"
│   │   └── sandbox/        # Throwaway learning experiments/snippets
│   └── ...                 # Sequential learning folders
│
├── specs/                  # Official Protocol Specifications
│   ├── yellow-paper.pdf    # Ethereum Execution Layer spec
│   ├── beacon-chain.md     # Consensus Layer specs
│   └── eips/               # Relevant Improvement Proposals
│
├── scripts/                # Task-specific Automation
│   ├── setup-node.sh       # Automation for local devnets
│   └── fetch-prices.py     # Data fetching utilities
│
├── research/               # Protocol Deep-Dives (Ethereum focus)
│   ├── execution-layer/    # EVM, MPT, Engine API notes
│   ├── consensus-layer/    # Gasper, Beacon Chain, SSZ
│   ├── networking/         # libp2p, devp2p, discv5
│   └── cryptography/       # ZK proofs, hashing, signatures
│
└── projects/               # The 36 Major Projects
    ├── month-01-gauntlet/  # 16 Mini-projects (Local folders)
    │   ├── 01-converter/
    │   ├── 02-guessing-game/
    │   └── ...
    │
    ├── phase-01-data/      # Git Submodules (Standalone Repos)
    │   ├── vault-cli-plus @[repo-hash]
    │   └── chain-indexer @[repo-hash]
    │
    ├── phase-02-systems/   # Git Submodules
    ├── phase-03-infra/     # Git Submodules
    └── phase-04-protocol/  # Git Submodules

## Folder Explanations

### Git Submodules
From **Month 2** onwards, each project is a heavy-duty system. Instead of being local folders, these will be **standalone Git repositories**.
- **Why?** This makes your portfolio look professional. Each project gets its own GitHub stars, CI/CD pipelines, and separate documentation.
- **Integration**: We link them to this roadmap repository using `git submodule add <repo-url>`.

### `sandbox/`
Think of this as your "drafting board." It’s for small, non-permanent code snippets that you write while reading *The Rust Book*. It prevents your `projects/` folder from being cluttered with hundreds of `main.rs` files just to test how a `match` statement works.

### `specs/`
Protocol engineering requires following strict rules. Store the Ethereum Yellow Paper, EIPs (Ethereum Improvement Proposals), and Consensus specs here for quick offline access.

### `scripts/`
In a submodule-based setup, this folder transitions from "project scripts" to **"workspace orchestration."**
- **Global Setup**: A script to verify you have all the tools needed for the 10-month journey (`rust-check.sh`).
- **Submodule Management**: A script to update all project repositories at once (`sync-all.sh`).
- **Provisioning**: Spin up global services like a shared Postgres or Redis instance via Docker that multiple projects might use.

## Setup Instructions
1.  **Keep it clean**: Avoid putting non-project specific code in the root.
2.  **Git integration**: Each project in `projects/` should eventually be a self-contained Cargo project (`cargo init`).
3.  **Documentation**: Use the `research/` folder as your "personal wiki" to store diagrams and explanations of complex Ethereum protocol topics.
