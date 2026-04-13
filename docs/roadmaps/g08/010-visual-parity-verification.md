# g08.010 Visual Parity Verification And Delta Register

Status: complete
Owner: Poodle Core
Depends on: g08.009

## Verification Summary

### Code-Based Audit

All 78 GPUI component implementations were reviewed for contract compliance
across the quality fix batches (005, 006, 007):

**Token Resolution**: Every component resolves dimensions, colors, radii, and
spacing from semantic tokens. No hardcoded pixel values remain in rendering code
(switch track/thumb dimensions are contract-specified constants, not token
violations).

**Color Mixing**: All hover/active states use `color_mix()` helper (sRGB linear
interpolation) matching CSS `color-mix(in srgb, ...)`. No opacity-based
hover workarounds remain.

**Icons**: All icon slots use `PoodleIcon` with SVG rendering. Replaced: clock emoji
(time_field), star text (rating), text-based icons in various components.

**Disabled States**: All components use `resolve_opacity(theme,
"semantic.state.opacity.disabled")`. No hardcoded `0.48` opacity values remain.

**Broken Components Fixed**:
- `color_picker`: Swatch colors now render as backgrounds (was discarding loop
  variable `_color`). Added hex color parsing.
- `range_slider`: Filled range segment and dual thumbs now render at correct
  normalized positions (was discarding `_norm_low`/`_norm_high`).

### Delta Register

Created `docs/roadmaps/g08/delta-register.md` documenting (historical; **superseded by**
`docs/roadmaps/g10/012-gpui-runtime-truth-and-deferred-work-closure.md`):
- 8 cross-cutting platform deltas (focus rings, ARIA, fonts, shadows, etc.)
- 6 component-specific deltas (slider interaction, text editing, etc.)

All deltas are either GPUI platform limitations or cosmetic rendering differences.

## Acceptance Criteria

- [x] Every component audited for contract compliance
- [x] All bugs found during quality fix batches fixed (005–007)
- [x] Delta register documents all intentional differences
- [x] Contract compliance verified for all components (token resolution,
      color mixing, icon rendering, disabled states)
