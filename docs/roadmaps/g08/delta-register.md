# GPUI Delta Register

This document records all known intentional differences between the GPUI
implementation and the Svelte reference implementation. Each delta is either
a platform limitation or an intentional design choice.

## Cross-Cutting Platform Deltas

### D-001: Focus Rings Not Rendered
- **Affects**: All interactive components (button, icon_button, checkbox, switch,
  select, tabs, text_input, text_area, number_entry, radio_group, slider,
  segmented_control, pin_input, toggle, split_button, etc.)
- **What differs**: Svelte shows `outline` on `:focus-visible`; GPUI shows nothing
- **Why**: GPUI fluent builder API does not expose outline or focus-visible styling.
  The `GpuiStyle` struct has `focus_ring_color`/`focus_ring_width` fields but they
  are only accessible through the adapter's render pipeline, not the component
  builder API.
- **Severity**: Functional (accessibility regression, keyboard navigation has no
  visual indicator)
- **Resolution**: Blocked on GPUI API extension

### D-002: ARIA Attributes Not Applied
- **Affects**: All interactive components
- **What differs**: Svelte applies `role`, `aria-label`, `aria-checked`,
  `aria-expanded`, `aria-disabled`, `aria-busy` per contract; GPUI applies none
- **Why**: GPUI element builders have no ARIA attribute methods
- **Severity**: Functional (screen reader support missing)
- **Resolution**: Blocked on GPUI API extension

### D-003: Font Rendering
- **Affects**: All text content
- **What differs**: Subpixel antialiasing, hinting, and kerning differ between
  browser (Svelte) and native (GPUI) text rendering
- **Why**: Platform difference — browser uses CSS font stack; GPUI uses native
  font rendering
- **Severity**: Cosmetic

### D-004: Box Shadow
- **Affects**: button (primary variant), elevated surfaces
- **What differs**: Svelte uses CSS `box-shadow` with inset highlight + outer shadow;
  GPUI uses `.shadow_sm()` / `.shadow_md()` which may differ in offset/blur/color
- **Why**: GPUI shadow system is fixed presets, not customizable like CSS box-shadow
- **Severity**: Cosmetic

### D-005: Letter Spacing
- **Affects**: button labels, potentially other text
- **What differs**: Contract specifies `letter-spacing: 0.01em`; GPUI does not
  support letter-spacing on text elements
- **Why**: GPUI text API does not expose letter-spacing
- **Severity**: Cosmetic

### D-006: SVG Icon Rendering
- **Affects**: All icon instances (button icons, icon_button, rating stars, etc.)
- **What differs**: Svelte renders SVG inline via `<svg>` elements; GPUI renders
  via luminance-based alpha masking of SVG assets
- **Why**: GPUI uses `svg().path()` with text_color-based tinting, which can
  produce slightly different visual results from inline SVG
- **Severity**: Cosmetic

### D-007: CSS Animations
- **Affects**: Loading spinners, transitions
- **What differs**: Svelte uses CSS `@keyframes` for spinner rotation and
  transitions; GPUI loading spinners use a static icon (no rotation animation)
- **Why**: GPUI does not support CSS-style keyframe animations on elements.
  Spinner implementation uses a Lucide "loader" icon as a static placeholder.
- **Severity**: Cosmetic (loading state is indicated by icon presence, not motion)

### D-008: Color-Mix Precision
- **Affects**: Hover, active, checked states on all interactive components
- **What differs**: Svelte uses CSS `color-mix(in srgb, ...)`; GPUI uses manual
  linear interpolation in sRGB space via `color_mix()` helper
- **Why**: GPUI converts Hsla → Rgba → interpolate → Rgba → Hsla, which may
  introduce minor floating-point rounding differences vs browser color-mix
- **Severity**: Cosmetic (sub-perceptual)

## Component-Specific Deltas

### D-100: Button Primary Border Darkening
- **Component**: button (primary variant)
- **What differs**: Contract specifies `color-mix(accent-base 84%, black)` for
  primary border. GPUI implements via `color_mix_black()` helper.
- **Severity**: Cosmetic (minor shade difference possible)
- **Status**: Implemented — may have minor precision delta

### D-101: Slider / Range Slider Thumb Interaction
- **Component**: slider, range_slider
- **What differs**: Svelte thumbs are draggable via native `<input type="range">`;
  GPUI thumbs are positioned statically based on spec value
- **Why**: GPUI does not have a native range input element. Drag interaction
  would require custom gesture handling.
- **Severity**: Functional (display-only, no drag interaction)

### D-102: Text Input / Text Area Native Editing
- **Component**: text_input, text_area
- **What differs**: Svelte uses native `<input>` / `<textarea>` elements with
  full cursor, selection, clipboard support; GPUI renders static display of
  current value
- **Why**: GPUI components render spec state as display elements. Full text
  editing requires GPUI's native text input integration.
- **Severity**: Functional (display-only in specimen context)

### D-103: Select Dropdown
- **Component**: select
- **What differs**: Svelte shows a real dropdown overlay with option list; GPUI
  renders options inline below the trigger when open
- **Why**: GPUI does not have a native popover/dropdown overlay system accessible
  from the component builder. Inline rendering is a workaround.
- **Severity**: Cosmetic / functional (positioning differs from Svelte)

### D-104: Drawer Sizing
- **Component**: drawer
- **What differs**: Svelte drawer may have variable width based on content/CSS;
  GPUI drawer uses `min_w(200px)` with flex layout
- **Severity**: Cosmetic

### D-105: Tri-State Switch Track Colors
- **Component**: tri_state_switch
- **What differs**: Mixed state uses a lighter accent blend (12%) vs checked (24%).
  Contract may not specify exact mixed state color.
- **Severity**: Cosmetic
