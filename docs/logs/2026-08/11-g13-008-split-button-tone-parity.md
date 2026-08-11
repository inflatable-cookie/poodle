# 11 — g13.008 SplitButton Tone Parity And Primary Status Shadow (batch log)

Branch: `thread/g13-008-split-button-tone-parity` (dedicated worktree)
Date: 2026-08-11
Card: `docs/roadmaps/g13/batch-cards/008-split-button-tone-parity.md`
Status: **DELIVERED** — shadow + success/warning tones + specimens + diff
enumeration. Visual refresh deliberately NOT performed (no `--update`; no
baseline file touched); orchestrator approves the refresh separately.

## 1. Shadow (step 1) — `packages/core/src/styles/split-button.css`

`[data-variant="primary"][data-tone="danger"]` now sets `--poodle-split-shadow`
to the elevation shadow mandated by `split-button.md` §8, expressed exactly
like `button.css:193` with SplitButton's own recipe-hook shape:

```css
--poodle-split-shadow: var(
  --poodle-recipe-split-button-primary-danger-shadow,
  inset 0 0.0625rem 0 color-mix(in srgb, white 14%, transparent),
  0 0.375rem 1.125rem color-mix(in srgb, black 18%, transparent)
);
```

Secondary and ghost shadow values untouched (secondary inherits root `none` =
the contract's `default`; ghost inherits root `none` = the contract's `none`).

**Delivery proof (live computed style).** Injected a `data-variant="primary"
data-tone="danger"` split-button into the running Svelte preview and read the
computed style of its primary half:

- `--poodle-split-shadow` resolves to the exact contract string
- computed `box-shadow`: `color(srgb 1 1 1 / 0.14) 0px 1px 0px 0px inset,
  color(srgb 0 0 0 / 0.18) 0px 6px 18px 0px` — identical shadow to
  `button.css:193` (Chrome emits `color(srgb …)` for `color-mix`; the
  white-14% inset highlight + black-18% drop are the same layers)
- screenshot pixel analysis: top edge lighter than body (inset highlight),
  dark pixels below the body (drop shadow)

The gate cannot capture this shadow: the specimen's existing danger row
renders the default (secondary) variant, and the card's step 3 forbids
restructuring groups, so no specimen surface renders primary danger. The
shadow is verified at CSS/computed-style level.

## 2. Success and warning tones (step 2)

Both derived from SplitButton's **own** danger rules with only the status
colour substituted (`status-success` / `status-warning`), reusing the file's
existing percentages and its own custom-property names (`--poodle-split-fill`,
`--poodle-split-border`, `--poodle-split-text`, `--poodle-split-shadow`).
No Button rule bodies copied. Per `split-button.md` §8 "Tone: success and
warning" derivation rule.

Added for each tone: secondary base (fill `16%` / border `46%` / text
primary), primary (fill status colour / border `84% black` / text inverse /
elevation shadow with `--poodle-recipe-split-button-primary-{success,warning}-
shadow` hooks), ghost (fill+border transparent / text status colour). Ghost
shadow stays inherited `none` exactly as ghost danger's.

## 3. Specimens (step 3)

- `packages/svelte/preview/src/specimens/SplitButtonSpecimen.svelte`
- `packages/react/preview/src/gallery/specimens/SplitButtonSpecimen.tsx`

Success (`Publish`, publish-now/schedule items) and Warning (`Archive`,
archive-selected/archive-all items) rows added after the Danger row, matching
the existing danger-row pattern (default variant). Identical content in both
frameworks. Existing groups untouched. All four tones now shown.

## 4. Visual diff enumeration (step 4) — report mode, no `--update`

Gate: `effigy visual:report` (`bun test/visual/run.ts --tier=sweep --report`),
run at HEAD and again after the change. The web gate is a Svelte↔React parity
gate with **no stored baselines** (by design, per `test/visual/README.md`), so
"changed baselines" were produced by capturing the split-button specimen
before and after the change with the gate's own capture machinery
(determinism pinning, same axes), diffed with pixelmatch.

### Changed-baseline table (before vs after captures)

| slug | axis | diff | diff ratio | cause | class |
|---|---|---|---|---|---|
| split-button | eclipse-compact-md (svelte) | 992×359 → 992×455 (+96px) | n/a (size change) | specimen gains Success + Warning rows | expected |
| split-button | eclipse-compact-md (react) | 992×359 → 992×455 (+96px) | n/a (size change) | specimen gains Success + Warning rows | expected |
| split-button | iceberg-compact-md (svelte) | 992×359 → 992×455 (+96px) | n/a (size change) | specimen gains Success + Warning rows | expected |
| split-button | iceberg-compact-md (react) | 992×359 → 992×455 (+96px) | n/a (size change) | specimen gains Success + Warning rows | expected |

