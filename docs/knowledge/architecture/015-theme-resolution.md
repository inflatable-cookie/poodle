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
