---
title: The header search looked dead because focus had no visual at all
status: complete
owner: Poodle core
updated: 2026-08-07
tags: [log, poodle-node, poodle-render, text-input, focus]
---

## Symptom

"The search input in the header doesn't work — can't click into it or type
anything."

## What Was Actually Happening

The field worked. Driven with the click driver: clicking focuses it
(`focused=yes`), typing lands, and three keystrokes issued across three
separate frames all registered, so focus survives repaints.

What it did **not** do was look any different once clicked. No focus ring, no
caret — a clicked field was pixel-identical to an idle one. There is no reason
for anyone to keep typing into a control that gives no sign of having received
the click, so "doesn't work" is a fair reading of it.

(My own first attempt to reproduce also failed for a boring reason: I clicked
`x=886`, which is the field's left border. Worth noting because it briefly
looked like confirmation of the bug.)

## Cause

The vocabulary had no focus channel. `NodeStyle` carried `hover` and `active`,
and nothing else — so a component could describe how it looks under the pointer
and while pressed, but not while focused. Every contract's `focus-visible` state
was therefore unrenderable.

`sidebar_nav` had already hit this and worked around it by putting its focus
ring on `active`, commented "Focus-visible: accent focus ring (contract §6/§8)".
`active` maps to gpui's *pressed* state, so that ring flashed on mouse-down and
never appeared for keyboard focus — the state the contract actually names.

## Changes

- **`NodeStyle::focus: Option<StylePatch>`** — style values applied while the
  node itself holds focus. Documented as distinct from `active`, with the
  reason: a focused-but-unpressed control showed nothing at all.
- The GPUI backend maps it to gpui's `focus(..)` styling, guarded on the node
  being focusable (which gpui requires).
- **TextInput** sets it to the contract's accent focus ring. Its own comment
  records why this matters more here than elsewhere: there is no caret either —
  a pure `Node -> element` backend cannot create a native gpui editor — so the
  ring is the only signal that typing will land.
- **`sidebar_nav`** moved its ring from `active` to `focus`.

`code_input` also resolves `color.accent.focusRing`, but for its *active slot*
— a component state, not a pointer state — so it was left alone.

## Verification

- Clicking the header search now rings it in the accent colour.
- Typing still lands, including across frames.
- `text-input` and `sidebar-nav` captures differ only in the header band
  (y 259..361) from this round's chrome changes, plus, on `text-input`, the
  animated spinner in its "Workspace" field. The focus ring itself does not
  appear in a static capture, which is correct — nothing is focused.

Green: `poodle-render` 112, node backend 5, both preview builds,
`effigy drift:handlers`, `git diff --check`.

## Still Missing

No caret. A `&Node -> AnyElement` backend cannot construct a gpui `Editor`
entity, which is the documented backend gap; the focus ring is the mitigation,
not the fix. Selection and IME are absent for the same reason. If real text
editing is wanted in the GPUI target, that needs a host-owned editor bridge
rather than anything in the vocabulary.