Size changes cannot be expressed as pixel ratios; the +96px equals two
specimen rows at ~48px each. The primary-danger shadow change is not
specimen-visible (existing danger row renders the default variant); delivery
proof in §1.

### Cross-framework parity of the change

After-state captures: svelte vs react differ on 0.0011% of pixels (5 px in
992×455; sub-pixel text antialiasing) — below the gate's 0.0002 default floor.

### Gate failure-set comparison (HEAD → after)

| run | compared | failing | delta |
|---|---|---|---|
| HEAD | 308 | 53 (46 size / 4 capture / 3 pixels) | — |
| after change | 308 | 52 | only `embed-preview` eclipse pixels 0.550% **disappeared** |

`embed-preview` renders YouTube/Vimeo iframes (network-dependent paint) — the
vanished failure is pre-existing flake, not this change; no file it renders
was touched. **No new failure appeared on any slug; no failure detail changed;
split-button not failing in either run.** Source scope corroborates: the only
files edited render solely on the split-button page, and
`split-button.css` is imported only by `SplitButton.tsx` / `SplitButton.svelte`.

### Classification

All split-button deltas **expected** (new tone rows; shadow per ruling). No
diff on any component other than split-button. No stop condition triggered.

### Baselines

No baseline file modified. No `--update` passed. `test/visual/out/`
(gitignored) holds the report artifacts; native GPUI/jetstream baselines
untouched (no Rust changed; native renderers unaffected by this CSS-only
change). Refresh is the orchestrator's separate approval.

## 5. Validation (step 5)

| Command | Exit | Notes |
|---|---|---|
| `bun install` | 0 | 234 packages |
| `effigy docs:lint` | 0 | 170 contracts, 42 operator guides, 12 parity targets, … |
| `effigy test:components` | 0 | 38 files / 810 tests |
| `effigy test:parity` | 0 | 2 files / 158 tests (incl. new specimen rows) |
| `effigy docs:check` | 0 | rewrote `packages/tokens/artifacts/rust/*` (card warning) |
| `git checkout -- packages/tokens/artifacts/rust/` | 0 | restored; nothing from that directory committed |
| `git diff --check` | 0 | — |
| `git status --porcelain` | — | only the writable paths below |

`effigy visual:report` (step 4) exit 0 (report mode never fails the process),
run twice; results in §4.

## 6. Acceptance criteria

- [x] `split-button.css` implements the primary danger elevation shadow per
  contract §8 — verified at computed-style level (§1).
- [x] Success and warning across primary, secondary, ghost — derived from
  SplitButton's own danger rules, only status colour substituted (§2).
- [x] Svelte and React specimens show all four tones, identical (§3, §4
  parity: 0.0011% sub-pixel).
- [x] Visual diffs enumerated and classified in this log (§4); no baseline
  file modified.
- [x] No component source, props, public API, token, or Rust changed.
- [x] No unrelated working-tree edits staged or committed (specimens under
  `packages/svelte/components/src/` were clean before and after this work).
- [x] `docs:lint`, `test:components`, `test:parity`, `docs:check`,
  `git diff --check` all exit 0 (§5).
- [x] Batch log records commands, exit states, and the diff table.

## 7. Changed paths

```
 packages/core/src/styles/split-button.css                         | shadow + success/warning tones
 packages/svelte/preview/src/specimens/SplitButtonSpecimen.svelte   | Success + Warning rows
 packages/react/preview/src/gallery/specimens/SplitButtonSpecimen.tsx | Success + Warning rows
 docs/logs/2026-08/11-g13-008-split-button-tone-parity.md           | this log
 PAPERCUTS.md                                                       | friction entry
```

## 8. Notes

- `packages/svelte/preview/artifacts/recipe-inventory.json` not regenerated:
  its generator is not wired into any gate and the inventory is already stale
  vs the b006 hooks (button `-success-shadow` absent), so adding the three
  `-shadow` hooks changes nothing gated.
- The visual gate at HEAD is not green (53 failing pairs, both axes) — see
  the PAPERCUTS entry; a before/after comparison is required to distinguish
  pre-existing debt from a card's regressions.
