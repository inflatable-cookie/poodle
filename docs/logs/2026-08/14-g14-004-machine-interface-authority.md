# 14 — g14.004 Machine Interface Authority (batch log)

Branch: `thread/g14-004-machine-interface-authority`
Date: 2026-08-13
Base: `230483fa`
Card: `docs/roadmaps/g14/batch-cards/004-machine-interface-authority.md`
Milestone: `g14.004`

Reset per Thread Reuse Protocol: `git fetch origin --prune`, branch created
from `origin/main` (this worktree cannot check out `main`). Baseline on
`230483fa`: `test:core`, `ci:rust`, `ci:web`, `git diff --check` all 0.

## What landed

One schema, `packages/contracts/headless/machine-interfaces.json`, generates
TS + Rust type declarations for the four pilots. Interface only: states,
events, effects, context, plus `PopoverInitialFocus` (used by context and
effects). No transitions, guards, or derivation.

Generator home is `poodle-codegen` (R1). Parallel CLI path, not an `IrModel`:

```
poodle-codegen --machine-interfaces <FILE> --out <DIR> --target machine-ts|machine-rust [--check]
```

`machine-ts` / `machine-rust` are select-only, like the shell. `ir:build` /
`ir:check` gained two runs. Shell targets untouched. `machines.json`
untouched.

Consumers re-export generated types. Public names unchanged. `HoverResult` /
`MenuResult` / `ModalResult` / `PopoverResult` stay one-line aliases over
`TransitionResult`. Menu list navigation (`MenuListMove`, `MenuListItem`)
stays hand-written — not the overlay machine.

`export type { X } from "./generated/..."` does not bind `X` in the file.
svelte-check (install-smoke) failed 39 errors on that. Fix: `import type`
then `export type`. `bun test` does not catch it.

## Plant proof

Authored-side plant: added `{ ts: "planted", rs: "Planted" }` to hover
states. `effigy ir:check` exit 1:

```
generated machine-interface artifacts are stale under packages/core/src/generated/machines (target machine-ts):
hover.ts (content drift)
```

Restored. `effigy ir:check` exit 0 (4 `machine-ts` + 4 `machine-rust`).

## LOC

Replaced type declarations (main, `*Result` aliases excluded): **199 lines**
(TS 78 + Rust 121).

Generated files: 243 lines including the 4-line header × 8 and rustfmt-skip.
Interface body (no header, skip, or blank): **173 lines**. Smaller than what
they replace.

## Findings

None of the four needed behaviour in the schema. Menu `disabled?: boolean` vs
Rust `disabled: bool` is `tsOptional` — type-shape, not a transition.
Modal/popover effect variant order in Rust now follows the schema (TS order);
match-by-name, not a rename.

## Acceptance

| Command | Exit |
|---|---|
| `effigy ci:rust` | 0 |
| `effigy test:core` | 0 (659) |
| `effigy test:components` | 0 (1112 / 79 files) |
| `effigy ci:web` | 0 (after import+re-export fix; first run 1 on svelte-check) |
| `effigy docs:lint` | 0 |
| `git diff --check` | 0 |
| `effigy ir:check` | 0 |
| `cargo test --manifest-path packages/codegen/Cargo.toml` | 0 |

## Writable paths

Schema, codegen machine-interface path, eight consumer modules, generated
`machines/` artifacts, `tasks/effigy.tasks.toml` (two `ir:build`/`ir:check`
runs appended), this log. No PAPERCUTS.
