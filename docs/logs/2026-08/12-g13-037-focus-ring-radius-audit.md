---
title: g13 batch 037 — focus rings, square outlines on rounded controls
status: complete
milestone: side-quest (component styling, outside the g13 IR lane)
owner: Poodle core
updated: 2026-08-12
tags: [log, g13, focus, a11y, styles, gate]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/037-focus-ring-radius-audit.md` on branch
`thread/g13-037-focus-ring-radius-audit`: browser-verified all 34 scan
candidates (R1), fixed the 7 confirmed square rings on their component's own
radius (R2) with fills agreeing (R3), and gated the pattern with
`effigy docs:focus-ring-drift` (R4), baselining the 5 intentional squares with
reasons. No visual baseline was refreshed; `git diff --check` and all step-7
commands exit 0. Nothing in the two out-of-scope findings was touched.

Per the card's worker rules: no sub-agents; sources read directly; every
candidate was verified in a browser before any change; only the card's
writable paths were staged.

## Baseline (step 1)

`effigy ci:web`, `effigy docs:lint`, `git diff --check` all green at HEAD
(`aa99121b`) on the card branch.

## Scan reproduction (step 2)

The R1 script reproduced 34 candidates across 33 stylesheets (stepper appears
twice). Exact list is in the three buckets below; every one was measured in a
browser, not assumed.

## Browser verification (R5) — method

Each candidate's specimen (`#components/<slug>` on the Svelte preview,
port 4174, `eclipse` theme, `compact` density, `md` size) was loaded and the
pseudo-state forced through CDP — not `element.focus()`, which never triggers
`:focus-visible`, and not a Tab sweep, which misses roving tabindex:

```js
const cdp = await page.context().newCDPSession(page);
await cdp.send("DOM.enable"); await cdp.send("CSS.enable");
const { root } = await cdp.send("DOM.getDocument");
for (const nodeId of await cdp.send("DOM.querySelectorAll", { nodeId: root.nodeId, selector: SEL }).nodeIds) {
  await cdp.send("CSS.forcePseudoState", { nodeId, forcedPseudoClasses: ["focus-visible", "focus"] });
}
```

Then measured computed `borderRadius`/`outline*` of every element actually
drawing a ring (not just the scan's base selector — a ring can land on a
sibling or descendant), plus each ring corner's distance to every visible
rounded ancestor's corner (the HistoryCentre defect is the ring's square
corner sitting in a rounded surface's corner notch or crossing its arc).

Three candidates are not rendered by their specimens in the state the scan
assumed (`message-center__content--interactive` needs `onItemSelect`;
`list-card__leading[data-interactive="true"]` needs a context menu;
`selection-summary__chip-activate` needs an activatable chip). Each was
verified by injecting exactly the attribute/class the component emits in that
state, then forcing the pseudo-state. MessageCenter's surface is portalled, so
its rows live outside the specimen section; measured document-wide.

## Bucket 1 — confirmed square ring on a rounded control (fixed, 7)

All measured under forced `:focus-visible`; ring `outline` 2px solid
`--poodle-color-accent-focusRing`, `borderRadius` 0 before the fix, ring
corner within/near a rounded surface's corner arc.

| Candidate | Evidence |
|---|---|
| `stepper.css` `.poodle-stepper__summary` | Full-width row (478 of the 480px track), inset ring (`-0.125rem`), ring corners 1.4px from the track's 10px-radius corners — inside the corner notches. The HistoryCentre shape exactly. |
| `stepper.css` `.poodle-stepper__trigger` | First and last step columns span to the track corners; their ring corners sit 1.4px from the track's rounded corners (vertical orientation same). Middle columns clear, but the class is one surface — fixed as a whole, matching the rerun button's already-rounded ring. |
| `filter-toolbar.css` `.poodle-filter-toolbar__header--button` | Full-width header row (936 of 958px toolbar); ring corners at the toolbar's 10px-radius corners (6.8px inside the arc). |
| `changed-files.css` `.poodle-changed-files__toggle` | Header row spanning the card; ring corner 2.4px inside the card's 10px-radius arc — the ring stroke crosses the rounded corner. |
| `message-center.css` `.poodle-message-center__content--interactive` | Full-content-width row (390 of 448px list), inset ring (`-0.125rem`); verified via injected class in the portalled surface. Same HistoryCentre shape. |
| `hover-card.css` `.poodle-hover-card__trigger` | Ring corner 2.9px inside the surface's 10px-radius arc — the ring stroke abuts the rounded corner (specimen places the trigger at the surface corner). |
| `popover.css` `.poodle-popover__trigger` | Same measurement as hover-card trigger. |

