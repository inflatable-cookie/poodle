# g15.038 — SegmentedControl Native Option Parity

Status: **ready** — stop condition returned by `g15.028`; clean pre-1.0 Rust
migration approved by the operator on 2026-08-20
Depends on: `g15.028` (gap discovery and audit disposition)
Blocks: `g15.029`–`g15.033` orchestration, `g15.012`, `g15.013`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/segmented-control.md`,
`specimen-catalogue-audit.md`, `release-gap-register.md`

## Outcome

Close the active-runtime SegmentedControl option-shape gap found by the first
screen-clear review. Rust gains a dedicated `SegmentedControlOption` matching
the component contract; shared rendering and GPUI teach labelled-icon and
icon-only options with the same accessible-name, tooltip, and geometry rules as
the web reference.

This is a clean pre-1.0 migration. Replace SegmentedControl's use of the broad
`ChoiceOption` type and migrate every in-repo caller. Do not add a type alias,
`From<ChoiceOption>` compatibility conversion, overloaded constructor, silent
fallback, or deprecated twin.

## Scope

- Add and publicly export a dedicated Rust `SegmentedControlOption` carrying:
  `value`, `label`, optional icon name, `icon_only`, disabled state, optional
  `aria_label`, and optional `title`.
- Change `SegmentedControlSpec.options` and its constructor to use the dedicated
  type; migrate every in-repo constructor and direct struct literal.
- Keep `ChoiceOption` unchanged for Select, RadioGroup, CardRadioGroup, and
  other choice families.
- Render optional icons before visible labels. Hide label text only when
  `icon_only=true` and an icon exists; without an icon, preserve the label.
- Preserve the required option label as the accessible-name fallback. An
  explicit `aria_label` wins.
- Carry an explicit title as the tooltip; for icon-only options without one,
  use the required label. Add the smallest reusable node/backend tooltip
  declaration needed to project this in GPUI if the current vocabulary has no
  equivalent. Do not build a SegmentedControl-only overlay system.
- Match the contract's supporting-visual icon sizing, inline icon/text gap, and
  compact square geometry for `equal_width=false` icon-only options.
- Add the contract's two-option icon-only example to the GPUI specimen and keep
  its selection live.
- Close the audit blocker and release-gap row only after the implementation and
  specimen evidence pass.

## Acceptance

- [ ] Rust's public option shape matches every portable
      `SegmentedControlOption` field in the contract.
- [ ] No SegmentedControl call site still passes `ChoiceOption`; other choice
      component APIs remain unchanged.
- [ ] Labelled icons and icon-only options render through `poodle-render` with
      correct ordering, label fallback, icon size, gap, and compact geometry.
- [ ] GPUI projects radiogroup/option semantics, selected and disabled state,
      accessible names, activation, focus behavior, and the icon-only tooltip.
- [ ] The GPUI specimen teaches the same icon-only example as Svelte and React.
- [ ] Focused evidence fails if the icon disappears, icon-only exposes visible
      text, accessible fallback is lost, square geometry regresses, or the
      option handler stops firing.
- [ ] `specimen-catalogue-audit.md` returns SegmentedControl to GPUI A / `keep`
      and reconciles the totals; `release-gap-register.md` records the gap
      closed by this card.
- [ ] The batch log names the breaking Rust API migration and the operator's
      approval. No compatibility surface remains.

## Stop Conditions

- The clean type split requires a compatibility shim or dual public API.
- Tooltip parity requires a component-specific overlay or a new general overlay
  architecture rather than one bounded reusable node declaration.
- Icon support expands into Select, RadioGroup, or another component family.
- A web contract or observable Svelte/React behavior must change.
- Validation exposes a broader icon-provider or backend capability gap than the
  existing named-icon node path can satisfy.

## Writable Scope

- SegmentedControl Rust option/spec declarations, public exports, renderer,
  focused tests, in-repo callers, and GPUI specimen
- the smallest reusable `poodle-node` / GPUI backend tooltip field and focused
  projection evidence, only if required by the contract
- `docs/contracts/components/segmented-control.md` to remove the provisional
  native icon delta after closure
- `specimen-catalogue-audit.md`, `release-gap-register.md`, one August batch log
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- focused `poodle-specs`, `poodle-render`, and GPUI backend/specimen tests
- `effigy ci:rust`
- `effigy check:gpui`
- `effigy regressions:native`
- `effigy probe:gpui-specimens`
- `effigy docs:check`
- `effigy qa`
- `git diff --check origin/main...HEAD`

Headless only. Never run a `*-windowed`, `test:native-visual`, Jetstream, or
release selector.
