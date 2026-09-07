# g17.002 — Web focus rings follow input modality

Status: implementation complete — pending independent exact-head review
Date: 2026-09-07
Card: `docs/roadmaps/g17/002-web-focus-ring-input-modality.md`
Base: `origin/main` at `83ce172bcf7354ff266b6a7c8fec9d438590622e`
Branch: `worker/g17-002-web-focus-ring-input-modality`

## Outcome

Pointer focus no longer paints composite web focus treatments. Keyboard
users keep the full treatment (border, fill, and shadow together). TokenInput
was the operator-reported case and the probe subject.

- `packages/core/src/dom/input-modality.ts` installs capture-phase listeners
  and writes `:root[data-poodle-input-modality="keyboard"|"pointer"]`. Initial
  value is `keyboard`. SSR-safe. Idempotent per document.
- Seven composite `:focus-within` rules now gate on that attribute.
- Six bare `:focus` rules are `:focus-visible`.
- The seven Svelte and seven React composites call `installInputModality()`
  on mount. No provider, no prop, no GPUI change.

## Probe evidence

Headless Playwright, foreground, one engine per run. Computed styles on
`.poodle-token-input`. jsdom was not used for this proof.

### Chromium

| Shell | State | modality | box-shadow | border-color |
| --- | --- | --- | --- | --- |
| svelte | resting | pointer | `none` | `color(srgb 0.890196 0.909804 0.933333 / 0.0447059)` |
| svelte | pointer click | pointer | `none` (equals resting) | same as resting |
| svelte | Tab | keyboard | `color(srgb 0.941177 0.698039 0.301961 / 0.18) 0px 0px 0px 2px` | `color(srgb 0.939094 0.706691 0.327756 / 0.604706)` |
| react | pointer click | pointer | `none` (equals resting) | same as resting |
| react | Tab | keyboard | same keyboard recipe as svelte | same keyboard recipe as svelte |

### WebKit

| Shell | State | modality | box-shadow | border-color |
| --- | --- | --- | --- | --- |
| svelte | pointer click | pointer | `none` | `color(srgb 0.890196 0.909804 0.933333 / 0.0456)` |
| svelte | Tab | keyboard | `color(srgb 0.941176 0.698039 0.301961 / 0.18) 0px 0px 0px 2px` | `color(srgb 0.939054 0.706857 0.328251 / 0.6052)` |
| react | pointer click / Tab | pointer / keyboard | matches svelte | matches svelte |

Background fill did not change: TokenInput's focus fill equals the resting
fill in eclipse. Border and shadow are the treatment.

Tracker unit tests (happy-dom, `packages/svelte/preview/test/input-modality.test.ts`):
flip pointer→keyboard→pointer, modifier-only ignored, one listener set on
double install, SSR no-op, both shells write the root attribute.

`rg -P ':focus(?!-visible|-within)' packages/core/src/styles` → zero matches.

## Out of scope (reported, not widened)

`packages/core/src/styles/text-input.css:36` still uses un-gated
`.poodle-text-input:focus-within` (same composite leak class as TokenInput).
`split-view.css`, `ref-select.css`, and `model-picker.css` also use
`:focus-within` for reveal/emphasis, not a field chrome ring. Chatterbox
owns whether those get a follow-up card.

## Validation

- `rg -P ':focus(?!-visible|-within)' packages/core/src/styles` — zero matches
- `effigy test:focus-ring-modality-chromium` — passed (svelte + react)
- `effigy test:focus-ring-modality-webkit` — passed (svelte + react)
- `effigy test:components` — 388 files / 3,749 tests passed
- `effigy docs:check` — passed
- `effigy ci:web` — passed (388 files / 3,749 tests; svelte-check 0 errors)
- `git diff --check origin/main` — clean
- `ci:rust` — not run (no Rust/GPUI change)

No windowed selector was run. No merge was performed. The worker stops after
one pushed PR for independent exact-head review.