## Bucket 2 — correctly square (left alone, 5 + 1 variant)

Square rings with no rounded-edge conflict — the element is genuinely square,
its ring corner floats clear of any rounded surface (measured ≥4.6px inside
the nearest arc, no stroke crossing). Baselined with reasons in the gate.

| Candidate | Reason |
|---|---|
| `agent-subagent.css` `.poodle-agent-subagent__action` | Transparent text action button; ring corner 4.6px inside the card's 6px arc — floats clear. |
| `agent-plan-record.css` `.poodle-agent-plan-record__toggle` | Transparent text toggle; same 4.6px measurement. |
| `agent-question.css` `.poodle-agent-question__dismiss` | Small transparent dismiss button; ring corner 21px from the nearest rounded edge. |
| `menu.css` `.poodle-menu__trigger` | Inline text trigger; ring clear of every rounded edge. |
| `resize-handle.css` `.poodle-resize-handle` | Square grip hit-target; a square ring is the intended shape. |
| `editable-label.css` flush variant (`.poodle-editable-label__display` under `[data-variant="flush"]`) | Explicitly `border-radius: 0` by design — inline label, transparent, no fill. Not a gate entry (the base class carries radius), but noted here: square by intent. |

## Bucket 3 — scan false positives (22)

The scan's regex ("this file has a `:focus-visible` outline rule and the base
selector never declares `border-radius`") missed that the radius arrives
elsewhere. Browser measurement, ring `borderRadius` at forced focus:

| Candidate | Why the ring is already rounded |
|---|---|
| `accordion.css` `.poodle-accordion__trigger` | Radius in the focus rule itself (`calc(radius-control - 0.125rem)`), 4px measured. |
| `collapsible.css` `.poodle-collapsible__trigger` | Same pattern, 4px measured. |
| `selection-summary.css` `.poodle-selection-summary__chip-activate` | Radius in the focus rule (`calc(radius-control - 0.0625rem)`); injected class measured 5px. |
| `data-table.css` `.poodle-data-table__sort` | Radius in the shared comma-list focus rule, 6px measured. |
| `relation-picker.css` `.poodle-relation-picker__item-button` | Radius in the focus rule, 6px measured. |
| `rating.css` `.poodle-rating[data-mode="fractional"]` | Radius in the focus rule, 6px measured. |
| `text-link.css` `.poodle-text-link` | Radius in the focus rule, 2px measured. |
| `token-input.css` `.poodle-token-input__remove` | Radius in the focus rule, 999px measured. |
| `audio-player.css` `.poodle-audio-player__play` | Base class radius 999px (circle), 15984px measured. |
| `editable-label.css` `.poodle-editable-label__display` | Base class radius-control, 6px measured (flush variant square by intent, see bucket 2). |
| `radio.css` `.poodle-radio__control` | Ring lands on the sibling indicator (999px circle), not the control. |
| `radio-group.css` `.poodle-radio-group__control` | Same. |
| `checkbox.css` `.poodle-checkbox__control` | Ring lands on the sibling indicator, 5px measured. |
| `switch.css` `.poodle-switch__control` | Ring lands on the sibling track (999px). |
| `tri-state-switch.css` `.poodle-tri-state-switch__control` | Ring lands on the sibling segment (999px). |
| `segmented-control.css` `.poodle-segmented-control__control` | Ring lands on the sibling label, 4px measured. |
| `detail-item.css` `.poodle-detail-item__info-trigger` | Ring lands on the info icon (`radius-pill`). |
| `field.css` `.poodle-field__info-trigger-wrap` | Ring lands on the info icon (`radius-pill`). |
| `card-toggle-group.css` `.poodle-card-toggle-group__option` | Ring lands on `.poodle-card`, whose radius lives in card.css — 10px measured. |
| `card-radio-group.css` `.poodle-card-radio-group__option` | Same. |
| `list-card.css` `.poodle-list-card__leading[data-interactive="true"]` | Leading's base class is a 999px circle; injected `data-interactive="true"` measured 999px. |
| `tree.css` `.poodle-tree__item` | Ring lands on `.poodle-tree__row`, radius `calc(radius-control - 0.125rem)`, 4px measured. |

