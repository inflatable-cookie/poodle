# PickerShell

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `PickerShell`
- Layer: `composites`
- Summary: a reusable workflow shell for selecting one or more items from a searchable candidate set
- In scope: title/description, search toolbar slot, selected-summary region, result list region, confirm/cancel footer, inline/popover/modal posture
- Out of scope: domain-specific relation logic, fetch policy, item renderer semantics, destructive confirmation policy

## 2. Core Rule

`PickerShell` owns workflow framing, not item semantics.

Hosts still own:

- which candidates exist
- query execution
- selection state
- confirm/cancel consequences

## 3. Variants

- `inline`: picker stays embedded in surrounding content
- `popover`: picker behaves like a compact transient chooser
- `modal`: picker behaves like a larger focused selection task

Variant changes posture and layout emphasis.
It does not change selection meaning.

## 4. Accessibility

- the picker needs a stable accessible name
- search controls remain before result candidates in focus order
- picker status such as result count and selection count should remain textual
  and announceable
- selection summary must remain textual
- confirm/cancel actions remain explicit and discoverable
- GPUI-native accessibility mapping notes: GPUI must preserve picker title, search grouping, candidate list semantics, selection summary, and confirm or cancel actions without relying on HTML dialog/popover defaults

## 5. Next Task

Build concrete workflows such as `RelationPicker` on top of `PickerShell` instead of redefining picker framing per feature.
