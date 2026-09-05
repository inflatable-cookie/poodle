# g16.108 — Docs Spine Compaction

Status: ready
Type: documentation hygiene — docs only, no active roadmap surface
Opened: 2026-09-05
Depends on: none
Governing refs: `../../README.md` (project record section),
`../../contracts/001-working-rules.md` (rollover purge rule),
`../../parity/README.md`, `../../specs/README.md`,
`../../guides/svelte-developer-guide.md`,
`../../contracts/components/README.md`,
`../../triage/20260901-233708-holistic-posture-assessment.md` (docs
compaction remainder)
Dispatch manifest: `../dispatch.md`

## Goal

Remove the dead weight that agents keep reading and sometimes editing, fix
the consumer-facing guides that teach removed APIs, and make the snippet
class of defect impossible to reintroduce.

## Fixed Boundary

1. **Handoffs.** 184 files under `docs/handoffs/`. Move every handoff whose
   lane is complete (its card or log says merged/complete) to
   `docs/handoffs/archive/YYYY-MM/`; keep open lanes' handoffs in place.
   Add a retention rule to `docs/README.md`: a handoff is archived when its
   lane closes. Fix any relative links the move breaks.
2. **Parity.** `docs/parity/` (141 files) is marked historical. Move it to
   `docs/archive/parity/`, keep its README as the pointer, and remove every
   instruction in `docs/roadmaps/g16/*.md` and log templates that tells a
   worker to edit a parity file (four g16 cards do).
3. **Specs.** Compute the set of `docs/specs/*.md` not referenced from
   `docs/architecture`, `docs/contracts`, `docs/roadmaps/g16`, or
   `docs/roadmaps/generation-index.md` (the audit found 28). Move them to
   `docs/specs/archive/` with a one-line index entry each. Mark spec 001
   `active`. Update `docs/specs/README.md`.
4. **Guides.** Fix `svelte-developer-guide.md:903,908` (`Tabs` variants are
   `card | pill | block`), `:1226` (`ButtonTone` has four members), the
   pre-state Popover trigger snippet near `:820`, and the HistoryCenter v1
   usage in `packages/svelte/preview/src/component-docs.ts` (v3 shape).
   Remove the duplicate `token-input.md` line in the contracts index.
5. **Snippet check.** Add `docs:snippet-check`: extract fenced `svelte`
   blocks from `docs/guides/*.md` into a temp Svelte project that depends on
   the workspace packages and run `svelte-check`; include it in
   `docs:check`. Plant one stale prop to prove it bites.

Out of scope: the active roadmap front doors (`docs/roadmaps/README.md`,
`generation-index.md`, `g16/README.md`) and `dispatch.md`; they compact at
generation rollover. Do not touch code outside the one guide-snippet check
script and its task entry.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Nothing open is archived | an open lane's handoff moved | reviewer checks against the manifest and open cards |
| Links survive | `effigy docs:check` | link checks green after the moves |
| Parity is not an edit target | grep `docs/parity/` in g16 cards and log templates | zero instructions remain |
| Guides compile | `docs:snippet-check` | green; planted stale prop fails |
| Specs index is honest | a moved spec still listed as active | `specs/README.md` diff |

## Validation

`effigy docs:check`, `effigy docs:snippet-check`, `git diff --check
origin/main...HEAD`.

## Owned Paths

`docs/handoffs/**`, `docs/parity/**` → `docs/archive/parity/**`,
`docs/specs/**`, `docs/guides/**`, `docs/contracts/components/README.md`,
`docs/README.md` (retention rule), `docs/roadmaps/g16/0*.md` (parity edit
instructions only), `packages/svelte/preview/src/component-docs.ts`
(HistoryCenter snippet), one new script under
`packages/svelte/preview/scripts/`, `tasks/effigy.tasks.toml` (one selector
and the `docs:check` line), execution log, `PAPERCUTS.md` (append only).

## Stop Conditions

Stop if a handoff's lane state cannot be determined (leave it in place and
list it), or if a spec move breaks a link that cannot be rewritten without a
content decision. Escalation owner: Chatterbox.
