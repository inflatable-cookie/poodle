# 2026-03-11 g01.007 Layout Primitives And Accessibility Baseline

## Changed

- completed the first `g01.007` primitive contract tranche under
  `docs/contracts/foundation/`
- added foundation contracts for:
  - `Box`
  - `Stack`
  - `Inline`
  - `Grid`
  - `Spacer`
  - `Surface`
  - `Separator`
  - `ScrollShell`
- added `docs/contracts/foundation/README.md` so the layout/surface tranche
  reads as one bounded family instead of a pile of isolated files
- kept the layout primitives accessibility-neutral by default where that is the
  correct contract posture, rather than over-semantifying structural containers
- made `Surface`, `Separator`, and especially `ScrollShell` explicit about when
  semantic opt-in is required and how keyboard/focus behavior should work
- added `docs/specs/003-accessibility-and-assistive-technology-baseline.md` so
  accessibility now exists as a first-class normative spec instead of only as a
  research note
- updated the contract template, existing seed contracts, and architecture note
  so GPUI accessibility obligations are explicit:
  - native accessible node mapping
  - role/name/state/value exposure
  - keyboard reachability
  - visible focus
  - focus restoration
  - dynamic announcement behavior where required
- closed `g01.007` in the active roadmap surface

## Why This Matters

The web side can lean on semantic HTML for much of this behavior. GPUI cannot.
That means accessibility had to be pushed into the contract and spec layer now,
before the interactive primitives land, or the library would drift into
“accessible on web, approximate on native.”

The new baseline explicitly rejects that posture.

## Validation

- `bun packages/tokens/scripts/build-tokens.ts`
- `git diff --check`

## Remaining

- execute `g01.008` for buttons, text input, text area, search field,
  editable-label, and number-entry contracts
- execute `g01.009` for selection, value, and feedback primitives
- execute `g01.010` for overlays, menus, dialogs, and tabs with the stricter
  accessibility baseline now in place

## Next Task

Open `docs/roadmaps/g01/008-action-and-text-entry-primitives.md` and author
the interactive foundation control tranche with explicit GPUI accessibility
mapping for every control.
