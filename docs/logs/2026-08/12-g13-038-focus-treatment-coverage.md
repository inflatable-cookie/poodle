---
title: g13 batch 038 — focus treatments, absent rings and stacked UA outlines
status: complete
milestone: side-quest (component styling, outside the g13 IR lane)
owner: Poodle core
updated: 2026-08-12
tags: [log, g13, focus, a11y, styles, gate]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/038-focus-treatment-coverage.md` on branch
`thread/g13-038-focus-treatment-coverage`. A previous run hit its 45-minute
deadline mid-work with 28 stylesheets and the extended gate uncommitted and
unlogged; this run resumed it. Completed in order:

1. Re-verified every new `:focus-visible` rule the earlier run added (27,
   unclassified — the R1 violation) in a browser under CDP
   `CSS.forcePseudoState`. Kept 25, removed 2 (the detail-item/field info-icon
   rings — the focusable is a nested Popover trigger that already draws its own
   ring, exactly the card's Dialog/IconButton case).
2. Finished the R2 stacking work: fixed the remaining real stacking, fixed
   three gate bugs that were mislabelling findings, and added explicit rings
   to the controls that genuinely had none (audio-player seek/volume/speed,
   order-by drag handle).
3. Proved the extended gate both ways (absent, stacked, and the 037 radius
   check each planted, failed, restored), and validated the whole step-8 suite.

Per the card's worker rules: no sub-agents; sources read directly; every rule
verified in a browser before any change; only the card's writable paths
staged.

## Resuming the interrupted run

The working tree carried 28 modified stylesheets and the extended
`focus-ring-drift.ts`, nothing committed, no log. The gate's absent-treatment
check already passed (the run's 27 rules + dialog/drawer baselines), but
nothing classified the 27, and the stacked-UA check failed with 25 findings.
Both were the resume work; the run's changes were treated as unverified
proposals, not as decisions.

## R1 — the 27 new `:focus-visible` rules, browser-verified

Method (per card R5 / 037's recipe): each target loaded on the Svelte preview
(port 4174, `eclipse` theme, `compact` density, `md` size), pseudo-state
forced through CDP (`DOM.enable`/`CSS.enable`, `DOM.querySelectorAll` +
`CSS.forcePseudoState` with `["focus-visible", "focus"]`), then the whole
specimen scanned for elements with a rendered outline. Machine-driven
controls (fader/knob/drag-number-field entries) were exercised through the
real keyboard path (focus root, Enter → entry opens and focuses), not forced.
Branches the specimen does not render (breadcrumbs `a`, block-editor textarea,
log-list scroll button) were verified by injecting exactly the element the
component emits in that branch, as 037 did for attribute/class states.

Kept — 25. Every one targets a focusable the component itself owns (native
button/anchor/input/textarea, or the toolbar's own `tabindex="0"` wrapper),
and each was verified to draw exactly one indicator:

- block-editor tool-btn + input (textarea injected)
- breadcrumbs `a` (injected) + `button`
- drag-number-field/fader/knob `__entry` (real keyboard path; the machine
  `data-focus` ring does NOT fire for entry focus — the entry had no
  indicator before, and the run's outline is the only one while editing)
- list-card-counter `a`
- log-list filter-btn + scroll-btn (injected)
- select `__option` (real click opened the listbox)
- toast-stack `__dismiss`
- toolbar (wrapper ring on the tabindex-carrying root)
- validation-summary entry `a`
- video-player big-play, `__btn`, `__volume`, progress-bar `:has(seek)` ring
- plus the 7 suppression rules (number-input steppers, select
  trigger/clear/indicator, text-input clear, video seek, waveform-display
  root) — each verified to leave exactly one indicator: the component's
  `:focus-within`/machine ring, UA outline gone.

Removed — 2. `detail-item.css` and `field.css`: the run retargeted the
info-icon ring selector to `.poodle-detail-item__label .poodle-popover__trigger
:focus-visible .poodle-detail-item__info-icon`, suppressing the Popover's own
ring (`outline: none` on the trigger, pre-existing) and drawing a second ring
on the icon instead. The focusable is a nested Poodle component — Popover's
trigger div (`tabindex="0"`, `role="button"` per `popoverParts`) already draws
the standard ring in popover.css. Measured live: icon ring at 1px offset around
a 1.25em icon. Removed both rules (the `outline: none` suppression and the
icon ring); the Popover's ring is now the indicator, matching every other
popover trigger. Same for field. (The old pre-run selectors were dead —
`.poodle-detail-item__info-trigger` is a plain span, never focusable — so
pre-run state was zero indicators.)

Counts: **25 kept, 2 removed.**

## R2 — the 25 stacked findings, dispositioned

Working through the gate's list revealed three gate bugs and a small set of
real issues. Every item was browser-verified before deciding.

Gate bugs fixed in `focus-ring-drift.ts`:

1. **Comma-list miscompare** — `stackedTreatment` compared the first
   `:focus-visible` part's last compound against the whole selector's last
   compound, so multi-class comma lists (`.poodle-data-table__sort:focus-visible,
   .poodle-data-table__row-action-btn:focus-visible, …`) were read as
   "ring lands on a different element" → stacked. Now evaluated per part: a
   rule stacks only if some part's ring element differs from its focused
   compound (descendant combinator), plus the existing `:focus-within`/
   `[data-focus]`/box-shadow/`:has(:focus-visible)` shapes. This cleared
   audio-player (play/mute), data-table (toolbar-btn/sort/row-action-btn),
   select option, color-picker trigger/gradient/swatch, agent-chat-input
   action, knob/fader/drag-number-field entries — all of which draw an
   outline ON the focused element (replaced, not stacked; each verified live).
2. **`outlineSuppressed` only accepted `none|0`** — an author outline on the
   focused element also replaces the UA `outline: auto` (author origin beats
   the UA sheet), so it needed counting. Now `outlineCovered` accepts any
   outline declaration, corpus-wide for classes (shared rules:
   selection-summary's chip-activate ring lives in selection-summary.css),
   same-file-only for bare tags.
3. **Heuristic false positives in `ownedFocusables`** — `tabindex="-1"`
   elements are not Tab-reachable and never draw the UA `:focus-visible`
   outline (select trigger-area, order-by surface, keyboard wrapper,
   mod-matrix-grid controls, filter-builder surface), and `type="hidden"`
   inputs are never rendered (token-input). Both are now skipped. One real
   bug surfaced by the tag fix: bare-tag coverage must be same-file scoped —
   `.poodle-breadcrumbs button:focus-visible` must not excuse number-input's
   classless steppers buttons (the regression proof caught this; fixed).

Real fixes (browser-verified before and after):

- **agent-chat-input action** — measured TWO rings when the action is focused
  (its own outline + the field's `:focus-within` ring; the action sits inside
  the field). Changed `:focus-visible` to `outline: none`; the field ring is
  now the single indicator, consistent with the editor.
- **audio-player seek/volume/speed** — measured UA-only (`outline: auto 1px`);
  the component owns them and drew no treatment. Added the standard ring
  (R3 vocabulary, radius `--poodle-radius-control` on the range inputs).
- **order-by drag-handle** — UA-only; it lives in the portalled panel, so the
  trigger-wrap `:focus-within` ring never covers it. Added the standard ring.
  The `.poodle-order-by__item:focus-visible` rule is dead (item div is never
  focusable) — logged as a papercut, not touched.

Everything else in the 25 was the gate bugs above; after the fixes the gate
passes with no `STACK_BASELINE` entries needed.

## Gate (R4)

`focus-ring-drift.ts` now runs three checks: the 037 radius check, the 038
absent-treatment check, and the 038 stacked-UA check. Baselines with reasons:
the 5 037 square-by-intent entries (unchanged), the 2 absent entries
(dialog/drawer — keyboard-unreachable backdrops + nested-component rings,
unchanged), and 0 stack entries. Nothing new was square-by-intent this batch —
all new rings carry `--poodle-radius-control` or the component's own radius.

Proven both ways (each planted, watched fail, restored):

- absent: removed toast-stack's dismiss ring → `[toast-stack.css] owns a
  focusable element and declares no focus treatment`
- stacked: removed number-input's stepper `outline: none` → `[number-input.css]
  <button> (focused element lacks outline: none)`
- radius (037's check, still intact): removed stepper trigger's ring radius →
  `[stepper.css] .poodle-stepper__trigger … with no border-radius anywhere`

## Validation

```text
effigy test:components        exit 0 (1012 tests)
effigy test:parity            exit 0
effigy check:svelte           exit 0
effigy docs:lint              exit 0   (includes the focus-ring check)
effigy docs:focus-ring-drift  exit 0   (radius + absent + stacked all OK)
effigy docs:contract-drift    exit 0
effigy drift:recipes          exit 0
effigy svelte:surface-audit   exit 0
effigy ci:web                 exit 0   (one transient svelte-check failure from
                                        a stale bunx cache; clean on re-run)
git diff --check              clean
```

One note: `effigy ci:web` failed once mid-run on `check:svelte-components`
with 9 pre-existing type errors under a stale bunx-resolved svelte-check
(676 files); the same task passes at 701 files / 0 errors on re-run. Not a
code issue.

No visual baseline refreshed. The `outline: none` additions and ring additions
affect only focus states; no committed baseline captures them.

## Writable paths touched

- `packages/core/src/styles/*.css` (28 files: the earlier run's treatments plus
  this run's removals in detail-item/field and rings in audio-player/order-by,
  agent-chat-input action suppression)
- `packages/svelte/preview/scripts/focus-ring-drift.ts` (gate fixes)
- `docs/contracts/components/{audio-player,block-editor,detail-item,field,
  list-card-counter,agent-chat-input,select,toolbar,breadcrumbs,toast-stack,
  log-list,validation-summary,drag-number-field,fader,knob,video-player,
  waveform-display,number-input}.md`
- `docs/logs/2026-08/12-g13-038-focus-treatment-coverage.md`
- `PAPERCUTS.md` (037 stacking entry marked resolved; dead order-by rule
  logged)

Nothing in the two out-of-scope files (SidebarNav, settings-shell.css) was
touched. CSS only — no component or markup files changed.
