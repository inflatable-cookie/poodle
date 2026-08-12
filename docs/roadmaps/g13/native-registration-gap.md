# Native Registration Gap Inventory

Status: open (report only — nothing implemented)
Milestone: pre-`g13.014` (recorded so the gap is tracked, not silent)
Governing refs: `docs/roadmaps/g13/014-native-backend-convergence.md`,
`docs/roadmaps/g13/015-generated-contracts-registries-and-parity-evidence.md`

## What This Is

Sixteen Svelte components have no entry in one or both native preview
registries. Nothing gates this, so the set grew unnoticed after the parity
campaign was declared essentially complete. This file records it so the gap is
visible while it waits for its owner.

Counted on 2026-08-12 from `packages/svelte/components/src/*.svelte` against
`display_name:` entries in each native `component_registry.rs` (updated the
same day by card `039`, which added the web-only `SettingsShell`):

- GPUI: 14 missing
- Jetstream: 15 missing
- Union: 16 components

| Component | GPUI | Jetstream |
|-----------|------|-----------|
| `AgentMessage` | — | — |
| `AgentPlan` | — | — |
| `AgentPlanRecord` | — | — |
| `AgentQuestionRecord` | — | — |
| `AgentSubagent` | — | — |
| `ChangedFiles` | — | — |
| `HistoryCenter` | — | — |
| `MenuSurface` | — | — |
| `Radio` | — | yes |
| `RemediationBanner` | — | — |
| `SettingsShell` | — | — |
| `StateTile` | — | — |
| `ThemeSelect` | yes | — |
| `ToolCall` | — | — |
| `ToolCallGroup` | — | — |
| `ValidationSummary` | yes | — |

## Why This Is Not A Card

Closing it by hand means writing fifteen native components against the current
composition model. `g13.010`–`g13.013` then delete that work: each wave's scope
says *"Remove superseded Svelte, React, and native composition duplicates."*
Hand-authored native composition is a duplicate-in-waiting.

`g13.014` owns the real fix — GPUI and Jetstream become strict interpreters of
one `poodle-render` implementation — and its acceptance already covers the
enforcement this inventory would otherwise motivate:

> Registration parity cannot report green when a runtime renders a placeholder
> or bypasses the shared path.

So the gate belongs to `g13.014`/`g13.015`, not here. Building it now would
duplicate a planned deliverable and lock in the composition model the pilot has
not yet judged.

Everything downstream is gated on the `g13.008` adopt verdict, and `g13.004`
through `g13.007` have not started.

## When To Revisit

- The `g13.008` verdict records **adopt** — then `g13.014` closes this whole
  table as a by-product, and the numbers here become the before-count.
- The verdict records **revise** or **reject** — then this becomes a real
  backlog, because no generated path is coming to absorb it, and it should be
  recompiled into cards.
- A consumer needs one of these components in a GPUI or Jetstream app before
  the pilot lands. Then it is a product need, not a parity chore, and the
  throwaway cost is worth paying for that one component.

## Refresh

```sh
bun - <<'JS'
import { readdirSync, readFileSync } from "node:fs";
const web = readdirSync("packages/svelte/components/src")
  .filter((f) => f.endsWith(".svelte")).map((f) => f.slice(0, -7));
const reg = (p) => new Set([...readFileSync(p, "utf8")
  .matchAll(/display_name:\s*"([^"]+)"/g)].map((m) => m[1]));
for (const [name, path] of [
  ["gpui", "packages/gpui/preview/src/component_registry.rs"],
  ["jetstream", "packages/jetstream/preview/src/component_registry.rs"],
]) {
  const have = reg(path);
  const missing = web.filter((c) => !have.has(c)).sort();
  console.log(`${name}: ${missing.length} missing — ${missing.join(", ")}`);
}
JS
```
