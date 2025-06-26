# CRDT Research Project

A comprehensive research project comparing Conflict-free Replicated Data Type (CRDT) implementations for collaborative applications. This project evaluates **YJS** and **Automerge** through practical implementations in both TypeScript and Rust.

## Project Structure

```
├── automerge-crdt-rs/     # Rust implementation using Automerge
├── automerge-crdt-ts/     # TypeScript implementation using Automerge
├── y-crdt-rs/             # Rust implementation using YJS/y-crdt
├── yjs-crdt-ts/           # TypeScript implementation using YJS
├── comparison.md          # Detailed performance and feature comparison
└── crdt.md               # Technical analysis of CRDT approaches
```

## Research Findings

This project provides practical benchmarks and analysis comparing YJS and Automerge across multiple dimensions:

- **Performance**: YJS shows ~9% faster execution (0.646s vs 0.707s)
- **Memory Efficiency**: YJS produces smaller binary updates (92 bytes vs 158 bytes)
- **API Design**: Automerge offers explicit change tracking vs YJS's event-driven approach
- **Use Cases**: Different strengths for real-time collaboration vs audit-heavy applications

## Getting Started

### TypeScript Implementations

**YJS Implementation:**
```bash
cd yjs-crdt-ts
npm install
npm start
```

**Automerge Implementation:**
```bash
cd automerge-crdt-ts
npm install
npm start
```

### Rust Implementations

**Y-CRDT (YJS) Implementation:**
```bash
cd y-crdt-rs
cargo run
```

**Automerge Implementation:**
```bash
cd automerge-crdt-rs
cargo run
```

## Documentation

- **[comparison.md](comparison.md)**: Executive summary and detailed comparison of YJS vs Automerge
- **[crdt.md](crdt.md)**: Technical deep-dive into CRDT approaches for decentralized systems

## Key Takeaways

- **Choose YJS** for performance-critical real-time collaboration
- **Choose Automerge** for applications requiring rich change attribution and audit trails
- Both libraries provide robust conflict-free synchronization for collaborative applications

## Research Context

This research was conducted to evaluate CRDT solutions for decentralized knowledge graph applications, particularly those using IPFS and blockchain technologies for data storage and consensus.