# Menu item explicit accessible name

Status: dependency-ready assessment; not promoted or dispatched
Captured: 2026-09-14
Owner: Poodle Chatterbox
Source: cross-repository papercut report from Tom, verified against Poodle and
Longhorn sources

## Issue

Poodle's Svelte and React `MenuSurface` render each non-separator `MenuItem` as
a button with `role="menuitem"`, `role="menuitemcheckbox"`, or
`role="menuitemradio"`. The button contains visible `item.label` text but does
not set an explicit accessible-name attribute.

A conforming browser accessibility tree derives the name from that visible
text, so this is not evidence that Menu is unnamed for ordinary assistive
technology. Longhorn's current serializer, however, deliberately falls back to
text for only a bounded role list that omits menuitem and treeitem roles. Its
serialized Menu items are therefore empty unless Poodle projects the semantic
label explicitly.

## Evidence

- `packages/svelte/components/src/MenuSurface.svelte` and
  `packages/react/components/src/MenuSurface.tsx` render visible
  `item.label` text and the three menu item roles without `aria-label`.
- Menu metadata glyphs and shortcut labels are already `aria-hidden`, so the
  browser-computed name is effectively the item label today.
- `MenuItem.label` is required in both web public types and is already the
  semantic label used for navigation and display.
- Shared Rust Menu rendering already assigns
  `item.a11y.label = Some(entry.label.clone())` before projecting the three
  menu item roles. The proposed web change closes a representation gap rather
  than changing cross-runtime semantics.
- Poodle commit `1d8e6aeab861055407010b0dd5c0203c1c84dfb5` established the same
  explicit-name rule for Tree items. It is an ancestor of both `v0.3.0` and
  `v0.4.1`; Figmatic's retained `0.3.0` pin therefore already contains that
  Tree repair and creates no new Poodle Tree task.
- Longhorn currently pins Poodle core and Svelte `0.3.0`. Its serializer checks
  `aria-labelledby`, `aria-label`, associated labels, image alt text, then a
  bounded text-fallback role list that excludes menuitem and treeitem.
- Poodle Queue task `e9cb23d3-ae92-4774-85f8-321b157e9460` (`g18.036`, PR
  #272) is still under review and reserves the shared g18 roadmap/dispatch
  surfaces. No sibling Poodle task should be promoted through those paths
  until its hook-owned closeout finishes.

## Chatterbox assessment

Poodle owns the reusable correction. Every non-separator MenuSurface item
should set `aria-label={item.label}` / `aria-label={item.label}` in Svelte and
React, covering action, checkbox, radio, disabled, and submenu-parent rows.
The visible label must remain unchanged, and the explicit value must be exactly
the same required `item.label`; add no override prop or consumer-named adapter.

Menu and ContextMenu both reuse MenuSurface, so their contracts and paired web
tests should state and prove the rule. Menubar, SplitButton, ListCard, and other
independent menuitem renderers are not evidence-backed scope for this lane and
must not be swept in speculatively. GPUI already satisfies the semantic rule
and needs no implementation change.

Impeccable classification: accessibility refinement. There is no visual,
layout, copy, focus, keyboard, or workflow change. Explicit naming makes the
existing visible/semantic label stable for bounded serializers without
altering browser behavior.

## Dependency and sequencing

1. Let `g18.036` finish independent review, merge, and lifecycle closeout.
2. Promote one successor Poodle task (recommended `g18.037`) from this note and
   create its worker handoff. Its canonical dependency is completed `g18.036`,
   a portfolio sequencing edge rather than an implementation dependency.
3. Implement and review the paired MenuSurface correction. Do not edit
   Longhorn, Figmatic, or their serializers/pins in the Poodle lane.
4. Return to Chatterbox after closeout. Combine this compatible fix with the
   accepted g18 consumer-sweep batch in the next operator-approved npm patch;
   do not open a release task from this assessment.
5. Longhorn adoption depends on the exact published Poodle patch version, not
   on a Poodle branch or merge commit. Longhorn owns a separate pin/adoption
   lane after registry proof. Figmatic may adopt a newer Poodle version on its
   own schedule; its Tree-name requirement is already present in `0.3.0`.

## Ready-handoff recommendation

After `g18.036` is complete, promote a general web worker handoff with:

- **Outcome:** every MenuSurface non-separator item exposes its required
  `item.label` as an explicit accessible name in Svelte and React; Menu and
  ContextMenu inherit the behavior; visible text and all interactions stay
  unchanged.
- **Owned paths:** both web `MenuSurface` implementations; paired Menu and
  ContextMenu tests as needed; Menu and ContextMenu component contracts; the
  successor task/handoff. No core DOM, native, consumer, workflow, or release
  paths.
- **Oracle:** action, checkbox, radio, disabled, and Svelte submenu-parent rows
  each have an exact `aria-label` equal to their visible `item.label`; separator
  rows remain unnamed; shortcut/check/submenu metadata stays hidden; Menu and
  ContextMenu activation, checked state, focus movement, and dismissal tests
  remain green in both frameworks.
- **Validation budget:** paired focused Menu/ContextMenu component tests during
  implementation, then one final `effigy ci:web` and `git diff --check`; no
  overlapping broad boards.
- **Stop:** any proposal for a new public naming prop, a general role-policy
  sweep, a Longhorn serializer change, a Figmatic/Longhorn pin edit, GPUI work,
  or release mutation returns to Chatterbox.

## Next check

When `g18.036` reaches hook-owned `done`, reconcile this note into the g18
roadmap/task/dispatch surfaces and delete it in the same planning commit. Do
not submit the resulting handoff without a later explicit dispatch instruction.
