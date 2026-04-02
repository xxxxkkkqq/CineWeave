# CineWeave

CineWeave is a local-first, open-source foundation for conversational video editing, developed with Codex.

The project explores a practical architecture where a user describes editing intent in natural language and the system can turn that intent into deterministic project state, task graphs, timeline operations, UI commands, and render-oriented plans.

## Status

This repository is an alpha-stage engineering foundation, not a finished editor.

What is already working:

- Intent parsing and task graph compilation in Node
- A Rust `media-core` crate with deterministic project state
- Timeline operations with undo and redo
- Adapter request handling for desktop-shell style command payloads
- Persistent project documents with checkpointed history logs
- Tests covering Rust core behavior and Node adapter helpers

What is not here yet:

- A full desktop editor product
- Real media ingestion and render execution pipelines
- A stable public API commitment

## Repository Layout

```text
apps/
  cli/                 CLI entry point
  desktop-shell/       Local desktop shell and adapter-facing UI surface
crates/
  media-core/          Rust project model, persistence, adapter, timeline history
docs/
  architecture.md      System architecture notes
  literature-review.md Upstream review and feasibility notes
  project-plan.md      Delivery plan
packages/
  agent/               Prompt parsing and task graph compilation
  core/                Shared assertions and error helpers
  domain/              Canonical project and intent models
  project/             Project factories and capability presets
  render/              Render plan compilation
research/
  upstreams/           Local reference clones used for research only
tests/                 Node test coverage
```

## Prerequisites

- Node.js 24 or newer
- Rust stable

Windows note:

- On the original development machine, the GNU Rust toolchain was the reliable local path because the MSVC linker was unavailable.
- On Linux and GitHub Actions, standard stable Rust is expected to work.

## Getting Started

Install dependencies if you need the Node workspace environment:

```bash
npm install
```

Run the Node test suite:

```bash
npm test
```

Run the Rust test suite:

```bash
cargo test -p media-core
```

Run the CLI:

```bash
node apps/cli/src/index.js doctor
node apps/cli/src/index.js plan "Turn an interview into a faster 9:16 cut with subtitles, film look, and light glitch accents"
```

Run the Rust document demo:

```bash
cargo run -p media-core -- document-demo
```

If you are on Windows and need the GNU toolchain explicitly:

```bash
cargo +stable-x86_64-pc-windows-gnu test -p media-core
```

## Persistence Model

`media-core` persists project documents as two files:

- `snapshot.json`: a checkpoint/base state
- `history-log.json`: persisted `past` and `future` history entries for undo/redo recovery

This means project state can be restored across restarts without losing undo/redo stacks.

Legacy `snapshot + event log` documents are still accepted by the loader for compatibility.

## Design Direction

CineWeave is informed by several upstream projects, each strong in a different layer:

- AutoClip: content understanding and clip extraction workflows
- OpenCut: open-source timeline/editor concepts
- capcut-mate: subtitles, filters, effects, and draft semantics
- Dify: workflow orchestration patterns
- CLI-Anything: tool-surface design for agents

These repositories are kept under `research/upstreams/` as local references. They are not the product codepath of CineWeave itself.

## Development Notes

- CI runs both Node and Rust tests on every push and pull request.
- Build output and local dependency folders are ignored in `.gitignore`.
- The repository is licensed under MIT.

## License

MIT. See [LICENSE](./LICENSE).
