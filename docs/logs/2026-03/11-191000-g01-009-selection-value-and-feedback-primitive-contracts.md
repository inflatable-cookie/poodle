# 2026-03-11 g01.009 Selection, Value, And Feedback Primitive Contracts

## Changed

- completed the `g01.009` foundation primitive tranche for selection, value,
  and feedback/status components
- added selection and value-control contracts for:
  - `Checkbox`
  - `RadioGroup`
  - `Switch`
  - `TriStateSwitch`
  - `SegmentedControl`
  - `Select`
  - `Slider`
  - `RangeSlider`
- added feedback and status contracts for:
  - `Progress`
  - `Skeleton`
  - `Badge`
  - `Pill`
  - `Callout`
  - `Banner`
  - `StatusIndicator`
- kept the contracts generic while reflecting real downstream patterns already
  visible in Aura and Spark:
  - shell toggles and segmented mode switches
  - selection and filter controls
  - compact badges/pills/callouts
  - progress and status messaging
- made GPUI accessibility expectations explicit across the whole tranche:
  - roving focus for exclusive-choice controls
  - role/name/state/value exposure for selection and range controls
  - focus restoration and trigger/listbox relationships for `Select`
  - non-color-only status semantics for indicators and banners
  - native announcement behavior for urgent inline messaging
- updated the foundation and contract indexes so the new family is visible from
  the top-level docs surfaces
- closed `g01.009` in the active roadmap

## Downstream Alignment

- Aura’s archived toggle, tri-state, segmented, badge, pill, and callout
  components confirmed the importance of keeping these contracts generic while
  documenting stronger semantics than the old ad hoc implementations carried
- Spark’s existing control posture reinforced that GPUI must explicitly model:
  - roving focus
  - exclusive-choice semantics
  - slider/range value exposure
  - accessible select relationships
  - status meaning beyond color

## Validation

- `bun packages/tokens/scripts/build-tokens.ts`
- `git diff --check`

## Remaining

- execute `g01.010` for tabs, menus, context menus, dialogs, drawers,
  popovers, and tooltips
- keep GPUI focus trapping, dismissal, and announcement semantics strict as the
  next overlay/navigation tranche becomes more complex

## Next Task

Open `docs/roadmaps/g01/010-overlay-navigation-and-interaction-primitives.md`
and author the next foundation tranche for overlays, tabs, menus, dialogs, and
interaction primitives.
