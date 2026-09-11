---
name: Poodle
description: A calm, precise, adaptable design system for professional web and native tools.
colors:
  signal-blue: "#2d86f3"
  signal-blue-hover: "#57a7ff"
  success-lime: "#7fd24d"
  warning-amber: "#f0b24d"
  danger-coral: "#e06a5f"
  glacial-canvas: "#e7eef5"
  glacial-surface: "#dbe5ef"
  glacial-panel: "#f7fafd"
  glacial-elevated: "#fcfdff"
  ink-primary: "#131a22"
  ink-secondary: "#314255"
  ink-muted: "#75869b"
  frost-border: "#c9d4e0"
  eclipse-canvas: "#0e1012"
  eclipse-surface: "#15181b"
  eclipse-panel: "#1b1f23"
  eclipse-elevated: "#20252a"
  eclipse-text: "#eef2f6"
  eclipse-muted: "#99a4b0"
  eclipse-accent: "#f0b24d"
typography:
  heading:
    fontFamily: "Inter, SF Pro Display, system-ui, sans-serif"
    fontSize: "1rem"
    fontWeight: 600
    lineHeight: "1.5rem"
  body:
    fontFamily: "Inter, SF Pro Display, system-ui, sans-serif"
    fontSize: "0.875rem"
    fontWeight: 400
    lineHeight: "1.25rem"
  label:
    fontFamily: "Inter, SF Pro Display, system-ui, sans-serif"
    fontSize: "0.8125rem"
    fontWeight: 500
    lineHeight: "1rem"
  caption:
    fontFamily: "Inter, SF Pro Display, system-ui, sans-serif"
    fontSize: "0.6875rem"
    fontWeight: 500
    lineHeight: "1rem"
  code:
    fontFamily: "IBM Plex Mono, SFMono-Regular, monospace"
    fontSize: "0.8125rem"
    fontWeight: 400
    lineHeight: "1rem"
rounded:
  none: "0rem"
  small: "0.1875rem"
  control: "0.375rem"
  surface: "0.625rem"
  pill: "999rem"
spacing:
  "0": "0rem"
  "1": "0.25rem"
  "2": "0.5rem"
  "3": "0.75rem"
  "4": "1rem"
  "5": "1.25rem"
  "6": "1.5rem"
  "8": "2rem"
  "10": "2.5rem"
  "12": "3rem"
components:
  button-primary:
    backgroundColor: "{colors.signal-blue}"
    textColor: "{colors.glacial-elevated}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "0 0.75rem"
    height: "2.25rem"
  button-secondary:
    backgroundColor: "{colors.glacial-surface}"
    textColor: "{colors.ink-primary}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "0 0.75rem"
    height: "2.25rem"
  text-input:
    backgroundColor: "{colors.glacial-surface}"
    textColor: "{colors.ink-primary}"
    typography: "{typography.body}"
    rounded: "{rounded.control}"
    padding: "0.5rem 0.75rem"
    height: "2.25rem"
  card:
    backgroundColor: "{colors.glacial-elevated}"
    textColor: "{colors.ink-primary}"
    rounded: "{rounded.surface}"
    padding: "1rem"
  pill:
    backgroundColor: "{colors.glacial-surface}"
    textColor: "{colors.ink-secondary}"
    typography: "{typography.caption}"
    rounded: "{rounded.pill}"
    padding: "0.1875rem 0.625rem"
---

# Design System: Poodle

## Overview

**Creative North Star: "The Precision Workbench"**

Poodle is calm, precise, and adaptable. It should feel like a well-calibrated
professional workspace: dense enough for real work, quiet enough for sustained
attention, and exact enough that state changes never depend on decoration or
guesswork. Its visual identity comes from disciplined semantic roles rather
than one fixed theme.

The system is compact, tactile, and restrained. Hairline borders, small radii,
tonal layers, and subtle inset detail give controls physical definition without
turning the interface ornamental. Accent color is a signal for action, focus,
selection, or status. It is not ambient decoration.

Poodle supports a broad theme family, but each theme must preserve the same
hierarchy and interaction truth. Avoid AI slop and unnecessary noise: no
generic oversized cards, arbitrary gradients, decorative pill fields, filler
copy, redundant labels, or motion without state meaning.

**Key Characteristics:**

