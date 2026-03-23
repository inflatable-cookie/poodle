# Region

A presentational placeholder block that designates an area where content could exist but currently doesn't. Uses a dashed border and centered label to communicate layout structure without actual content. Useful in documentation, wireframes, layout specimens, and empty states within shells.

## Anatomy

```
region (root)
└── label (text)
```

| Part    | Element | Notes                             |
|---------|---------|-----------------------------------|
| `root`  | `div`   | Dashed-border container           |
| `label` | `span`  | Centered uppercase label text     |

## Props

| Prop        | Type     | Default | Description                                           |
|-------------|----------|---------|-------------------------------------------------------|
| `label`     | `string` | `""`    | Text displayed centered in the region                 |
| `color`     | `string \| null` | `null` | Custom CSS color for border and label text   |
| `minHeight` | `string` | `"4rem"` | Minimum height of the region                        |

## Token Targets

### Root

| Property        | Token / Value                                              |
|-----------------|------------------------------------------------------------|
| `display`       | `flex`                                                     |
| `align-items`   | `center`                                                   |
| `justify-content` | `center`                                                 |
| `border`        | `0.125rem dashed var(--region-color, var(--flint-color-border-default))` |
| `border-radius` | `var(--flint-radius-surface)`                                |
| `padding`       | `var(--flint-space-inline-md)`                               |

### Label

| Property          | Token / Value                                             |
|-------------------|-----------------------------------------------------------|
| `color`           | `var(--region-color, var(--flint-color-text-tertiary))`     |
| `font-family`     | `var(--flint-typography-label-family)`                      |
| `font-size`       | `var(--flint-typography-label-size)`                        |
| `font-weight`     | `600`                                                     |
| `text-transform`  | `uppercase`                                               |
| `letter-spacing`  | `0.05em`                                                  |
| `user-select`     | `none`                                                    |

## Custom Color

When the `color` prop is set, it applies to both the dashed border and the label text via a `--region-color` CSS custom property. This allows different regions to be visually distinguished in layout diagrams.

## Specimen Definitions

### Group: Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `label="Content area"` | Single region with dashed border, default border color, centered uppercase "CONTENT AREA" label, default 4rem min-height |

### Group: Custom colors

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Header | `label="Header"`, `color="#5b9bd5"`, `minHeight="3rem"` | Blue dashed border and blue label text, 3rem min-height |
| Sidebar | `label="Sidebar"`, `color="#70ad47"`, `minHeight="6rem"` | Green dashed border and green label text, 6rem min-height |
| Main content | `label="Main content"`, `color="#ed7d31"`, `minHeight="8rem"` | Orange dashed border and orange label text, 8rem min-height |
| Footer | `label="Footer"`, `color="#a855f7"`, `minHeight="3rem"` | Purple dashed border and purple label text, 3rem min-height |

### Group: Layout composition

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Nav | `label="Nav"`, `color="#5b9bd5"`, `minHeight="100%"` | Blue region filling left column of a two-column grid layout |
| Toolbar | `label="Toolbar"`, `color="#70ad47"`, `minHeight="2.5rem"` | Green region at top of right column, 2.5rem tall |
| Content | `label="Content"`, `color="#ed7d31"`, `minHeight="10rem"` | Orange region filling remaining space in right column |

## Accessibility

| Attribute | Value          |
|-----------|----------------|
| `role`    | `presentation` |

The Region is purely decorative and conveys no semantic meaning to assistive technology.
