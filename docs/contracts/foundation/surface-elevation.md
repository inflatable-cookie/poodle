# Surface Elevation

Status: active
Updated: 2026-03-23

- Component name: `SurfaceElevation`
- Layer: `foundation`

## 1. Purpose

Surface elevation is the cross-cutting contract that keeps nested surfaces
visibly distinct across themes and renderers. It defines how surface creators
set the current surface value and how surface consumers derive contrast from
that value without hard-coding renderer-specific background tokens.

## 2. Anatomy

```text
surface creator
└── provides current surface color
    └── surface consumer
        └── derives contrast from current surface
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| creator | yes | Element or view that establishes a visual surface | surface fill |
| consumer | yes | Child surface that derives contrast from parent surface | contrast mix, border, shadow |

## 3. Props And Inputs

| Input | Type | Required | Notes |
|-------|------|----------|-------|
| `--flint-surface` | CSS custom property | yes for surface creators | Must match the creator's computed background value |
| surface context | renderer context value | yes for GPUI and Jetstream creators | Native equivalent of `--flint-surface` |
| mix ratio | tokenised constant | yes for consumers | Keeps contrast tiers consistent across renderers |

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| creator | element establishes a background | Surface value is propagated to descendants |
| consumer | nested inside a creator | Background derives from current surface plus contrast mix |
| nested creator | creator inside another creator | Inner surface replaces inherited surface for descendants |

## 5. Accessibility

### Semantics

- Role: not applicable
- Required attributes: none
- Optional attributes: none
- Labeling rules: not applicable

### Keyboard

Surface elevation does not define keyboard behavior directly. Interactive
components layered on top of it must preserve their own keyboard contracts.

### Focus And Announcement

- focus entry: not applicable
- focus exit: not applicable
- live-region or announcement behavior: none
- GPUI-native accessibility mapping notes: native renderers must preserve the
  same contrast intent even though they do not use CSS custom-property
  inheritance

## 6. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| creator | `--flint-surface` | Canonical current surface value for descendants |
| consumer | `--flint-color-text-primary` | Contrast-mix target across themes |
| consumer | surface mix ratios | Keeps strong, medium, and subtle elevation tiers aligned |
