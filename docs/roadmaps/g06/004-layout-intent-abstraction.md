# g06.004 — Layout Intent Abstraction

Status: Completed
Updated: 2026-03-14

## Objective

Define renderer-agnostic layout intent types that both GPUI and Jetstream can
map losslessly to their native layout systems.

## Deliverables

New crate: `pug-layout` at `packages/contracts/layout/`

### Types

| Type | Purpose | GPUI Mapping | Jetstream Mapping |
|------|---------|-------------|-------------------|
| `LayoutDirection` | Row/Column flow | `FlexDirection` | `Direction` |
| `LayoutSizing` | Fit/Grow/Fixed/Constrained | `Length` + flex grow/shrink | `Sizing` |
| `LayoutEdges` | Top/Right/Bottom/Left insets | `Edges` | `Edges` |
| `LayoutSpacing` | Gap + padding + margin | Style gap/padding/margin | `UiStyle` fields |
| `MainAxisAlignment` | Start/Center/End/SpaceBetween | `JustifyContent` | `Justify` |
| `CrossAxisAlignment` | Start/Center/End/Stretch | `AlignItems` | `Align` |
| `LayoutAlignment` | Combined main+cross | Composite | Composite |
| `LayoutOverflow` | Visible/Hidden/Scroll | `Overflow` | ScrollView widget |
| `LayoutIntent` | Complete layout description | Maps to `Style` | Maps to `UiStyle` |

### Design Decisions

- All dimension values are resolved `f32` pixels (not token references) — the
  style descriptor layer (g06.006) handles token resolution
- `LayoutSizing::Constrained` supports optional min/max bounds for flexible
  layouts with limits
- Jetstream only supports vertical scroll, so `overflow_x: Scroll` will map
  to hidden in the Jetstream adapter (intentional delta)
- Builder pattern matches existing spec crate conventions

## Verification

- [x] `pug-layout` crate compiles
- [x] 4 tests pass covering defaults, builder, edges, and constrained sizing
- [x] All types map cleanly to both GPUI and Jetstream layout models
