# Knowledge index

Every topic has exactly one owning file. Link to it; don't restate it.

| Topic | Owner |
| --- | --- |
| Vision, direction and non-goals | [vision.md](vision.md) |
| System shape and ownership | [architecture/001-poodle-system-shape.md](architecture/001-poodle-system-shape.md) |
| Product guardrails | [architecture/product-guardrails.md](architecture/product-guardrails.md) |
| Narrower architecture decisions | [architecture/](architecture/README.md) |
| Repository-wide normative rules | [specs/](specs/README.md) |
| Working rules: parity authority, specimens, validation budgets | [contracts/working-rules.md](contracts/working-rules.md) |
| Agent-local paths and worktrees | [contracts/agent-local-paths.md](contracts/agent-local-paths.md) |
| Internal writing style | [contracts/writing-style.md](contracts/writing-style.md) |
| How we release | [contracts/release.md](contracts/release.md) |
| GPUI capture research behind the held visual lane | [research/](research/gpui-offscreen-capture-feasibility.md) |
| Retired concepts | [retired.toml](retired.toml) |
| Open questions | [questions.md](questions.md) |

Product documentation for users or consumers lives outside `docs/knowledge/`:

| Product docs | Location |
| --- | --- |
| Component contracts: public inputs, states, events, accessibility, layout, tokens | [`docs/contracts/`](../contracts/README.md) |
| Cross-component contracts: overlays, native accessibility, shared control types | [`docs/contracts/`](../contracts/README.md) |
| Developer guides and pattern recipes | [`docs/guides/`](../guides/README.md) |
| Release notes | [`docs/release-notes/`](../release-notes/README.md) |

Executable evidence (generated receipts, ledgers and census output checked by
repository scripts) lives in [`docs/evidence/`](../evidence/README.md). It
records what was observed and is regenerated, never hand-edited.
