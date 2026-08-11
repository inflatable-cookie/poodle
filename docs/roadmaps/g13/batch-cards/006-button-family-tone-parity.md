# 006 Button Family Tone Parity

Status: merged (`22337a31`); SplitButton slice completed by `008`
Milestone: `g13.001` (contract-conformance work, not IR implementation)
Owner: Poodle core
Branch: `thread/g13-006-button-tone-parity`
Depends on: contract amendments in `282ce489`
Governing refs: `docs/contracts/004-shared-control-types.md` (`T1`–`T4`),
`docs/contracts/components/button.md`,
`docs/contracts/components/icon-button.md`,
`docs/contracts/components/split-button.md`

## Goal

Deliver every `ButtonTone` member in every runtime for Button, IconButton, and
SplitButton. The contracts now declare the full set
(`default | danger | success | warning`); the stylesheets do not implement it.
Close that gap.

This is conformance work against amended contracts. It creates no IR crate,
schema, or generator — the `IR-12` pause is unaffected.

## Background (already established — do not re-derive)

The type layer is already unified and correct:

- `packages/svelte/components/src/types.ts:68` and
  `packages/react/components/src/types.ts:76` —
  `ButtonTone = "default" | "danger" | "success" | "warning"`
- `packages/contracts/components/src/types.rs` — `ButtonTone` with all four
  members; `ButtonVariant::{fill_token, border_token, text_token}` resolve
  every `variant × tone` pair
- Button, IconButton, and SplitButton all import that one shared type in both
  Svelte and React, and all three Rust specs carry `ButtonTone`

The gap is CSS, and possibly renderer/specimen coverage:

| Stylesheet | Implements | Missing |
|---|---|---|
| `packages/core/src/styles/button.css` | danger, warning | **success** |
| `packages/core/src/styles/icon-button.css` | danger, success | **warning** |
| `packages/core/src/styles/split-button.css` | danger | **success, warning** |

## The Derivation Rule (normative)

From `004-shared-control-types.md`: `danger`, `success`, and `warning` are
status tones sharing one structure. Each component's danger treatment is
authoritative for that component; success and warning mirror **that component's
own** danger rules with `--poodle-color-status-success` /
`--poodle-color-status-warning` substituted for `--poodle-color-status-danger`
throughout — fill, border, text, shadow, across idle, hover, and active, for
each of `primary`, `secondary`, and `ghost`.

Do not copy another component's danger structure. Do not invent new
percentages: reuse whatever `color-mix` percentages that component's danger
rules already use.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Contracts are authority. If a contract table and its stylesheet disagree on
  an existing tone, stop and report — do not reconcile silently.
- Do not edit contracts, specs, architecture, roadmap/milestone/card status,
  or `docs/roadmaps/dispatch.md`.
- Do not change component behavior, props, public API surface, or tokens.
- Do not refresh any visual or native baseline. Not for web, not for native.
- `effigy docs:check` rewrites `packages/tokens/artifacts/rust/*`; restore with
  `git checkout -- packages/tokens/artifacts/rust/` and never commit it.
- Run `bun install` before any web generator or build command.
- Commit on the branch above and push with
  `git push -u origin thread/g13-006-button-tone-parity`. Do not merge.

## Writable Paths

- `packages/core/src/styles/button.css`
- `packages/core/src/styles/icon-button.css`
- `packages/core/src/styles/split-button.css`
- `packages/svelte/preview/src/specimens/ButtonSpecimen.svelte`
- `packages/svelte/preview/src/specimens/IconButtonSpecimen.svelte`
- `packages/svelte/preview/src/specimens/SplitButtonSpecimen.svelte`
- `packages/react/preview/src/gallery/specimens/ButtonSpecimen.tsx`
- `packages/react/preview/src/gallery/specimens/IconButtonSpecimen.tsx`
- `packages/react/preview/src/gallery/specimens/SplitButtonSpecimen.tsx`
- `docs/logs/2026-08/<DD>-g13-006-button-tone-parity.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

Regenerated artifacts under `packages/{svelte,react}/preview/artifacts/` are
permitted **only** as generator output, never hand-edited.

Any other changed path is a scope failure. In particular: do not edit
`packages/svelte/components/src/*.svelte`,
`packages/react/components/src/*.tsx`, or any Rust source. If you believe one
needs changing, that is a stop condition.

## Steps

### 1. Baseline

```sh
bun install
effigy docs:lint
effigy svelte:surface-audit
git diff --check
```

Record exit states. All should be 0 on a clean checkout.

### 2. Implement the missing tones

For each stylesheet, read that component's existing danger rules first, then
add the missing tone(s) by substitution per the derivation rule. Match the
existing file's selector shape, ordering, and formatting exactly — the new
rules should be indistinguishable in style from the danger rules beside them.

- `button.css` — add `success` (secondary base, primary, ghost; plus any
  hover/active rules danger has)
- `icon-button.css` — add `warning` (matching all four danger rule groups,
  including ghost hover)
- `split-button.css` — add `success` and `warning` (matching all three danger
  rule groups)

Cross-check each result against the component's contract §8 tone tables.

### 3. Verify the Rust renderer already delivers

Read `packages/render/src/{button,icon_button,split_button}.rs` and confirm each
resolves all four tones through the shared `ButtonVariant` token resolvers with
no tone-specific gaps or `unreachable!`/fallback arms that would drop success or
warning.

**Report findings; do not edit Rust.** If a renderer gap exists, that is a stop
condition — record it with exact paths and stop.

### 4. Specimen coverage

Add the missing tone to each component's Svelte and React specimen tone rows so
all four tones are visible in the preview, matching the existing row pattern.
Keep Svelte and React identical — they are parity-tested against each other.

Do not add new specimen groups or restructure existing ones. One additional row
or entry per missing tone.

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

Record every command and exit state. If `test:visual` is triggered and reports
diffs, **do not update baselines** — record the diff list and stop.

## Acceptance Criteria

- [x] All three stylesheets implement all four tones across primary, secondary,
  and ghost. **SplitButton completed by `008` (`f59adac0`).**
- [x] Each added rule derives from that component's own danger rules with only
  the status color substituted; no new percentages invented.
- [x] Rust renderer tone delivery verified and reported for all three
  components; no Rust edited.
- [x] Svelte and React specimens show all four tones for all three components,
  and remain identical to each other. **SplitButton completed by `008`.**
- [x] No component source, props, public API, token, or baseline changed.
- [x] `effigy docs:lint`, `test:components`, `test:parity`, `docs:check`, and
  `git diff --check` all exit 0.
- [x] Only the writable paths (plus regenerated artifacts) changed.
- [x] Batch log records commands, exit states, and the renderer verification.

## Stop Conditions

- A contract §8 tone table and its stylesheet disagree on an existing tone.
- The Rust renderer does not deliver a tone (report; do not fix here).
- Delivering a tone requires changing component source, props, or tokens.
- A visual baseline would need refreshing.
- A component's danger structure is ambiguous enough that substitution is a
  judgment call.

Stop with exact paths, selectors, commands, and the smallest unresolved
question. Do not patch around it.
