# 023 Svelte Visual Hierarchy And Contrast Baseline

Status: active
Updated: 2026-03-12
Depends on: `020-docs-site-example-and-component-discoverability-rules.md`

## Purpose

Freeze the current Svelte-side visual hierarchy rules so preview polish and
later GPUI parity work target one deliberate presentation system instead of
reopening card, chip, border, and spacing decisions component by component.

## Surface Hierarchy Rule

Svelte implementations should present four distinct visual layers:

- `canvas`: the broadest app or docs background
- `panel`: section-level containers and shell regions
- `surface`: recessed controls, item rows, and inner utility chrome
- `elevated`: overlays, primary cards, and intentionally lifted callouts

These layers must remain visibly distinct in both dark and light themes.
`light` may not collapse canvas, panel, and elevated into one flat white field.

## Border Rule

Borders are no longer the default separation mechanism.

Use borders only when they add real semantic clarity, such as:

- explicit focus outlines
- outlined comparison variants
- table or token-inspector structures
- overlay frames that need edge definition against a noisy background

Most cards, panels, and action rows should separate primarily through tonal
contrast, spacing, and occasional shadow or inset edge treatment.

## Radius Rule

Use radius by role, not by habit:

- `radius.control` for buttons, segmented controls, inline chips, and compact
  workstation affordances
- `radius.surface` for cards, panels, and larger grouped regions
- `radius.pill` only for true micro-badges or status pills where a capsule shape
  is semantically helpful

Tabs, close buttons, palette controls, and workstation action affordances should
not default to pill geometry.

## Active State Rule

Active and selected states should primarily read through fill and emphasis, not
through louder borders on transparent backgrounds.

Preferred active-state signals are:

- accent-tinted fills
- slightly stronger tonal separation
- inset edge or highlight treatment where needed

Avoid stacking:

- strong border
- transparent fill
- strong shadow

on the same small control unless a specific accessibility case requires it.

## Light Theme Contrast Rule

Light mode must preserve visible hierarchy without becoming harsh or noisy.

Required light-theme posture:

- darker canvas than panel
- panel darker or more grounded than elevated card content
- recessed controls clearly distinguishable from their container
- secondary text and borders strong enough to remain legible against pale
  surfaces

Component-level light-theme overrides are acceptable when the shared token
hierarchy is not sufficient on its own.

## Spacing Rule

Section identity, metadata, and actions must not crowd each other.

At minimum:

- headers should maintain a distinct gap between eyebrow, title, and subtitle
- action clusters should keep horizontal breathing room between controls
- detail sections should rely on spacing before reintroducing dividers
- readonly rows should not appear as undifferentiated table stripes when a
  grouped card treatment communicates structure better

## Workstation Chrome Rule

Command palettes, discovery panels, dock controls, and project headers should
share one workstation-chrome language:

- compact control radii
- tonal item backgrounds instead of outline-heavy chips
- restrained metadata badges
- visible but not overdrawn shell framing

This chrome should feel denser and more intentional than the broader docs
surface without becoming visually noisy.

## Seed Evidence

- `packages/svelte/composites/src/Card.svelte`
- `packages/svelte/workstation/src/CommandPalette.svelte`
- `packages/svelte/workstation/src/ActionDiscoveryPanel.svelte`
- `packages/svelte/workstation/src/ProjectHeader.svelte`
- `packages/svelte/workstation/src/DockRegion.svelte`
- `packages/svelte/composites/src/SelectionSummary.svelte`
- `packages/svelte/composites/src/MediaPreview.svelte`
- `packages/svelte/preview/src/app.css`

## Next Task

Use this baseline while finishing the remaining workstation/docs polish and
before translating any visual hierarchy decisions into GPUI.
