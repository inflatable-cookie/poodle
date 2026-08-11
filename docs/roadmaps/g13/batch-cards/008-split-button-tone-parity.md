# 008 SplitButton Tone Parity And Primary Status Shadow

Status: merged (commit `f59adac0`)
Milestone: `g13.001` (contract-conformance work, not IR implementation)
Owner: Poodle core
Branch: `thread/g13-008-split-button-tone-parity`
Depends on: `g13-b006` merged (`22337a31`)
Governing refs: `docs/contracts/004-shared-control-types.md` (`T1`–`T4`),
`docs/contracts/components/split-button.md`,
`docs/contracts/components/button.md`

## Goal

Finish the button-family tone work. `g13-b006` delivered Button and IconButton
and correctly stopped on SplitButton, because `split-button.md` §8 mandates an
elevation shadow on `[data-variant="primary"][data-tone="danger"]` that
`split-button.css` does not implement. That conflict is now ruled; deliver the
result.

## Orchestrator Ruling — the shadow (already decided, do not re-litigate)

**The contract is correct. `split-button.css` is the outlier. Implement the
shadow.**

`g13-b006` reasoned that both runtimes agree on `none` and only the contract
disagrees, so the contract looked stale. That reasoning was sound but
incomplete — it did not check the sibling component. The evidence:

- `packages/core/src/styles/button.css:193` already ships exactly that shadow
  on primary danger:
  `inset 0 0.0625rem 0 color-mix(in srgb, white 14%, transparent), 0 0.375rem 1.125rem color-mix(in srgb, black 18%, transparent)`
- `button.css:223` ships the same on primary warning.
- `split-button.md` §1 scopes the component as "variant, tone, and size parity
  with Button".
- `split-button.md` §8 mandates the identical shadow string.

Three sources agree. `split-button.css:13,25` set `--poodle-split-shadow: none`
and never override it for primary status tones. That is the defect.

This is a deliberate, approved **visual change** to SplitButton primary danger.
It is a bug fix, not a redesign.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Do not edit contracts, specs, architecture, roadmap/milestone/card status, or
  `docs/roadmaps/dispatch.md`.
- Do not change component behavior, props, public API surface, or tokens.
- Do not edit `packages/svelte/components/src/*.svelte`,
  `packages/react/components/src/*.tsx`, or any Rust source. A needed change
  there is a stop condition.
- **Do not refresh any visual or native baseline.** Enumerate the diffs and
  stop for orchestrator approval — see step 4. This is the one deliverable the
  orchestrator must approve before it lands.
- `effigy docs:check` rewrites `packages/tokens/artifacts/rust/*`; restore with
  `git checkout -- packages/tokens/artifacts/rust/` and never commit it.
- The working tree may contain unrelated in-progress edits under
  `packages/svelte/components/src/`. Leave them alone; never `git add -A`.
  Stage only your writable paths by explicit path.
- Run `bun install` before any web generator or build command.
- Commit and push with
  `git push -u origin thread/g13-008-split-button-tone-parity`. Do not merge.

## Writable Paths

- `packages/core/src/styles/split-button.css`
- `packages/svelte/preview/src/specimens/SplitButtonSpecimen.svelte`
- `packages/react/preview/src/gallery/specimens/SplitButtonSpecimen.tsx`
- `docs/logs/2026-08/<DD>-g13-008-split-button-tone-parity.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

Regenerated preview artifacts are permitted only as generator output.

## Steps

### 1. Deliver the primary danger shadow

Give `[data-variant="primary"][data-tone="danger"]` the elevation shadow from
`split-button.md` §8. Match how `button.css:193` expresses it, including the
`--poodle-recipe-split-button-*-shadow` hook shape used elsewhere in
`split-button.css`. Leave the secondary and ghost shadow values as the contract
specifies (`default` and `none`).

### 2. Add success and warning

Derive both from SplitButton's **own** danger rules — now including the shadow
from step 1 — substituting `status-success` / `status-warning`. Reuse the
existing percentages. Do not copy Button's rule bodies; SplitButton has its own
fill/border/text/shadow structure and its own custom-property names.

Cross-check against `split-button.md` §8 "Tone: success and warning", which
states the derivation rather than tabulating it.

### 3. Specimen coverage

Add Success and Warning tone rows to the Svelte and React SplitButton
specimens, matching the existing danger row pattern. Keep the two identical —
they are parity-tested against each other. Do not restructure existing groups.

### 4. Visual diff enumeration — then stop

Run the visual gate in **report** mode. Do not pass `--update`. Produce a table
in the batch log with one row per changed baseline: slug, axis, diff ratio, and
a one-line cause (expected: SplitButton primary danger gains a shadow;
SplitButton gains success/warning specimen rows).

Classify each diff as `expected` or `unexpected`. **Any unexpected diff, or any
diff on a component other than split-button, is a stop condition.**

Commit the code and the enumeration. Leave every baseline file unchanged — the
orchestrator approves the refresh separately.

### 5. Validate

```sh
effigy docs:lint
effigy test:components
effigy test:parity
effigy docs:check
git checkout -- packages/tokens/artifacts/rust/
git diff --check
git status --porcelain
```

Record every command and exit state.

## Acceptance Criteria

- [x] `split-button.css` implements the primary danger elevation shadow per
  contract §8.
- [x] `split-button.css` implements success and warning across primary,
  secondary, and ghost, derived from SplitButton's own danger rules with only
  the status colour substituted.
- [x] Svelte and React SplitButton specimens show all four tones and remain
  identical.
- [x] Visual diffs enumerated and classified in the batch log; **no baseline
  file modified**.
- [x] No component source, props, public API, token, or Rust changed.
- [x] No unrelated working-tree edits staged or committed.
- [x] `effigy docs:lint`, `test:components`, `test:parity`, `docs:check`, and
  `git diff --check` all exit 0.
- [x] Batch log records commands, exit states, and the diff table.

## Stop Conditions

- A visual diff appears on any component other than split-button.
- A visual diff on split-button cannot be explained by the shadow or the new
  specimen rows.
- Delivering the shadow requires changing component source, tokens, or Rust.
- SplitButton's danger structure is ambiguous enough that substitution becomes
  a judgment call.

Stop with exact selectors, paths, commands, and the smallest unresolved
question.
