# 11 — g13.006 Button Family Tone Parity (batch log)

Branch: `thread/g13-006-button-tone-parity` (dedicated worktree, pushed with
`git push -u origin thread/g13-006-button-tone-parity`)
Date: 2026-08-11
Card: `docs/roadmaps/g13/batch-cards/006-button-family-tone-parity.md`
Status: **STOPPED** — split-button slice blocked by a contract §8 /
stylesheet disagreement on the existing danger tone (see §5). Button and
IconButton slices delivered and validated.

## 1. Baseline (step 1)

| Command | Exit | Notes |
|---|---|---|
| `bun install` | 0 | 234 packages installed |
| `effigy docs:lint` | 0 | 170 contracts, 12 parity targets, … |
| `effigy svelte:surface-audit` | 0 | 175 files, 163 public exports, 0 coverage gaps |
| `git diff --check` | 0 | clean tree, branch `thread/g13-006-button-tone-parity` |

## 2. Stylesheets (step 2) — delivered for Button and IconButton

Both derivations follow the card's normative rule: mirror **that component's
own** danger rules with the status color substituted, reusing the exact
`color-mix` percentages already present in the file. Recipe-hook names follow
the danger naming shape (`-success-` / `-warning-` in the same slot).

### `packages/core/src/styles/button.css` — added `success`

Three blocks inserted between the danger group and the existing warning group,
mirroring `.poodle-button[data-tone="danger"]`,
`.poodle-button[data-variant="primary"][data-tone="danger"]`, and
`.poodle-button[data-variant="ghost"][data-tone="danger"]` with
`status-success` substituted:

- secondary base: fill 16% / 24% / 32%, border-hover 62% — percentages from
  the file's own danger base
- primary: fill `status-success`, hover `white 12%`, active `88% black`,
  border `84% black`, border-hover `72% black`, shadow `white 14%` +
  `black 18%` — percentages from the file's own primary danger
- ghost: fill-hover `12%`, fill-active `18%`, border-hover `28%`,
  text `status-success` — percentages from the file's own ghost danger

Button has no tone-specific hover/active rules (generic `:hover`/`:active`
consume the custom properties), so no additional rules are needed — same
shape as the existing warning group.

### `packages/core/src/styles/icon-button.css` — added `warning`

Four blocks, each placed after its success counterpart to match the file's
danger/success interleaving: base, primary, ghost, ghost hover. All mirror
the file's own danger rules with `status-warning` substituted:

- base: fill `16%`, border `46%`
- primary: fill `status-warning`, border `84% black`, text inverse
- ghost: fill/border transparent, text `status-warning`
- ghost hover: border `46%`, background `10%`

### `packages/core/src/styles/split-button.css` — NOT delivered (stop condition, §5)

## 3. Renderer verification (step 3) — all three deliver all four tones

Read-only; no Rust edited.

- `packages/contracts/components/src/types.rs:255-306` — `ButtonVariant`::
  `fill_token` / `border_token` / `text_token` matches are exhaustive over
  `variant × tone`; `success`/`warning` resolve dedicated status tokens
  (`COLOR_STATUS_SUCCESS` / `COLOR_STATUS_WARNING`). No `unreachable!`, no
  fallback arm.
- `packages/render/src/button.rs:69-77,95-113,126-150` — explicit
  `ButtonTone::Danger|Success|Warning` → `Some(status_color)`; variant arms
  keyed on `(variant, status)` deliver fill/border/text/hover/active for all
  four tones. Primary border uses the old GPUI tier's `mix_black(.., 0.86)`
  vs CSS `84%` — pre-existing variant-level parity nuance, applies to every
  primary tone including default, not tone-specific; noted, not a gap.
- `packages/render/src/icon_button.rs:64-107` — tones resolved through the
  shared `fill_token/border_token/text_token` plus an explicit secondary
  status-tint arm (`Danger | Success | Warning` → 16% / 46% mixes). All four
  tones delivered.
- `packages/render/src/split_button.rs:36-64` — `resolve_split_colors` uses
  the shared token resolvers (exhaustive for all tones); Primary/Ghost
  transforms are tone-agnostic, Secondary/legacy wear tokens straight. No
  tone dropped.

