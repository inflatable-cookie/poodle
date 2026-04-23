# g04.008 Code Display And Color Selection

Status: planned
Owner: Poodle Core
Updated: 2026-03-14
Depends on: g04.001
Primary repos: `poodle`

## Goals

- [ ] implement Code as a primitive for syntax-highlighted code display
- [ ] implement ColorPicker as a primitive for color selection with swatches,
  input, and format support

## Execution Checklist

- [ ] write contract for Code: source text, language, line numbers, line
  highlighting, copy button, max-height with scroll, inline vs block mode
- [ ] evaluate syntax highlighting approach (static CSS classes vs runtime
  highlighter)
- [ ] implement Code primitive in `@poodle/svelte`
- [ ] write contract for ColorPicker: hex/rgb/hsl input, swatch palette,
  eyedropper (where supported), alpha channel, format toggle
- [ ] implement ColorPicker primitive in `@poodle/svelte`
- [ ] create specimens for Code and ColorPicker
- [ ] register in component-registry.ts and specimen registry

## Acceptance Criteria

- [ ] Code renders syntax-highlighted source with language detection
- [ ] Code supports optional line numbers and line highlighting
- [ ] Code includes a copy-to-clipboard button
- [ ] Code handles overflow with horizontal scroll or wrapping
- [ ] ColorPicker renders a color area, hue slider, and text input
- [ ] ColorPicker supports swatch palette selection
- [ ] ColorPicker outputs color values in hex, rgb, and hsl formats
- [ ] both components pass build and render in the preview catalogue

## Next Task

Open `g04.009` and implement navigation card and list card patterns.
