# Theming model study

Status: research, 2026-11-25. Adopted by the operator as the direction for
Poodle's themes; see [vision](../../vision.md#direction). Not yet
implemented in the packages.

A theme is six settings: scheme, base colour, base blend, accent colour,
accent blend and contrast. One pure function derives every colour role and
holds text contrast. Every named theme is a preset of those settings.

## What is here

- [model.md](model.md): the model after six rounds, with the rules for
  layers, fills, text, the accent and the status colours.
- [fidelity.md](fidelity.md): how close the Nord and Solarized presets come
  to their source palettes.
- `index.html`: a self-contained page that runs the function. Open it
  in a browser. It has the six controls, a preset row, a live contrast
  readout and a preview drawn in the quiet direction.
- `tools/`: the scripts that tested the prototype's function: the blend
  invariants, the random contrast sweep, the accent rules and the preset
  solver. They read `index.html` beside them and run under Node, for
  example `node tools/invariant.cjs`. `model.md` names them with a `.js`
  ending; they are `.cjs` here because the repository's packages are ES
  modules. `unchanged.cjs` compares against the round-five copy and does
  not run here. The study's screenshot scripts are not kept.

`model.md` names result files and screenshot folders from the study's
scratch folder. They are not in the repository; the numbers it quotes from
them are in the text.

## Operator rulings on the model (2026-11-25)

- A named preset need not match its source palette exactly.
- At a blend of zero, a colour has no effect on any neutral.
- Even tint between light and dark is on.
- The accent fill stays as picked wherever its text can reach 7:1; only
  mid-tones move. A separate accent line colour serves thin marks.
- A fill gets a hairline edge only when it would vanish against its layer.
- Hover and pressed states darken the fill and keep text at 7:1 or more.
- The danger button follows the accent rule.
- The two blends are independent: each works alone, and at zero each
  removes its colour from the neutrals whatever the other is set to. The
  accent blend adds its tint to the base's, and the depth of the ground
  follows the larger blend.
- "No effect on a neutral" means no hue. A picked accent may still move a
  text lightness by one step to hold contrast.
- The danger button has its own edge role, shown only when the fill would
  vanish against a layer.

One ruling is not in `index.html`: the danger fill takes no pull
toward the ground's tint, so that red stays red on yellow, green and brown
grounds. The prototype still applies the half pull that the other status
colours keep. The port into the packages implements the ruling.

## What the study did not verify

The Rust port is not written. Only Chromium was checked. Hover was checked
on a states sheet, not live. The tolerances (how far a fill may move, when
the edge appears) are judgement calls.