- Compact workstation density with independent density and control-size axes.
- Semantic, themeable color roles rather than component-local palettes.
- Tactile controls built from borders, tonal fill, and restrained inset depth.
- Clear keyboard focus, selection, validation, and disabled states.
- Equivalent visual intent across Svelte, React, GPUI, and admitted runtimes.

## Colors

Glacial neutrals and signal blue form the light semantic baseline; Eclipse uses
near-black structural layers and warm amber while preserving the same roles.
Other themes may change hue and temperature, never the hierarchy.

### Primary

- **Signal Blue:** the base interactive accent for action, selection, focus,
  and informational status.
- **Signal Blue Hover:** a brighter response color for hover and focus-ring
  visibility.

### Secondary

- **Success Lime:** positive state and completion feedback.
- **Warning Amber:** caution, pending attention, and Eclipse's warm accent.
- **Danger Coral:** destructive action, invalid state, and failure feedback.

### Neutral

- **Glacial Canvas:** the cool pale application ground.
- **Glacial Surface:** recessed control and secondary surface fill.
- **Glacial Panel:** card and panel layer.
- **Glacial Elevated:** floating or highest light surface.
- **Primary Ink:** high-emphasis content and control labels.
- **Secondary Ink:** supporting text and inactive controls.
- **Muted Ink:** tertiary metadata and low-emphasis content.
- **Frost Border:** low-contrast structural separation.
- **Eclipse Canvas, Surface, Panel, and Elevated:** four near-black layers that
  preserve the same elevation order in dark presentation.

**The Semantic Color Rule.** Components consume background, text, border,
accent, and status roles. They do not hard-code a theme's hue.

**The Signal, Not Decoration Rule.** Accent color marks action, state, or
orientation. If a colored element communicates none of those, remove the color.

## Typography

**Display Font:** Inter, with SF Pro Display and system sans-serif fallbacks
**Body Font:** Inter, with SF Pro Display and system sans-serif fallbacks
**Label/Mono Font:** IBM Plex Mono, with SFMono-Regular and monospace fallbacks

**Character:** The sans scale is neutral, compact, and legible at workstation
density. Monospace is reserved for code, identifiers, counters, and technical
values rather than used as generic decoration.

### Hierarchy

- **Heading** (600, 1rem, 1.5rem): section and component titles.
- **Body** (400, 0.875rem, 1.25rem): primary interface and document-supporting
  copy.
- **Label** (500, 0.8125rem, 1rem): controls, fields, navigation, and compact
  metadata.
- **Caption** (500, 0.6875rem, 1rem): tertiary explanation and dense status
  information.
- **Code** (400, 0.8125rem, 1rem): source text, identifiers, and machine values.

**The Role Before Size Rule.** Choose the semantic type role first. Size and
density variants may scale it, but one-off typography must not create a second
hierarchy.

**The Mono Has Meaning Rule.** Monospace communicates code or machine data; it
is never a shortcut for making ordinary UI feel technical.

## Layout

Poodle uses a 0.25rem base rhythm with reusable 0.5rem, 0.75rem, 1rem, 1.25rem,
1.5rem, 2rem, 2.5rem, and 3rem steps. Standard panels use 1rem inline and
0.75rem block padding. Controls use 0.75rem inline and 0.5rem block spacing.

Density and control size are independent. Compact, default, and comfortable
density change breathing room; `xs` through `xl` control sizes change physical
footprint and type scale. Components must remain usable when either axis changes
without assuming that larger controls imply looser layout.

Professional-tool shells favor grids, split regions, scroll-contained panels,
and aligned control rows. Responsive behavior reflows regions and preserves
task order; it does not simply scale a desktop canvas down.

**The Two-Axis Rule.** Density controls space; control size controls footprint.
Never collapse them into one implicit size choice.

**The Bounded Work Surface Rule.** Editors, lists, panels, and overlays own
their scroll and minimum-size behavior so one component cannot expand or clip
the surrounding workstation.

## Elevation & Depth

Poodle uses layered, structural depth. Tonal background changes and hairline
borders establish most hierarchy. A small surface shadow supports raised
content; overlays and dialogs receive progressively wider ambient shadows.
Buttons may use a restrained inset highlight and primary actions a compact
ambient shadow, but ordinary surfaces remain quiet.

### Shadow Vocabulary

