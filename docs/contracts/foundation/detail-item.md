# DetailItem

Status: contract
Updated: 2026-03-31

## 1. Purpose

- Component name: `DetailItem`
- Layer: `foundation`
- Summary: a lightweight label/value atom for use inside detail-section body
  grids where a full `DetailItem` would be too heavy
- In scope: label/value display, optional custom value content, optional full
  column span in multi-column detail grids
- Out of scope: section headers, row actions, inline editing, complex metadata
  composition

## 2. Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `label` | `string` | — | yes | visible detail label |
| `value` | `string \| number \| null` | `null` | no | simple text value fallback |
| `span` | `"full" \| "half" \| null` | `null` | no | `full` spans all columns in the parent grid |
| `children` | `Snippet \| undefined` | `undefined` | no | custom value content rendered instead of `value` |

## 3. Anatomy

```text
[Root]  <div class="detail-item">
  ├── [Label] <dt>
  └── [Value] <dd>
```

## 4. Behavior

- when `children` is present, it replaces the simple `value` text
- when `value` is `null`, the fallback display is `—`
- when `span="full"`, the root spans all columns in the parent grid

## 5. Accessibility

- renders semantic `<dt>` and `<dd>` pairs
- remains accessibility-neutral beyond host-provided surrounding section/group