Renderer corroboration for §5: `split_button.rs` paints no shadow in any
state (no `ShadowLayer`; `packages/render/src/split_button.rs:8-13` imports
don't include it) — web and native agree on `none` for split-button primary
danger; only the contract disagrees.

## 4. Specimens (step 4) — Button and IconButton

One additional tone group per missing tone, matching each file's existing
danger-row pattern (three variants, same props/icon shape); Svelte and React
identical:

- `ButtonSpecimen.svelte` / `ButtonSpecimen.tsx` — added `Success tone`
  group (primary / secondary / ghost) after `Danger tone`
- `IconButtonSpecimen.svelte` / `IconButtonSpecimen.tsx` — added
  `Warning tone` group (primary / secondary / ghost) after `Danger tone`

SplitButton specimens untouched (split slice blocked).

## 5. Stop condition — split-button danger shadow

**Finding.** `docs/contracts/components/split-button.md` §8 "Tone: danger"
(line 178) specifies `[data-variant="primary"][data-tone="danger"]` →
`--poodle-split-shadow` = `inset 0 0.0625rem 0 color-mix(white 14%,
transparent), 0 0.375rem 1.125rem color-mix(black 18%, transparent)`, and
lines 184-185 state derived tones carry "fill, border, text, **and shadow**".
`packages/core/src/styles/split-button.css` does not deliver it:

- `:13` root — `--poodle-split-shadow: none`
- `:25` `.poodle-split-button[data-variant="primary"]` — `--poodle-split-shadow: none`
- `:40-44` `.poodle-split-button[data-variant="primary"][data-tone="danger"]`
  — sets fill/border/text only; shadow inherits `none`
- `:74` sole consumer: `box-shadow: var(--poodle-recipe-split-button-toggle-shadow, var(--poodle-split-shadow))` (recipe hook is app-side)

Secondary base (`default` = root `none`) and ghost (`none`) rows are
consistent; the primary row disagrees. History corroborates: the shadow
column arrived with the contract amendment `282ce489` (this card's
dependency); `split-button.css` was last touched in `5ef12f0c` — the shadow
was never implemented. Button implements the same-shaped primary-danger
shadow (`button.css` `.poodle-button[data-variant="primary"][data-tone="danger"]`),
matching `button.md` §8, so SplitButton deviates from both its own contract
and the family pattern. Renderer paints no split shadow either (§3).

**Why stopped, not patched.** Worker Rules: "If a contract table and its
stylesheet disagree on an existing tone, stop and report — do not reconcile
silently." The split-button success/warning derivation is downstream: adding
the shadow to danger + derived tones would silently reconcile an existing
tone's rendering; omitting it would ship success/warning contradicting the
contract's "and shadow" note. Neither is permitted.

**Smallest unresolved question.** Should
`.poodle-split-button[data-variant="primary"][data-tone="danger"]` gain
`--poodle-split-shadow: inset 0 0.0625rem 0 color-mix(in srgb, white 14%,
transparent), 0 0.375rem 1.125rem color-mix(in srgb, black 18%, transparent)`
(with success/warning mirroring it), or is `none` the intended SplitButton
primary treatment, meaning the contract row and the "and shadow" derivation
note need correction?

## 6. Validation (step 5) — run on the delivered slices

| Command | Exit | Notes |
|---|---|---|
| `effigy docs:lint` | 0 | — |
| `effigy test:components` | 0 | 38 files / 810 tests |
| `effigy test:parity` | 0 | 2 files / 158 tests (Svelte/React specimen parity incl. new rows) |
| `effigy docs:check` | 0 | rewrote `packages/tokens/artifacts/rust/*` (card warning) |
| `git checkout -- packages/tokens/artifacts/rust/` | 0 | restored; nothing from that directory committed |
| `git diff --check` | 0 | — |
| `git status --porcelain` | — | only the 6 writable paths below |

No `test:visual` invocation; no baseline refreshed. Final changed paths:

```
 packages/core/src/styles/button.css                 | success tone added
 packages/core/src/styles/icon-button.css            | warning tone added
 packages/svelte/preview/src/specimens/ButtonSpecimen.svelte     | Success tone group
 packages/react/preview/src/gallery/specimens/ButtonSpecimen.tsx | Success tone group
 packages/svelte/preview/src/specimens/IconButtonSpecimen.svelte | Warning tone group
 packages/react/preview/src/gallery/specimens/IconButtonSpecimen.tsx | Warning tone group
 docs/logs/2026-08/11-g13-006-button-tone-parity.md  | this log
 PAPERCUTS.md                                        | friction entry
```

## 7. Acceptance criteria — partial

- [x] All three stylesheets implement all four tones — **partial**: Button
  and IconButton yes; SplitButton blocked (§5).
- [x] Added rules derive from that component's own danger rules with only the
  status color substituted; no new percentages invented — for delivered
  slices.
- [x] Rust renderer tone delivery verified and reported for all three
  components; no Rust edited.
- [x] Svelte and React specimens show all four tones — **partial**:
  Button/IconButton yes; SplitButton untouched (blocked). Svelte/React
  identical.
- [x] No component source, props, public API, token, or baseline changed.
- [x] `effigy docs:lint`, `test:components`, `test:parity`, `docs:check`,
  `git diff --check` all exit 0.
- [x] Only the writable paths (plus regenerated artifacts) changed.
- [x] Batch log records commands, exit states, and renderer verification.