(The editable-label flush note in bucket 2 is not a separate scan item — it
belongs to scan item 13, whose base class is this false positive. Buckets
total 7 + 5 + 22 = 34; stepper's two entries were both confirmed.)

## Fixes (steps 4, R2/R3)

Every confirmed ring now rounds on `var(--poodle-radius-control)` — the
component's own radius vocabulary everywhere (stepper rerun, list rows,
triggers), and per R2 the default where the element itself had none (the
stepper summary/trigger, filter-toolbar header button, changed-files toggle,
message-center row, hover-card/popover triggers). No invented radii; ring
colour, width and offset untouched.

R3 — fills agree: the two full-width rows with hover fills got the radius on
their base block so the fill rounds with the ring (`stepper__summary`,
`message-center__content`); every other fixed element is transparent with no
fill. `stepper__trigger`'s hover/current tints live on the step column, not
the trigger, so they are unaffected. The message-center row carries the
radius on both the base (fill) and the focus rule (ring), since the ring rule
names only the `--interactive` class.

## Gate (step 5, R4)

`packages/svelte/preview/scripts/focus-ring-drift.ts` — standalone selector +
call in `lint-docs.ts`, wired as `effigy docs:focus-ring-drift`.

Smarter than the scan by necessity: it flags a ring only when **no rule in
the whole corpus** gives the **ring element** a radius. "Ring element", not
the scan's naive base — checkbox draws its ring on `.poodle-checkbox__
indicator`, card-toggle-group on `.poodle-card` (whose radius is in
card.css), data-table's sort on a shared comma-list rule. This is what keeps
the baseline (5 entries) well under the fix list (7) instead of a parking lot
for 21 false positives — the stop condition on gate design did not trigger.

Baseline (each with a one-line reason in the file): `agent-subagent__action`,
`agent-plan-record__toggle`, `agent-question__dismiss`, `menu__trigger`,
`resize-handle`. The gate also fails on stale baseline entries (ratchet).

Proven both ways (step 6): removed the stepper trigger's radius →
`focus-ring-drift` exits 1 naming `.poodle-stepper__trigger`; restored →
exits 0. Baseline entries all still flagged; none stale.

## Validation (step 7)

```text
effigy test:components        exit 0
effigy test:parity            exit 0
effigy check:svelte           exit 0
effigy docs:lint              exit 0   (includes the new focus-ring check)
effigy docs:focus-ring-drift  exit 0
effigy docs:callback-drift    exit 0
effigy drift:recipes          exit 0
effigy svelte:surface-audit   exit 0
effigy ci:web                 exit 0
git diff --check              clean
```

Required tests: the gate fails on a planted square ring and passes clean
(proven above); every baseline entry carries a reason; every changed
component re-measured under forced `:focus-visible` with radius non-zero and
outline present — stepper summary/trigger 6px, filter-toolbar header button
6px, changed-files toggle 6px, hover-card trigger 6px, popover trigger 6px,
message-center interactive row 6px, all `2px solid` focusRing outline.

No visual baseline refreshed. Both web previews share `poodle-core/styles`,
so the Svelte↔React cross-framework gate diffs identical CSS on both sides;
the native previews do not consume these stylesheets. Nothing in this batch
touches a committed baseline artifact.

## Notes for the follow-up cards ("What This Card Is Not")

- **79 sheets with no focus treatment**: confirmed at the margin — every
  focusable control that draws a ring does so through these core styles, and
  the corpus-wide count of sheets with *no* `:focus-visible` outline rule is
  the follow-up's subject. Not started here.
- **56 sheets with a ring but no UA-outline suppression**: measured live —
  Chrome applies `outline: auto` to `:focus-visible` elements regardless of
  specificity, so the hidden native inputs in checkbox/radio/switch/tri-
  state/segmented-control and the tabindex-carrying info-trigger wrappers in
  detail-item/field draw a second 1px `auto` ring (or an invisible one on
  the clipped inputs). The components' own rings coexist with it; not this
  card's scope, but the stacking is real and visible on the info triggers.

## Writable paths touched

- `packages/core/src/styles/{stepper,filter-toolbar,changed-files,message-center,hover-card,popover}.css`
- `packages/svelte/preview/scripts/{focus-ring-drift.ts (new), lint-docs.ts}`
- `tasks/effigy.tasks.toml`
- `docs/contracts/components/{stepper,filter-toolbar,changed-files,message-center,hover-card,popover}.md`
- `docs/logs/2026-08/12-g13-037-focus-ring-radius-audit.md`
- `PAPERCUTS.md` (entry added)
