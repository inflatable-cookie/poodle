# 2026-03-11 g01.008 Action And Text-Entry Primitive Contracts

## Changed

- completed the `g01.008` interactive foundation contract tranche
- extended the button family with `docs/contracts/foundation/icon-button.md`
- added the text-entry family contracts:
  - `docs/contracts/foundation/text-input.md`
  - `docs/contracts/foundation/text-area.md`
  - `docs/contracts/foundation/search-field.md`
  - `docs/contracts/foundation/editable-label.md`
  - `docs/contracts/foundation/number-entry.md`
- kept the contracts generic enough for Pug core while still reflecting the
  first real downstream pressure from Aura and Spark:
  - search and browser query flows
  - inline rename/edit flows
  - shell action controls
  - Spark-style text-input focus ownership and keybinding suppression
- made validation, submission, cancel, and icon-adorned states explicit in the
  contract layer rather than treating them as implementation flavor
- pushed GPUI accessibility obligations into every interactive contract:
  - role/name/state/value exposure
  - text-focused shortcut suppression
  - IME-safe editing semantics
  - focus transfer and restoration for editable labels
  - numeric/spinbutton accessibility semantics for number entry
- updated the contract and accessibility indexes so the new control family is
  visible from the main docs surfaces
- closed `g01.008` in the active roadmap

## Downstream Alignment

- Aura’s archived `SearchField` and `EditableLabel` patterns reinforced that
  the contracts should stay shallow on the web side while still documenting
  stronger semantics than the old ad hoc implementations did
- Spark’s archived text-input implementation reinforced that GPUI needs
  explicit contract language for:
  - caret and selection behavior
  - IME handling
  - focused text-input shortcut suppression
  - submit/cancel key semantics

## Validation

- `bun packages/tokens/scripts/build-tokens.ts`
- `git diff --check`

## Remaining

- execute `g01.009` for selection, value, and feedback primitives
- execute `g01.010` for overlays, menus, dialogs, tabs, and interaction
  primitives
- keep the GPUI accessibility baseline strict as the controls become more
  composite

## Next Task

Open `docs/roadmaps/g01/009-selection-value-and-feedback-primitives.md` and
author the next foundation control tranche for selection, value, and feedback
primitives.