- **Surface:** a low 0.25rem/0.75rem ambient shadow for raised cards and panels.
- **Overlay:** a medium 0.75rem/2rem shadow for menus, popovers, and floating
  utility surfaces.
- **Dialog:** a broad 1.5rem/3.75rem shadow for modal separation over a scrim.
- **Inset Detail:** a one-pixel highlight or border mix that gives compact
  controls tactile definition without simulating glossy material.

**The Structural Elevation Rule.** Shadows explain stacking or response. Never
add a shadow solely to make a flat layout look more designed.

## Shapes

The form language is gently squared: 0.375rem control corners and 0.625rem
surface corners, with 0.1875rem available for compact internal geometry. Pills
are reserved for badges, tags, binary capsules, and intrinsically rounded
controls. Hairline borders separate adjacent layers; focus uses a distinct
0.125rem ring with offset where the component allows it.

**The Rounded Square Rule.** Controls and cards use compact radii that preserve
their edges. Do not turn every button, tab, input, or container into a pill.

**The Shape Communicates Rule.** Full rounding identifies a pill-like semantic
or continuous track. It is not the default expression of friendliness.

## Components

Components feel compact, tactile, and restrained. Their states are legible from
fill, border, text, and focus treatment before animation or ornament enters.

### Buttons

- **Shape:** gently squared control radius with a one-pixel border.
- **Primary:** signal accent fill, inverse text, restrained inset highlight,
  and compact ambient shadow.
- **Secondary:** tonal surface fill, primary text, and a structural border.
- **Ghost:** transparent at rest; hover and pressed states introduce only the
  fill needed to make interaction legible.
- **Hover / Focus:** hover shifts fill and border; keyboard focus uses a clear
  two-pixel accent ring with offset; active state moves by half a pixel.

### Pills

- **Style:** compact fully rounded status or metadata capsules with tinted
  semantic fill, border, and strong small label text.
- **State:** status tones remain readable without relying on hue alone; subtle
  appearance reduces tint rather than removing the semantic role.

### Cards / Containers

- **Corner Style:** surface radius, with clipped internal media using a smaller
  nested radius.
- **Background:** elevated and panel roles are mixed to preserve theme depth.
- **Shadow Strategy:** base cards use border and inset definition; elevated
  cards use structural shadow; selected cards use the accent boundary.
- **Internal Padding:** one rem at default density, with compact variants
  reducing padding and gap together.

### Inputs / Fields

- **Style:** surface fill, one-pixel default border, body typography, and a
  control radius.
- **Focus:** accent border plus a translucent two-pixel focus shadow.
- **Error / Disabled:** semantic validation borders; disabled surfaces retain
  structure at reduced opacity rather than disappearing.

### Navigation

- **Style:** small medium-weight labels, quiet transparent items, and compact
  group spacing.
- **Active State:** a narrow accent edge, subtle accent-tinted fill, and stronger
  label weight establish orientation without a large selected capsule.
- **Group Labels:** uppercase, tightly sized, widely tracked accents used only
  as section markers.

### Editors

- **Style:** bounded panel surfaces with code or document typography, internal
  scrolling, and theme-owned selection and diagnostic colors.
- **Focus:** keyboard entry is visible, but the ring yields once editing begins;
  pointer entry does not add persistent chrome.
- **Engine Boundary:** CodeMirror, TipTap, and ProseMirror remain invisible
  implementation details behind Poodle controls and tokens.

## Do's and Don'ts

### Do:

- **Do** use semantic roles so every component remains coherent across the
  twelve admitted themes.
- **Do** preserve compact information density while maintaining target size,
  focus visibility, and readable state contrast.
- **Do** use tonal layering and hairline borders before reaching for shadow.
- **Do** keep Svelte, React, and native visual intent aligned through tokens and
  documented component contracts.
- **Do** let density, control size, theme, and motion policy remain independent.

### Don't:

- **Don't** add AI-slop composition: oversized generic cards, arbitrary
  gradients, excessive pills, fake metrics, filler labels, or empty decorative
  space.
- **Don't** add visual noise that does not clarify hierarchy, state, navigation,
  or interaction.
- **Don't** hard-code one theme's colors inside a reusable component.
- **Don't** use monospace, uppercase tracking, shadows, or motion as decoration
  without a semantic reason.
- **Don't** hide unsupported runtime behavior behind a visual fallback that
  implies parity.
