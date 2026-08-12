# 043 Container Queries That Query Themselves

Status: ready
Milestone: side-quest (component layout, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-043-self-container-queries`
Depends on: none
Governing refs: `docs/contracts/components/detail-item.md`,
`docs/contracts/components/detail-section.md`,
`docs/contracts/components/form-layout.md`,
`PAPERCUTS.md` (2026-08-12, the SettingsShell entry with the measurement)

## Goal

An element **cannot be matched by its own container query**. A rule inside
`@container` resolves against the nearest *ancestor* container, so a stylesheet
that puts `container-type` on `.poodle-x` and then writes `@container … {
.poodle-x { … } }` is not asking what it thinks it is asking.

**All five** of Poodle's container-query stylesheets do this.

## Current State — Measured

| Stylesheet | Class that declares `container-type` **and** is targeted inside its own `@container` |
|---|---|
| `detail-item.css` | `.poodle-detail-item` |
| `detail-section.css` | `.poodle-detail-section` |
| `detail-section-group.css` | `.poodle-detail-section-group` |
| `form-actions.css` | `.poodle-form-actions` |
| `form-layout.css` | `.poodle-form-layout` |

That is every stylesheet in `packages/core/src/styles` that uses `@container`.

**The reference case, measured in the SettingsShell specimen:**
`detail-item.css` declares `container-type: inline-size` on
`.poodle-detail-item` (`:15`) and writes `@container (max-width: 26rem)` rules
targeting `.poodle-detail-item[data-layout="inline"]` (`:183`). Those resolve
against the nearest ancestor container — a page-wide
`.poodle-detail-section` — so at an item width of **240px** the 26rem query
never fires. The label column takes its `11.25rem` max and the value column
resolves to **20px**:

```
DetailItem width: 240px   resolved columns: 180px 20px 0px
```

At 20px, `word-break: break-word` (`:81`) breaks values character by character
— "Da rk", "Co m pa ct". Any narrow grid cell reproduces it.

## Fixed By Ruling (do not re-decide)

### R1 — Five candidates. Verify each; they are not all the same failure.

A self-query is always wrong, but it fails in two different ways and the fix
differs:

- **Wrong scale** — an ancestor container exists, so the rule fires, just
  against the wrong box. `detail-item` is this: it responds to the section's
  width instead of its own.
- **Dead** — no ancestor container, so the rule never fires at all.
- **Accidentally fine** — the ancestor happens to be the box you wanted. Then
  the rule is right and the `container-type` on the element is the confusing
  part.

Classify all five in a browser before changing any. Record the three buckets,
as `037` and `038` did. Do not assume the reference case generalises.

### R2 — Split concerns that live at different scales. Do not move a block wholesale.

`detail-item.css`'s single `@container (max-width: 26rem)` block does **two
jobs at two scales**, and only one of them is misplaced:

- `[data-span] { grid-column: 1 / -1 }` — about the item's position in the
  **section's** grid. Correctly keyed to the section. **Leave it.**
- the inline → stacked collapse — about whether the **item** has room for two
  columns. Wrongly keyed to the section. **This is the one to move.**

Moving the whole block would break the span behaviour. Check the other four for
the same mixing before touching them.

### R3 — The mechanism is a genuine choice. The requirement is not.

Two defensible routes, and the card does not force one:

- **A wrapper.** Keep `container-type` on `.poodle-detail-item` and move the
  grid onto an inner element, so the query has an ancestor to resolve against.
  Correct and conventional; costs a markup change in both runtimes and touches
  every descendant selector.
- **Intrinsic sizing.** Drop the query for this concern and let the grid stack
  on its own — `repeat(auto-fit, minmax(min(<col>, 100%), 1fr))` and relatives.
  CSS-only; must be shown to preserve the action column and every
  layout/presentation combination.

Pick per component, say why in the log, and hold to these constraints either
way:

- No public API change. No new prop, no renamed part.
- No pixel change at widths where the layout is already correct. If a baseline
  moves at a wide width, **stop** — you have changed more than the narrow case.
- Every `data-layout` and `data-presentation` combination still renders. These
  components have three layouts and two presentations between them; a fix that
  only holds for `inline`/`default` is not a fix.

### R4 — Gate it, so the fifth one is the last.

Add a check that fails when a stylesheet declares `container-type` on a class
and targets that same class inside its own `@container` block. It is a
static-text rule, cheap, and it is the reason this went unnoticed in five
places.

Put it beside the existing drift gates in
`packages/svelte/preview/scripts/`, wire it into `lint-docs.ts`, and expose it
as `effigy docs:container-query-drift`. Baseline any *accidentally fine* case
from R1 with its reason rather than restructuring it — but say in the entry
that the `container-type` is then redundant.

## Scope

### In scope

- The five stylesheets, and the markup of any component where R3 chooses a
  wrapper.
- The new gate and its wiring.
- Contract §Layout rows where behaviour at narrow widths is now documented.
- A specimen exercising a narrow container, if none does — the reference case
  only surfaced because SettingsShell put DetailItems in a 240px cell.

### Out of scope — stop conditions if reached

- Any component not in the table.
- `word-break` / typography changes. The value column collapsing is the
  defect; how text breaks at a sane width is not this card.
- `SettingsShell`, which merely revealed it.
- Refreshing visual baselines (R3).

## Required Tests

- The gate fails on a planted self-query and passes clean.
- For each component changed: a browser measurement at a narrow width showing
  the corrected layout, and one at a wide width showing it unchanged.
- The reference case specifically: a `DetailItem` in a ~240px cell renders a
  usable value column and does not break words character by character.
- Every layout × presentation combination still renders (R3).

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- R1 first: classify before changing. `037` found 22 of 34 candidates were
  false positives; five is a small enough set to get every one right.
- Use CDP or a resize to reach narrow widths — do not infer from source.
- **Run `effigy check:svelte`**, plus `docs:lint`, `docs:contract-drift`,
  `docs:focus-ring-drift`, `drift:recipes`, `svelte:surface-audit`.
- Verify every governing-ref path resolves before relying on it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-043-self-container-queries`. Do not merge.

## Writable Paths

- `packages/core/src/styles/{detail-item,detail-section,detail-section-group,form-actions,form-layout}.css`
- `packages/{svelte,react}/components/src/{DetailItem,DetailSection,DetailSectionGroup,FormActions,FormLayout}.{svelte,tsx}`
  (only where R3 chooses a wrapper)
- `packages/{svelte,react}/components/test/{DetailItem,DetailSection,FormLayout}*.test.*`
- `packages/svelte/preview/scripts/**`
- `packages/{svelte,react}/preview/src/specimens/**`
- `tasks/effigy.tasks.toml`
- `docs/contracts/components/{detail-item,detail-section,detail-section-group,form-actions,form-layout}.md`
- `docs/logs/2026-08/<DD>-g13-043-self-container-queries.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy ci:web`, `docs:lint`, `test:components`,
   `git diff --check`. All green.
2. Classify all five (R1). Record the buckets and the evidence.
3. Fix `detail-item` first — it is the measured reference and the hardest,
   because of the two-scale split (R2).
4. Work the other four, each on its own R3 decision.
5. Add the gate; baseline anything accidentally-fine with a reason (R4).
6. Prove the gate: plant a self-query, watch it fail, restore.
7. Validate:
   ```sh
   effigy test:components
   effigy test:parity
   effigy check:svelte
   effigy docs:lint
   effigy docs:container-query-drift
   effigy docs:contract-drift
   effigy docs:focus-ring-drift
   effigy drift:recipes
   effigy svelte:surface-audit
   effigy ci:web
   git diff --check
   ```

## Acceptance Criteria

- [ ] All five classified, buckets recorded.
- [ ] No stylesheet targets a class inside that class's own `@container`,
  except a baselined accidentally-fine case with a reason.
- [ ] A `DetailItem` at ~240px renders a usable value column.
- [ ] `[data-span]` still spans — the section-scale rule survived the split.
- [ ] Nothing moved at wide widths; no baseline refreshed.
- [ ] The gate fails on a planted regression and passes clean.
- [ ] All step-7 commands exit 0.

## Stop Conditions

- A wrapper cannot be added without changing a public part name.
- Intrinsic sizing cannot preserve a layout/presentation combination.
- A visual baseline moves at a wide width.
- The gate cannot distinguish a self-query from a legitimate descendant rule
  well enough to avoid baselining most of the five.

Stop with exact paths, commands, and the smallest unresolved question.
