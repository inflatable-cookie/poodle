# 038 Focus Rings — Components With No Explicit Treatment

Status: ready
Milestone: side-quest (component styling, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-038-focus-treatment-coverage`
Depends on: `g13-b037` (`f7946bec`), merged
Governing refs: `docs/logs/2026-08/12-g13-037-focus-ring-radius-audit.md`
("Notes for the follow-up cards" — read this first, it pre-verified both
findings), `packages/svelte/preview/scripts/focus-ring-drift.ts`

## Goal

`037` fixed focus rings that were the wrong **shape**. This card is about rings
that are **absent**, and rings the browser stacks a second one on top of.

The maintainer's standing rule: *components that need focus rings should apply
them explicitly rather than rely on a blanket-applied thing.* Poodle is
partway there — 81 of 160 stylesheets draw a treatment, 25 both draw one and
suppress the UA outline.

## Current State — Measured

**79 stylesheets have no `:focus-visible` rule at all.** Cross-referencing each
against its Svelte component's markup:

| | count |
|---|---|
| no focusable element in the markup — correct as-is | 54 |
| markup contains something focusable | **24** |
| no matching Svelte component | 1 |

**56 stylesheets draw a ring without suppressing the UA outline.** `b037`
measured what that actually does, and it is not a specificity fight:

> Chrome applies `outline: auto` to `:focus-visible` elements regardless of
> specificity, so the hidden native inputs in checkbox/radio/switch/tri-state/
> segmented-control and the tabindex-carrying info-trigger wrappers in
> detail-item/field draw a second 1px `auto` ring (or an invisible one on the
> clipped inputs). The stacking is real and visible on the info triggers.

## Fixed By Ruling (do not re-decide)

### R1 — 24 is an upper bound from a source heuristic. Verify each.

The count came from grepping each component's markup for `<button>`, `<a href>`,
`<input>`, `tabindex`, and interactive roles. It over-counts, and the main way
it over-counts matters:

**A focusable child that is itself a Poodle component draws its own ring.**
`dialog.css` appears in the 24 because Dialog contains a close `IconButton` —
but `IconButton` has a ring, so Dialog has no gap. Only an element this
component itself owns and leaves unstyled is a finding.

Sort every candidate into: *owns an unstyled focusable* (fix), *its focusables
are nested components* (no gap), *not actually focusable* (heuristic false
positive). Record all three.

Reproduce with:

```sh
bun - <<'JS'
import { readdirSync, readFileSync, existsSync } from "node:fs";
const S = "packages/core/src/styles", C = "packages/svelte/components/src";
for (const f of readdirSync(S).filter((n) => n.endsWith(".css"))) {
  if (readFileSync(`${S}/${f}`, "utf8").includes(":focus-visible")) continue;
  const comp = f.slice(0, -4).split("-").map((w) => w[0].toUpperCase() + w.slice(1)).join("");
  const src = `${C}/${comp}.svelte`;
  if (!existsSync(src)) { console.log(`${f}: no svelte component`); continue; }
  const t = readFileSync(src, "utf8");
  if (/<button|<a\s[^>]*href|<input|<select|<textarea|tabindex|role="(button|tab|option|menuitem)"/.test(t))
    console.log(`${f}: FOCUSABLE`);
}
JS
```

### R2 — Every focusable element ends with exactly one visible indicator.

That is the whole point: not zero, not two.

- **Zero** is a WCAG 2.4.7 failure. Never remove a ring without replacing it.
- **Two** is the 56-sheet stacking. Where a component draws its indicator as
  something other than an `outline` (a border, a box-shadow, an inset ring), it
  must also set `outline: none` on the focused element, or the UA draws a
  second one over it.

Where a component's indicator *is* an outline, leave it — the UA outline is
replaced, not stacked.

### R3 — Use the existing vocabulary.

`--poodle-color-accent-focusRing` and `--poodle-border-width-focus`, matching
the component's own radius, as `037` established. Do not introduce a new focus
token, a new colour, or a per-component variation. If a component genuinely
cannot use the standard ring, say why in the log.

### R4 — Extend the existing gate. Do not add a second one.

`docs:focus-ring-drift` already exists from `037` and already walks every
stylesheet. Add the two new checks to it:

- a component that owns a focusable element and declares no focus treatment
- a component that draws a non-outline indicator and does not set
  `outline: none`

Same baseline convention: an entry needs a one-line reason, and "not looked at
yet" is not a reason.

### R5 — Verify in a browser. `037`'s trap still applies.

`element.focus()` does not trigger `:focus-visible`, and a Tab sweep misses
anything behind a roving tabindex. Use CDP `CSS.forcePseudoState`, exactly as
`037` did — the recipe is in that card and its log.

For the stacking specifically, the second ring is a 1px `auto` outline: measure
`outlineStyle` and `outlineWidth`, do not eyeball it.

## Scope

### In scope

- `packages/core/src/styles/*.css` — adding treatments, suppressing stacked UA
  outlines.
- The two new checks in `focus-ring-drift.ts` and their baseline.
- Contract §Accessibility rows where a component gains a documented focus
  treatment.

### Out of scope — stop conditions if reached

- Radius work — `037` closed that. If you find a *new* square ring, log it;
  do not fix it here.
- Changing ring colour or width (R3).
- Any component markup, logic, or Svelte/React file. **CSS only.** If a
  component cannot be given a ring without a markup change, that is a finding
  and a stop, not a licence.
- Refreshing visual baselines. Adding a ring where there was none **will**
  move pixels if a baseline captures a focused state — stop and classify.

## Required Tests

- The extended gate fails on each new check independently (plant one of each,
  watch it fail, restore), and passes clean.
- Every baseline entry carries a reason.
- For each component changed: a browser measurement under forced
  `:focus-visible` showing exactly one indicator — the component's — and
  `outlineStyle` proving the UA ring is not stacked underneath.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- **Read `037`'s log first.** It pre-verified both findings and names the
  specific components where the stacking is visible; you are not starting cold.
- R1 is the rule that matters, as it was in `037`, where 22 of 34 scan hits
  were false positives. Verify before changing.
- Run `effigy check:svelte`, `docs:lint`, `docs:focus-ring-drift`.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-038-focus-treatment-coverage`. Do not merge.

## Writable Paths

- `packages/core/src/styles/*.css`
- `packages/svelte/preview/scripts/focus-ring-drift.ts`
- `packages/svelte/preview/src/specimens/**` (only to expose a focus state
  with no specimen)
- `docs/contracts/components/*.md`
- `docs/logs/2026-08/<DD>-g13-038-focus-treatment-coverage.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy ci:web`, `docs:lint`, `docs:focus-ring-drift`,
   `git diff --check`. All green.
2. Read `037`'s log. Reproduce the R1 scan. Record the list.
3. Classify all 24 in a browser (R1, R5). Record the three buckets.
4. Give every *owns an unstyled focusable* case a ring (R3).
5. Work the 56-sheet stacking (R2). `037` names detail-item and field as the
   visible cases — start there, then the hidden native inputs.
6. Extend the gate with both checks; baseline with reasons (R4).
7. Prove the gate: plant one regression of each kind, watch each fail,
   restore.
8. Validate:
   ```sh
   effigy test:components
   effigy test:parity
   effigy check:svelte
   effigy docs:lint
   effigy docs:focus-ring-drift
   effigy docs:contract-drift
   effigy drift:recipes
   effigy svelte:surface-audit
   effigy ci:web
   git diff --check
   ```

## Acceptance Criteria

- [ ] All 24 classified, three buckets named in the log.
- [ ] Every component that owns an unstyled focusable has exactly one
  indicator.
- [ ] The stacked UA outline is gone where `037` measured it.
- [ ] The gate fails on both new regression kinds and passes clean.
- [ ] Every baseline entry has a reason.
- [ ] All step-8 commands exit 0; no baseline refreshed.

## Stop Conditions

- A component needs a markup change to be focusable-with-a-ring.
- A visual baseline moves.
- Suppressing the UA outline removes the only indicator a component had.

Stop with exact paths, commands, and the smallest unresolved question.
