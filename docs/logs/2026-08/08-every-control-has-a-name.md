---
title: ci:native is green — every control that needs a name has one
status: complete
owner: Poodle core
updated: 2026-08-08
tags: [log, accessibility, poodle-render, g12.015]
---

## Result

`effigy test:jetstream-a11y`: **151 unnamed → 0**, across 135 specimens and
20,625 nodes. `effigy ci:native` exits **0** for the first time in this
campaign.

## The Shape Of The 151

Not one problem. Three, in descending size, and only the smallest was what the
papercut described ("these specimens do not set an `aria_label`").

### A Field's label named nothing (55 nodes)

`field.md`'s anatomy calls the Label the *"accessible naming anchor for the
slotted control"*, and the web target spells it `<label for={id}>`. There is no
`for` on the Rust targets, and nothing had replaced it — so a field rendered
visible label text beside a control announced as an unnamed text input.

`field()` now names its control from its own label, and only when the control
has no name of its own: an explicit `aria_label` is more specific than the
visible label and must win. One change, 55 nodes: `field`, `form-layout`,
`form-dialog`, `field-set`.

### Composites left their nested fields anonymous (61 nodes)

`token-input`, `relation-picker`, `embed-input`, `editable-list`, `ref-select`,
`filter-builder` and `time-field` all build an internal `TextInputSpec`. Each
forwarded the host's `aria_label` *if it had one* and otherwise left the field
nameless.

A nested field has a known purpose, so it now has a default name — "Add token",
"Search relations", "Embed URL", "New item", "Search references", "Filter
value", "Time" — used when the host supplies nothing. The host's own label
still wins.

The contracts prescribe none of these strings, so each is commented with why
that field has no other source of a name (the tokens beside a draft field are
not its label; a panel's search box has nothing visible next to it).

### Specimens genuinely missing labels (26 nodes)

The `text-input` specimen's 22 fields sit inside `group("Caption", …)`, which is
the same relationship `Field` has to its control — so each is named from its
group, with an index where a group holds several variants. `picker-shell` and
`ui-presentation-provider` got the same treatment.

## Why It Had Been Invisible

The papercut already recorded it: `ci:native` died at `drift:clicks` — whose
subject was deleted in `ee704699` — long before it reached the audit, so the
count regressed from zero unnoticed. Fixing the *runner* is what exposed the
work; the audit had been right all along and simply never ran.

## Verification

- `test:jetstream-a11y`: 0 unnamed. `drift:roles` still passes — every ARIA role
  a contract names is still projected.
- `ci:native`: exit 0.
- `test:native-visual` on the nine touched components: **0 failing**.
  Accessible names are not pixels.
- `poodle-render` 129, `poodle-jetstream` 161, node backend 8.

## Papercuts Closed

Both remaining `2026-08-07` entries are removed: the missing
contract-declares-events gate (now `drift:events`) and the 151 unnamed nodes.
Five papercuts remain, none of them blocking a gate.
