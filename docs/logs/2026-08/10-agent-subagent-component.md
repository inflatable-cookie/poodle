# Agent Subagent Component

Executed card `docs/roadmaps/g12/023-agent-subagent-component.md` on branch
`thread/023-agent-subagent-component`.

## Deliverables

- Contract `docs/contracts/components/agent-subagent.md` (status `draft`), the
  eight-value Swallowtail status vocabulary pinned in §4, plus index entries
  in `docs/contracts/README.md` and `docs/contracts/components/README.md`
- Headless core `packages/core/src/agent-subagent.ts`
  (`AgentSubagentStatus`, `AgentSubagentItem`, `subagentStatusLabel`,
  `isTerminalSubagentStatus`, `subagentStatusSpins`) exported from
  `packages/core/src/index.ts`
- Conformance vectors `packages/contracts/headless/vectors/agent-subagent.json`
  (labels / terminal / spinning) + TS runner
  (`packages/core/test/agent-subagent-conformance.test.ts`) + Rust mirror
  (`packages/contracts/headless/src/agent_subagent.rs`,
  `packages/contracts/headless/tests/agent_subagent_conformance.rs`)
- Spec crate `packages/contracts/components/src/agent_subagent.rs`
  (`AgentSubagentSpec`) exported from its lib.rs
- Renderer `packages/render/src/agent_subagent.rs` (`agent_subagent`,
  `AgentSubagentHandlers`) exported from its lib.rs
- Svelte `packages/svelte/components/src/AgentSubagent.svelte` + exports in
  `index.ts` and `types.ts`; styles
  `packages/core/src/styles/agent-subagent.css`
- Transcript: new `TranscriptItem`/`TranscriptBlock` kind `"subagent-group"`
  (`TranscriptSubagentGroup`) in `packages/core/src/agent-transcript.ts`, the
  Rust mirror in `packages/contracts/headless/src/agent_transcript.rs`, two new
  grouping vectors in `agent-transcript.json` (pass-through; splits a run),
  rendered by `AgentTranscript.svelte` via `AgentSubagent` and by the shared
  renderer via `agent_subagent` (`expanded_subagent_groups` on
  `AgentTranscriptSpec`; `on_subagent_toggle` / `on_subagent_open` handlers)
- Specimen `packages/svelte/preview/src/specimens/AgentSubagentSpecimen.svelte`
  + registry entries (`specimens/registry.ts`, `component-registry.ts`,
  `parity.ts`) + `test/fixtures/component-props.ts` fixture
- This batch log

## Stop-Condition Findings

- **Batch log path differs from the card.** The card names
  `docs/logs/2026-08-10-agent-subagent-component.md`; the repo groups logs by
  year and month (`docs/logs/README.md`: "Logs are grouped by year and month",
  existing 2026-08-10 entries all `docs/logs/2026-08/10-*.md`), so this log
  lives at `docs/logs/2026-08/10-agent-subagent-component.md`. No other card
  path conflicts with the current layout; all files cited in Scope exist at
  the card's paths.
- No existing transcript vectors broke from the `subagent-group` union change
  (grouping passes non-tool-call items through untouched; the new kind only
  adds cases). No React/GPUI/Jetstream variants were added.

## Validated (commands and exit states)

| Command | Exit |
|---------|------|
| `effigy docs:contract-drift` (contract-prop-drift: 119 checked, OK) | 0 |
| `effigy docs:spec-drift` (contract-spec-drift: 112 checked, OK) | 0 |
| `bun run --cwd packages/core test` (373 pass, 0 fail) | 0 |
| `cargo test --manifest-path packages/contracts/headless/Cargo.toml` (agent_subagent_conformance 3, agent_transcript_conformance 4, all crates green) | 0 |
| `cargo test --manifest-path packages/contracts/components/Cargo.toml` (236 pass incl. 4 new AgentSubagentSpec tests) | 0 |
| `cargo test --manifest-path packages/contracts/markdown/Cargo.toml` | 0 |
| `cargo check` + `cargo test` `packages/render/Cargo.toml` (138 pass) | 0 |
| `bunx vitest run` (32 files, 723 tests incl. axe sweep over AgentSubagent) | 0 |
| `bunx vitest run --project parity` (146 pass) | 0 |
| `bun run --cwd packages/svelte/preview docs:lint` (158 contracts, OK) | 0 |
| `git diff --check` | 0 |

Note: `bun run --cwd packages/core test` initially failed with
`Cannot find package 'marked'` because no dependencies were installed
(`node_modules` absent); `bun install` fixed the environment, after which all
349 pre-existing tests plus 24 new vector tests pass. Pre-existing
environment gap, not a code regression.

## Browser Verification

Preview dev server (`bun run --cwd packages/svelte/preview dev --port 5199`,
route `#components/agent-subagent`): all six specimen states render — running
(dots spinner + activity line), waiting (no spinner), completed (summary),
failed (summary), unknown (badge reads literally "Unknown"), and expanded
(3 detail lines). Clicking the disclosure toggles `data-expanded` and swaps
"Show activity"/"Hide activity". Screenshots via the headless browser driver
timed out (driver quirk, not a page error); DOM-level state assertions stand.

## Papercuts

- None new. The `find / -name poodle-node` lookup timed out (90s+); the crate
  is at `packages/contracts/node` — could not add a PAPERCUTS entry beyond
  existing ones, so noted here: locating a crate by path search is slow; use
  `Cargo.toml` path deps instead.
