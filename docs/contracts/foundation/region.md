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
| `border`        | `0.125rem dashed var(--region-color, var(--pug-color-border-default))` |
| `border-radius` | `var(--pug-radius-surface)`                                |
| `padding`       | `var(--pug-space-inline-md)`                               |

### Label

| Property          | Token / Value                                             |
|-------------------|-----------------------------------------------------------|
| `color`           | `var(--region-color, var(--pug-color-text-tertiary))`     |
| `font-family`     | `var(--pug-typography-label-family)`                      |
| `font-size`       | `var(--pug-typography-label-size)`                        |
| `font-weight`     | `600`                                                     |
| `text-transform`  | `uppercase`                                               |
| `letter-spacing`  | `0.05em`                                                  |
| `user-select`     | `none`                                                    |

## Custom Color

When the `color` prop is set, it applies to both the dashed border and the label text via a `--region-color` CSS custom property. This allows different regions to be visually distinguished in layout diagrams.

## Accessibility

| Attribute | Value          |
|-----------|----------------|
| `role`    | `presentation` |

The Region is purely decorative and conveys no semantic meaning to assistive technology.
