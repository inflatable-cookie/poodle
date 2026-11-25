# Theme resolution

Status: preview

`poodle-core` and `poodle-headless` resolve themes from six settings. The
scheme is light or dark. Base and accent each take an sRGB colour or
`mono` and an independent blend from 0 to 1. Contrast (0.5 to 2) sets
layer spacing. Even tint is on by default and makes the same colour tint
both schemes more evenly.

The blends add tint symmetrically in OKLab. A blend of zero contributes no
hue, and the larger blend sets ground depth. The result has 44 8-bit RGBA
roles for layers, text, fills, selection, borders, accent, status, syntax,
scrim and shadow. The report records clamped inputs, fitted colours,
warnings and notes. The danger fill uses its own hue anchor, and the
danger foreground and tint share the ground pull used by other statuses.

The resolver fits primary text to 7:1 on canvas, surface and raised, and
primary and secondary text to 4.5:1 on all layers and composited tints.
Accent line reaches 3:1 on every layer. Text on accent fills reaches
4.5:1 in every state and aims for 7:1 within the fill's movement limit.
Checks use the emitted 8-bit colours. Near-white layer spacing can fall
below the study's 0.02 lightness target at lower contrast.

The [TypeScript resolver](../../../packages/core/src/theme-resolver.ts)
and [Rust mirror](../../../packages/contracts/headless/src/theme_resolver.rs)
are pure. [Presets](../../../packages/core/src/theme-presets.ts) are input
tuples. The current token build and theme CSS use their existing sources.

`effigy theme-resolver:vectors` generates the
[shared vectors](../../../packages/contracts/headless/vectors/theme-resolver.json)
from TypeScript. Both runtimes compare every role and the report against
those vectors. See the [theming model study](../research/theming-model/README.md)
for derivations and design trade-offs.

## Live previews (Svelte and React)

Choose **Live theme** from the preview's theme picker to edit the settings
or apply a resolver preset. The URL stores the settings, including even tint.
The style picker works independently. Choosing a built-in theme clears the
live settings and restores the generated CSS. The React preview mirrors the
Svelte one: the same core export, the same URL names and parsing (shared from
`packages/svelte/preview/src/live-theme.ts`), so one URL drives either preview.

[`previewThemeCssProperties`](../../../packages/core/src/theme-css.ts) maps
resolved roles to the current semantic CSS colour properties. The preview
sets them on the document root, so the shell, portals and the tab-list
colour aliases resolve against the same colours. The mapper outputs the
final layers and borders after the resolver applies contrast. The old CSS
contrast anchor applies only to built-in themes.

The bridge covers these differences in vocabulary:

| Current token | Live value |
| --- | --- |
| Background canvas, surface, panel, elevated | Resolver canvas, surface, raised, overlay |
| Background overlay | Scrim |
| Surface hover | Hover fill |
| Tertiary text | Secondary text, also used by muted icons |
| Primary icon | Primary text |
| Inverse text | Text on the accent fill |
| Default border | Mix of subtle and strong borders |
| Accent base, hover, focus ring | Accent fill, hover fill, line |
| Status colours | Status foregrounds |
| Accent selection variants | Selection tint adjusted to the existing subtle and strong ratios, composited over the surface for opaque variants. Accent text mixed with neutral borders for the two border variants. |

The current colour vocabulary has no separate tokens for disabled text,
the pressed accent fill, the accent edge, status backgrounds, solid danger
fill and its text and edge, or the shadow colour. Existing component CSS
still uses local mixes and literal effects, and shares colours between
some fills and thin marks. The preview bridge leaves those rules in place,
so the resolver's contrast guarantees apply to its own roles.
Primitive swatches such as the keyboard's two fixed neutrals also stay
fixed. CSS references with no generated declaration, such as
`--poodle-color-text-muted`, keep their existing fallback behaviour.
