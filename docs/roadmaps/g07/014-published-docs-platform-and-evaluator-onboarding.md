# g07.014 — Published Docs Platform and Evaluator Onboarding

Status: Completed
Updated: 2026-03-14

## Objective

Document the GPUI adapter for external review. Provide integration guides,
token reference, and component coverage evidence.

## Deliverables

### Crate documentation

The `poodle-gpui` crate includes:

- **Module-level docs** (`lib.rs`) — Architecture overview, adapter pattern diagram
- **Public API docs** — `GpuiAdapter`, `GpuiThemeProvider`, `GpuiElementHandle`, style types
- **Demo app module** (`demo_app.rs`) — Screen family documentation
- **Reference app** (`examples/reference_app.rs`) — Integration walkthrough

### Component coverage documentation

| Document | Location | Content |
|----------|----------|---------|
| g07 README | `docs/roadmaps/g07/README.md` | Milestone status, dependency shape |
| Parity report | `docs/roadmaps/g07/011-*` | Full 118-component coverage matrix |
| Milestone docs | `docs/roadmaps/g07/001-013-*` | Per-milestone deliverables and verification |

### Evaluator onboarding path

1. **Read** `docs/roadmaps/g07/README.md` for generation overview
2. **Run** `cargo test` in `packages/gpui/adapter/` — 145 tests
3. **Run** `cargo run --example reference_app` — see adapter in action
4. **Review** `src/demo_app.rs` — 6 screen families exercised
5. **Check** parity report in `011-*` — all 118 specs covered

### Token reference

Tokens are documented in the contract crates:
- `poodle-tokens/src/lib.rs` — re-exports semantic, typed, density modules
- `poodle-tokens/src/typed/semantic.rs` — 76 typed token constants
- `poodle-tokens/src/semantic.rs` — string-form token constants

### Integration guide summary

```rust
// 1. Create adapter
let adapter = GpuiAdapter::new(GpuiThemeProvider::default());

// 2. Build a spec
let spec = ButtonSpec::new();
let style = StyleDescriptor::new();

// 3. Render through the adapter
let handle = adapter.render(&spec, &style, adapter.theme());
// handle.element_id = "button-anonymous"
// handle.spec_type = "ButtonSpec"
```

## Verification

All documentation is in-repo and accessible via standard Rust doc tooling:
- `cargo doc --open` generates browsable API docs
- Example is runnable without external dependencies
- Milestone documents provide full audit trail
