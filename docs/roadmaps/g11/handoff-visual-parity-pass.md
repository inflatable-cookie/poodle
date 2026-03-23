# GPUI Visual Parity Pass — Handover (Thread 3)

## Context

You are continuing a systematic visual quality pass of the Poodle design system's GPUI preview app. The goal is to make the GPUI preview **pixel-equivalent to the Svelte preview** — matching visual appearance, interactivity, and state management across all specimen pages.

This is the third thread working on g11 GPUI Contract Compliance. The first thread did the gap audit and began component fixes. The second thread (documented in `handoff-gpui-compliance.md`) continued component fixes. This thread completed specimen structural alignment and verified component rendering against Svelte screenshots.

## Project Structure

- **Repo root**: `/Users/betterthanclay/Dev/projects/poodle`
- **GPUI preview app**: `packages/gpui/preview/` — run with `cargo run` from that directory
- **Svelte preview app**: `packages/svelte/preview/` — run with `npm run dev` (serves on localhost, port varies)
- **GPUI components**: `packages/gpui/components/src/primitives/` and `.../composites/`
- **Svelte components**: `packages/svelte/primitives/src/` and `packages/svelte/composites/src/`
- **GPUI specimens**: `packages/gpui/preview/src/specimens/`
- **Svelte specimens**: `packages/svelte/preview/src/specimens/`
- **Shared Rust specs**: `packages/contracts/primitives/src/` and `packages/contracts/composites/src/`
- **Contracts**: `docs/contracts/foundation/*.md` and `docs/contracts/composites/*.md`
- **Gap report**: `docs/roadmaps/g11/gap-report.md` (777 lines, all 118 components audited)
- **Compile**: `cd packages/gpui/preview && cargo check`

## Architecture Quick Reference

- Components use a **spec + Deref pattern**: shared Rust specs (e.g., `ButtonSpec`) live in `poodle-primitives`/`poodle-composites`. GPUI components own a spec internally via `Deref<Target = SpecType>`.
- Two constructor patterns: `Component::new(theme)` (creates default spec) and `Component::from_spec(spec, theme)` (takes pre-built spec). All specimens use `from_spec`.
- Specimens have two signatures: `render(theme: &GpuiThemeProvider)` (static) or `render(state: &AppState, cx: &mut Context<PreviewRoot>)` (interactive with state).
- Interactive state uses `cx.listener(|this, val: &T, _w, cx| { ... })` — note `&T` reference parameter.
- `SpecimenState` tracks toggles (`is_on`/`toggle`), text values, counts, and selections.
- Section labels use the `Eyebrow` component: `Eyebrow::from_spec(EyebrowSpec::new().with_content("Label text"), theme)`.
- Each specimen group is wrapped: `div().flex().flex_col().gap(px(10.0)).child(Eyebrow...).child(...)`.
- Outer gap between groups: `gap(px(24.0))`.

## Key GPUI Patterns & Gotchas

- **No CSS text-transform** — uppercase done in Rust (`to_uppercase()` in Eyebrow component)
- **No CSS letter-spacing** — known limitation
- **No CSS gradients** — solid colors approximate where Svelte uses gradients
- **Focus ring**: Svelte uses `outline` with offset; GPUI approximates with `border_color` change on `.focus()` plus shadow ring (spread_radius 2px, 28% opacity)
- **Interactive divs need `.id("name")` to use `.on_click()`**
- **Callback types**: `&str` ✓, `&ClickEvent` ✓, `&bool` ✓, `&f64` ✓, `&usize` ✓
- **Color mixing**: `color_mix(a, b, ratio)` blends ratio% of `a` with (1-ratio)% of `b`
- **Theme helpers** in `theme_ext`: `resolve_color()`, `resolve_px()`, `resolve_radius()`, `resolve_opacity()`, `color_mix()`, `color_mix_black()`

## What's Been Done

### Structural (g11.001-002)
- 118/118 Svelte-GPUI component coverage
- Misplaced components reorganized (primitives vs composites)

### Gap Audit (g11 gap report)
- All 118 components audited against contracts
- Documented in `docs/roadmaps/g11/gap-report.md`

### Component Fixes (threads 1-2)
~30 components fixed against contracts. Key fixes documented in `handoff-gpui-compliance.md` and the previous `GPUI_VISUAL_PASS_HANDOVER.md`.

### Specimen Alignment (this thread)
- **All 87 GPUI specimen pages** updated to match Svelte structure:
  - Replaced all manual `section_label()` helpers with `Eyebrow` component
  - Mixed case labels matching Svelte (was ALL CAPS)
  - Consistent gaps: 24px between groups, 10px within groups
  - Each group wrapped in `div > Eyebrow + content` pattern
  - Content (labels, demo items, default states) matched to Svelte

