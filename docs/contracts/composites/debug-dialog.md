# DebugDialog

Status: active
Updated: 2026-04-09

## 1. Purpose

- Component name: `DebugDialog`
- Layer: `composites`
- Summary: a developer-facing debug overlay that renders arbitrary data as
  formatted JSON inside a dialog, triggered by a button that only appears when
  the value is non-null
- In scope: trigger button, dialog with syntax-highlighted JSON code block,
  configurable trigger label/variant/size, configurable close button
- Out of scope: data mutation, filtering, or editing; production user-facing
  data display

## Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `unknown \| null` | `null` | no | Data to display; component hidden when null/undefined |
| `title` | `string` | `"Debug data"` | no | Dialog title |
| `triggerLabel` | `string` | `"View debug data"` | no | Trigger button label |
| `maxHeight` | `string` | `"min(60vh, 32rem)"` | no | Max height for the code block |
| `triggerVariant` | `ButtonVariant` | `"ghost"` | no | Visual variant for trigger button |
| `triggerSize` | `ControlSize \| null` | `"sm"` | no | Size of trigger button |
| `showCloseButton` | `boolean` | `true` | no | Whether dialog shows a close button |
| `closeLabel` | `string` | `"Close debug dialog"` | no | Accessible label for close button |

## Composition

- Composes: `Button`, `Code`, `Dialog` from `@poodle/svelte-primitives`
- Trigger button only renders when `value` is non-null
- Value is serialized via `JSON.stringify` with 2-space indentation

## 2. Accessibility

- trigger button inherits standard `Button` accessibility
- dialog inherits standard `Dialog` accessibility including focus trap and
  `aria-modal`
- close button label configurable via `closeLabel` prop
- code block uses `Code` primitive which provides appropriate code semantics
