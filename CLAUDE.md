# CLAUDE.md — Instructions for Claude Code and Autonomous Agents

## Overview

This repository is **Trinity Git Orchestrator** — an MCP server for AI agents to control Git and GitButler via libgit2 + CLI.

**Repo:** https://github.com/gHashTag/trios

---

## Project Structure

```
trios/
├── crates/                 # Rust crates
│   ├── trios-server/      # Axum server (port 9005)
│   ├── trios-git/         # git2-rs bindings
│   ├── trios-gb/          # GitButler CLI integration
│   └── trios-ext/         # WASM browser extension
├── .trinity/
│   ├── prompts/           # Agent dispatch prompts
│   ├── agent-dispatch.md # Universal ONE-SHOT Agent Briefing (v1.0)
│   └── experience/        # Agent institutional memory
├── LAWS.md                # Constitutional document (v2.0)
└── CLAUDE.md              # This file
```

---

## Mandatory Read Order

1. **[.trinity/prompts/universal-agent-dispatch.md](.trinity/prompts/universal-agent-dispatch.md)** — Universal ONE-SHOT Agent Briefing
   - Complete 11-step PHI LOOP workflow embedded in single briefing
   - DONE checklist that blocks premature victory
   - HEARTBEAT format with timestamp and structured evidence
2. **[LAWS.md](LAWS.md)** — Constitutional stack, immutable laws L1-L9
3. **[CLAUDE.md](CLAUDE.md)** — This file — Project conventions

---

## Agent Dispatch

For autonomous agents, use the **universal agent dispatch prompt** located at:

**[.trinity/prompts/universal-agent-dispatch.md](.trinity/prompts/universal-agent-dispatch.md)**

This prompt contains:
- Complete PHI LOOP workflow (11 steps)
- All LAWS embedded (L1-L9)
- DONE checklist that blocks premature victory
- HEARTBEAT format for structured status reporting

**Usage:** Replace `{{ISSUE_NUMBER}}`, `{{ISSUE_TITLE}}`, and let agent choose `{{YOUR_SOUL_NAME}}` in step NAME.

---

## Engineering Workflow

### Development

```bash
# Build all crates
cargo build --all

# Run tests
cargo test --all
```

### Commit Protocol

**L6 (PUSH_FIRST) is absolute:** No commit is "done" until:
1. `cargo clippy --all-targets` = 0 warnings
2. `cargo test --all` = all pass
3. `git status` = 0 modified files
4. Commit visible on `github.com/gHashTag/trios`

### PR Requirements

Every PR must include:
- `Closes #{{ISSUE_NUMBER}}` in body
- Zero clippy warnings
- All tests passing
- Link to issue

---

## Architecture

### MCP Server

```
BrowserOS Agent
    │ MCP tool call (A2A)
    ▼
trios-server (port 9005, Axum)
    ├── trios-git   (git2-rs) ← stable git ops
    └── trios-gb    (CLI)      ← GitButler virtual branches
```

### Browser Extension

Location: `crates/trios-ext/extension/` (single extension tree)

---

## Quick Reference

### Common Commands

```bash
# Start MCP server
cargo run --bin trios-server

# Run GitButler integration tests
cargo test --package trios-gb

# Build WASM extension
wasm-pack build --target web crates/trios-ext
```

---

**Last Updated:** 2026-05-02
**Status:** Active