### Cross-Cutting Component Fixes (verified in place)
- **Eyebrow**: `to_uppercase()`, font px(11.0), FontWeight::SEMIBOLD, line-height 1.5
- **Progress**: Track height px(8.0), indeterminate width 40%
- **Meter**: Fill uses `status.success` not `accent.base`, track bg with text-primary mix
- **StatusIndicator**: Dot px(9.0), gap px(7.0), border-radius px(999.0), shadow glow
- **Card**: Selected state uses border_1 + shadow ring (not border_2), footer 52% divider
- **Surface**: Uses `radius.surface` token, elevated shadow, non-elevated inset shadow ring
- **Separator**: Subtle tone uses 72% opacity approximation

### Svelte Specimen Cross-Reference (verified exact match)
14 Svelte specimens documented in detail and verified against GPUI:

| Component | Svelte Sections | GPUI Match |
|-----------|----------------|------------|
| Checkbox | Default, States | ✓ |
| Switch | Default, States | ✓ |
| RadioGroup | Vertical, Horizontal, Disabled | ✓ |
| Select | Default, Grouped, Disabled | ✓ |
| Slider | Default, With step, Disabled | ✓ |
| Progress | Determinate, Indeterminate, Custom max | ✓ |
| Separator | Horizontal, Vertical, Decorative | ✓ |
| Eyebrow | 3 demo items | ✓ |
| Pill | Tones, Sizes, Code font, Muted, Badge | ✓ |
| Callout | Tones, Message, Dismissible, Without title | ✓ |
| Dialog | Basic, Alert, No backdrop dismiss | ✓ |
| Drawer | Right edge, Left edge | ✓ |
| Tooltip | Default, Placements | ✓ |
| Field | Default+desc, Required, Error, Valid, Optional, Hint, Combined | ✓ |

Additional verified (via code review): Button, IconButton, SplitButton, SearchField, Accordion, Rating, Slider, Tabs, Collapsible.

## What Still Needs Work

### Pixel-Level Visual Tuning
The specimens are structurally correct but the *rendered output* of GPUI components may differ from Svelte at the pixel level. This requires running both apps side-by-side and fixing discrepancies. Areas to check:

1. **Color-mix precision** — GPUI approximates CSS `color-mix()` with linear interpolation. Some tints may be slightly off.
2. **Focus ring appearance** — Most interactive controls have `.focus()` border-color change but the visual weight may differ from Svelte's `outline` + `box-shadow`.
3. **Typography micro-adjustments** — Some components may still use `.text_sm()` instead of contract-specified sizes.
4. **Hover/active state colors** — Blend formulas may produce slightly different results.
5. **Shadow rendering** — GPUI box-shadow API differs from CSS; elevated surfaces and cards may look different.
6. **Border subtlety** — Color-mix opacity on borders may need tuning per-component.

### Known Component-Level Issues
From the handoff and gap report, these components have known rendering issues beyond pixel tuning:

- **TriStateSwitch**: Needs full architectural rewrite (sliding switch → 3-segment radiogroup)
- **Temporal pickers** (DateTimePicker, DateTimeRangePicker, ZonedDateTimePicker): Trigger-only — no overlay/calendar rendered when open
- **TimeZoneSelect**: Only 3 hardcoded timezone options
- **ColorPicker**: Simplified swatch grid instead of gradient pad + hue slider
- **FileUpload**: No file list rendering, no drag-active state
- **EditableLabel**: No mode switching (display vs input)
- **Code**: No inline mode, no toolbar, no copy button
- **TimeAgo**: No relative time computation — displays raw timestamp
- **ScrollShell**: Essentially a stub — no scrolling behavior
- **New composites** (17 total): All minimal stubs — AudioPlayer, BlockEditor, EmbedInput, EmbedPreview, FormDialog, FormLayout, LogList, MarkdownEditor, MediaPicker, PageLoading, ReorderableList, SlugField, VideoPlayer, etc.

### Structural Issues
- **Orphaned Rust specs** (no contract): badge.rs, banner.rs, autonomous_list.rs, form_shell.rs, inline_remediation.rs, remediation_banner.rs, state_tile.rs, validation_summary.rs
- **surface_elevation**: Contract exists, nothing implemented
- **call_out.rs** naming: should be `callout.rs` to match contract (functional, just naming)

## Workflow

1. Start the Svelte preview: `cd packages/svelte/preview && npm run dev`
2. Start the GPUI preview: `cd packages/gpui/preview && cargo run`
3. Navigate both apps to the same component
4. Compare visually — note differences in color, spacing, typography, shadows, borders
5. Fix the GPUI component (in `packages/gpui/components/src/primitives/` or `composites/`)
6. Fix the specimen if content/layout doesn't match (in `packages/gpui/preview/src/specimens/`)
7. `cargo check` after each fix
8. Commit per-batch with descriptive messages

## Reference Files

- Prior handoff: `docs/roadmaps/GPUI_VISUAL_PASS_HANDOVER.md` (thread 2 handoff)
- Component fixes handoff: `docs/roadmaps/g11/handoff-gpui-compliance.md` (thread 1 handoff)
- Gap report: `docs/roadmaps/g11/gap-report.md`
- Milestone files: `docs/roadmaps/g11/003-inputs-batch.md` through `013-generation-closeout.md`
