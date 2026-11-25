# Theme model: scheme, base, accent, two blends, contrast

Proposal, round six, 2026-11-25. The prototype `index.html` runs the function described here, and
the scripts in `tools/` test that same code. Raw output is in `invariant-results.txt`,
`unchanged-results.txt`, `sweep-results.txt`, `sweep-contrast-results.txt`,
`test-engine-results.txt`, `accent-results.txt`, `slope-results.txt`, `capture-results.txt` and
`preset-pixel-diff.txt`. Captures are in `shots/`: full pages at the top level, the app alone in
`shots/app/`, the chooser readouts in `shots/readout/`, round five's preset pages in `shots/v5/`,
and contact sheets in `shots/sheets/`. Round five is unchanged in `../theming-prototype-v5/`.
`fidelity.md`, `solve-results.txt`, `rules-results.txt` and `balance-results.txt` are carried over:
at accent blend 0 nothing they measure has changed.

## What changed from round five, and why

**The report.** "Accent blend doesn't work unless some base blend is also mixed." Reproduced in
round five: at base blend 0, with a `mono` or Pink base, moving accent Blue or Yellow from 0 to 1
changed nothing. Every layer stayed `#000000 #0c0c0c #161616 #1f1f1f` in dark and
`#efefef #f7f7f7 #ffffff #ffffff` in light, ΔE 0.0 at every step.

**Cause, in two parts.**

1. Round three made the accent a second base under the same cap: accent blend moved the tint from
   the base's slope toward the accent's. Round four then multiplied that whole slope by
   `w(base blend)`. At base blend 0, `w` is 0, so the accent's share was multiplied by zero too.
2. Depth came from base blend alone. At base blend 0 the dark canvas is pure black, and the light
   raised and overlay layers are pure white, where no tint can show. Fixing only the multiply
   would tint the other layers at about half strength, beside a black canvas.

**The rule.** The two pairs are two tint sources, treated alike:

```
tint  = cap( w(baseBlend) × cap(base) + w(accentBlend) × cap(accent) )      w(b) = b(2 − b)
depth = max(baseBlend, accentBlend)
```

Each colour's slope is muted to the cap as before and weighted by its own blend. The two are
added, and the sum is muted to the same cap. Depth follows the larger blend, because a ground
needs distance from black or white to carry a tint, which is why base blend moved depth in round
four.

- **The pairs are symmetric.** Swap (base, base blend) with (accent, accent blend) and the neutrals
  are byte-identical. Only the accent roles differ, because they still come from the accent colour
  (`shots/sheets/swap.png`). *Opinion:* symmetry is right. "Independent" means each pair works on
  its own the same way. The alternative, where accent blend never moves depth, leaves the dark
  canvas black and the light cards white at base blend 0. Its tint is 45–75% as strong, and light
  Blue stops growing after 0.5 (table below).
- **What accent blend means now.** It adds the accent's tint; it no longer replaces the base's. In
  round five, accent blend 1 meant "the accent's tint instead of the base's". Now it means "the
  accent's full tint, added to the base's". For the accent's tint alone, set base blend to 0. A
  `mono` accent adds no tint, but an accent blend above base blend still sets depth, the mirror of
  a `mono` base.

**At base blend 0, accent blend 0.5** (canvas, surface, raised, overlay; even tint on), the owner
sees exactly what that colour gives as the base at 0.5:

| | Dark | Light |
|---|---|---|
| Blue | `#111d33 #17253f #1c2c4a #223455` | `#d9e7ff #e5eeff #f1f6ff #fdfeff` |
| Yellow | `#231d08 #2c250c #342c10 #3c3313` | `#efe6c4 #f4eed7 #f9f6e9 #fefefc` |
| Round five, any accent | `#000000 #0c0c0c #161616 #1f1f1f` | `#efefef #f7f7f7 #ffffff #ffffff` |
| If accent blend didn't move depth, Blue | `#000000 #060c19 #0c1629 #131f36` | `#e7efff #f3f7ff #ffffff #ffffff` |

That is navy or olive-brown in dark and pale blue or cream in light: an accent-toned theme
(`shots/sheets/owner-*.png`).

**Both blends above 0.** Same hues add up: Blue 0.5 with Blue 0.5 gives surface `#122447`
(chroma 0.070), against `#17253f` (0.052) for either alone, at the same depth. Neighbouring hues
land between: Blue with Violet gives `#1e2148`. Opposite hues cancel toward grey: Pink with Green
gives `#2f231a`, Amber with Indigo `#2b222d`. The report says "Base and accent hues cancel, so the
neutrals lose most of their tint." The note now fires when the combined tint is under half the
stronger one, which means hues about 150° or more apart. Orange and Blue are 140° apart in OKLab
and keep 68% of their tint (`#31202e`), so they no longer get the note they got in round five.
Stacking two colours can reach the full cap where neither can alone. Teal with Green at 0.5 in
light gives a mint canvas, `#a3f6dc`. *Opinion:* fine. That is what two greens at half strength
ask for, and it stays inside the cap (`shots/sheets/both-*.png`).

**The edge sees more of it.** Accent blend now tints the ground toward the accent's own hue, so a
muted fill can sit closer to the layers. At accent blend 1 the edge appears for Muted blue and
Muted indigo in 86% of dark combinations (14–15% at accent blend 0), for Indigo in 57% of dark
ones and for Yellow in 69% of light ones (neither ever gets one at accent blend 0). That is the edge rule working as designed.

**The danger button (operator ruling).** `status.danger.solid` now uses the accent fill's fit. It
stays as derived if its label reaches 7:1; otherwise it leaves the mid-tones by the nearer exit.
The derived red is a mid-tone, so it always takes the darker exit with white text: `#d33b3a` →
`#b1101d` on a neutral ground, 7.05:1 or more in all 20,000 test themes. With no duty to the layers,
it came within ΔE 10 of a layer in 58 of them (0.3%: strongly brown or red dark grounds at high
blend). So it also gets the accent's edge, as one new role, `status.danger.edge`, in the danger
text colour (4.5:1 on every layer), transparent otherwise. That makes **44 roles**
(`shots/danger-edge-dark.png`). Hover and pressed are computed by the same fit but not emitted,
as in round five.

**Cost.** On warm or green grounds the danger red turns brick: `#a23100` at Yellow accent blend 1,
`#a42e00` on Teal with Green. The cause is the existing half-tint pull, which turns the derived
red orange, and then 7:1 darkens it. Round five's lighter fill hid this.

**Unchanged at accent blend 0.** All 43 of round five's roles are byte-identical apart from the
danger pair, over 10,000 random inputs (1,674 at base blend 1, where the second cap could have
rounded). The presets all have accent blend 0, so every preset's surface and every other role is
unchanged, in plain and even tint. A pixel diff of every preset page against round five changes
only the danger button (2,334 to 2,376 pixels). There are also up to 8 pixels of caret or
anti-aliasing noise elsewhere, 5 levels at most; Graphite has none.

| Test | Cases | Result |
|---|---:|---|
| R1: base blend 0, accent colour at t gives the same neutrals as that colour as base at t | 10,000 | 0 failures |
| R1: accent blend 0 → 1 at base blend 0, in 0.01 steps | 1,200 colours × 101 | no drop over 0.6 ΔE (largest 0.46, rounding); 0.14 ΔE at 0.01; strength equal to the base's (ratio 1.000) |
| R2: base blend 0, accent blend above 0: the base changes no role | 10,000 | 0 failures |
| R2: accent blend 0, base blend above 0: the accent changes no neutral | 10,000 | 0 failures (text lightness, see below) |
| R3: swap the pairs | 10,000 | 0 failures |
| R4: one cap with both blends above 0 | 10,000 | slope never above the cap; largest layer chroma 0.118, same as round five |
| R4: no jump as a slider leaves 0 | 2,400 sweeps | largest change in any neutral 0.66 ΔE (disabled text, one 8-bit step) |
| R4: no dip while the hues are within 90° | 2,400 + 2,400 sweeps | the slope the rule asks for never falls (`tools/slope-monotone.js`). The visible tint falls over 0.6 ΔE in 45 sweeps (largest 2.44): 44 at the sRGB edge, one at exactly 0.60 |
| R5: depth is the larger blend | 10,000 | 0 failures |
| Round four: base blend 0 equals `mono`; mono base at accent blend 0 is grey | 10,000 each | 0 failures |
| Unchanged from round five at accent blend 0 (`tools/unchanged.js`) | 10,000 + 26 presets | 0 outside the danger pair |
| Random themes, contrast 1 | 6,000 | no failures |
| Random themes, contrast 0.9 to 2 | 4,000 | 2: the known light ΔL floor at contrast 0.90 and 0.91 (the new one fails in round five too at accent blend 0) |
| Garbage inputs | 5,000 | none threw, all 44 roles valid |
| Accent guarantees, plus danger (`tools/accent.js`) | 20,000 | 0 failures; fill shares as round five; edge on 13.7% / 5.2% (round five 11.7% / 4.3%) |

The R cases run 2,500 per scheme and tint mode, so 5,000 per scheme. "Neutral" means the
tint-carrying roles: layers, ink fills, neutral selection, both borders, disabled text, scrim and
shadow, plus the tint vector and depth. **Primary and secondary text** are fitted against every
tint, including the accent selection and the status tints, which follow the accent. So the accent
colour can shift their lightness by a step, but not their hue or chroma, even at accent blend 0
(83% of cases). That was already true in rounds four and five. It is a contrast guarantee, not a
tint, and the tests count it separately.

**Open for the owner (round six).**

1. **Symmetric depth.** Accent blend moves depth when it is the larger blend. The alternative keeps
   depth on base blend only. That would make a `mono` base plus accent blend a depth-and-tint pair
   and reach shallow tinted grounds (round five's question 2), but at base blend 0 the canvas
   stays black. I chose symmetric (opinion).
2. **Accent blend adds rather than replaces.** Confirm that losing "accent blend 1 removes the
   base's tint" is fine.
3. **Danger on warm grounds.** Accept the brick red, or derive the danger fill from the unpulled
   red, so it stays red on any ground. I'd drop the pull for the danger fill only (opinion).
4. **`status.danger.edge`**, the 44th role. Confirm.
5. **Text lightness and the accent** (above). If "no effect on any neutral" should include text
   lightness, fit text against a neutral selection tint.

Round five's questions 1 to 7 stand; question 8 is answered by the ruling.

**Dips.** The rule never asks for less tint as a slider rises, while the hues are within 90°. What
shows can still fall a little in two ways, both from earlier rounds. At the sRGB edge, a hue turning
toward green or blue can't hold the chroma at that lightness: Dark Green at 0.59 with Teal rising
drops 0.75 ΔE. At the cap, surface chroma can fall up to 6.6% as the hue turns, because a layer
sits at fixed luminance and its lightness depends on hue.

**Not verified.** Chromium only, headless. Strength and the look of the stacked and danger cases
are judged by eye on the captures. Byte-identity at accent blend 0 is tested, not proven (the
second cap could, in principle, round differently at base blend exactly 1; it never did in 1,674
cases). Rust is not written.

## Round five: what changed from round four, and why


**The report.** Light, `mono` base at blend 0, accent yellow `#ffd400`. The fill came out mustard
(`#b37f00`), with dark text at 5.1:1 that was hard to read. **Cause:** `fitFill` required the fill
to reach 3:1 against every layer. Yellow is 1.2 to 1.4:1 against light layers, so the fill was
darkened to OKLab L 0.63. That is a mid-tone, where dark text gets 5.1:1 and white 3.5:1. Dark
text on the picked yellow gets 13.1:1.

**The accent now does three jobs, fitted separately.** Everything else stays as round four had it:
the six inputs, `w = b(2 − b)`, the invariants, even tint (on by default), the presets and the
preview's look. No role outside the accent fill family changed (`tools/unchanged.js`: 10,000
random inputs, every shared role byte-identical).

| Role | Used for | Rule |
|---|---|---|
| `accent.fill`, `-hover`, `-active` | primary button, large fills, checked checkbox, switch on, radio on | As picked, unless its own label can't reach 7:1. No duty against the layers. |
| `accent.on` | text and marks on the fill | Dark ink (OKLab L 0.18 in the fill's hue) or white, whichever is stronger. Always at least 4.5:1, and 7:1 is the aim. |
| `accent.line` (new, replaces `focus.ring`) | focus ring, tab underline, selected-row bar, current nav icon, the fill's edge | 3:1 on every layer, fitted as before |
| `accent.text` | links, text in the accent colour | 4.5:1 on every layer (unchanged) |
| `accent.edge` (new) | hairline on filled controls | `accent.line` when the fill would vanish, otherwise transparent |

That makes 43 roles (round four had 42).

**The fill rule.**

1. If dark ink or white reaches 7:1 on the picked colour, the fill *is* the picked colour. That is
   64% of accents from the sweep's generator and 55% of uniform-RGB picks: every light colour
   (yellow, amber, pastels, white) and every dark one (navy, black).
2. Otherwise the colour is a mid-tone (luminance about 0.10 to 0.34), and it leaves the mid-tones
   by one of two exits: lighter with dark ink, or darker with white. It takes the exit nearer the
   picked colour in OKLab ΔE, and moves only until text reaches 7:1. Hue and chroma are held, so
   the distance counts only what sRGB forces: lost chroma and the yellow-to-amber drift. The muddy
   way (yellow to olive, orange to rust) costs more and loses. Orange, green, pink and Clay get
   lighter with dark text. Blue, indigo, violet and muted blue get darker with white.
3. **Reach: at most ΔE 12** (OKLab × 100). That is the largest move a mid-tone needs when sRGB keeps
   its chroma (a mid grey needs 11.7). If 7:1 needs more, the fill stops at ΔE 12 and the report
   says so ("Text on the accent fill reaches 6.91:1, short of 7:1 …"). That happens for about 0.5%
   of picks, all saturated reds, magentas and violets at the sRGB edge, and they reach 6.89 to
   6.99:1. 4.5:1 always holds.
4. It never moves *into* a mid-tone. A fill moves only when the picked colour can't carry 7:1, and
   then only toward an exit.
5. **Hover and active darken** (0.04, then up to 0.08 OKLab L), as buttons usually do, while text
   keeps min(rest, 7:1). Under dark text a light fill darkens only as far as 7:1 allows (yellow:
   13.1, 11.5, 10.0). A fill that can't darken by 0.04 (one fitted to exactly 7:1, or black)
   lightens instead. I read "keep or improve that contrast" as "stay at or above 7:1". The literal
   reading, never lower it, makes a light fill lighten: pressed yellow turned cream (`#fff2c5`),
   ΔE 6 from the layer (`shots/fill-states.png`). That reading is one line to restore.

**The edge.** It appears when the resting fill is within **ΔE 10 of any layer**, measured in OKLab
with flare lightness in place of L. Flare lightness is the lightness the layer ramp already uses,
and it keeps near-black differences honest: black on `#0c0c0c` is 2.4, where plain OKLab says
15.4. ΔE 10 is about four layer steps at contrast 1, and closer than that a fill reads as one more
layer. Yellow on `#f7f7f7` is 20 (its chroma separates it, though luminance contrast is only
1.34:1), so it gets no edge. The edge never appears for a vivid swatch, in any scheme, base, blend
or contrast. It does appear for white, pale yellow, pale blue and muted yellow on light, and for
black and navy on dark. Muted blue and muted indigo on dark get it too, in 14 to 15% of
combinations: those where the base is the same muted blue at a high blend.

**Small filled controls** (checked checkbox, switch on, radio) use the fill, a mark in the
on-colour (checkmark, thumb, dot) and the same conditional edge as the button. The state
indicator is the mark, and it gets the label's contrast: at least 4.5:1, and 7:1 in 99.5% of
picks. That is above the 3:1 WCAG asks of a state indicator. Using the line colour instead would
bring back the owner's mustard checkbox and switch. Off states are unchanged (a strong-border ring
and a neutral track), so on and off differ by fill, mark and, for the switch, thumb position. The
preview has no radio, so the radio rule is stated but not captured.

**Unchanged on purpose (superseded in round six):** `status.danger.solid` kept the old fit (4.5:1 text, 3:1 on the layers).

**Cost.** Mid-tone presets move further from their authored accent than in round four, because
the aim rose from 4.5:1 to 7:1. Accent fill against today's themes: Iceberg 0 → 6.8 (deeper blue),
Midnight 4.0, Rose 3.6, Solarized 4.9. Clay 9.5, Meadow 11.4 and Solarized Light 8.5 now carry
dark text on a lighter fill, where round four had white text at 4.6:1. The authored colour stays
exact in `accent.line` for every one of them.

| Test | Cases | Result |
|---|---:|---|
| Round four invariants (`tools/invariant.js`, role count 42 → 43) | 30,000 + growth | 0 failures |
| Random themes, contrast 1 (`tools/sweep.js`) | 6,000 | no failures |
| Random themes, contrast 0.9 to 2 | 4,000 | 1: the known light ΔL floor at contrast 0.91 (as round four) |
| Garbage inputs (`tools/test-engine.js`) | 5,000 | none threw, all 43 roles valid |
| Accent guarantees (`tools/accent.js`) | 20,000 | 0 failures, details below |
| Nothing else changed (`tools/unchanged.js`) | 10,000 | 0 roles outside the accent fill family differ |

Accent guarantees, over every input (both schemes, both tint modes, random base, both blends,
contrast 0.5 to 2). The shares are for sweep-generator accents, then uniform-RGB picks:

- Text on the fill at least 4.5:1 at rest, hover and active: always. At 7:1: **99.8% / 99.5%**
  (lowest 6.97).
- Hover and active never below min(rest, 7:1): always.
- Accent line at least 3:1 and accent text at least 4.5:1 on every layer: always.
- Fill exactly as picked: **64.3% / 55.4%**. Within ΔE 5: 77.4% / 70.5%. Within ΔE 10: 93.0% / 91.0%.
  Within ΔE 12: **100%**.
- The fill is exact whenever the picked colour carries 7:1. A moved fill goes lighter with dark
  ink or darker with white, and its text always beats the picked colour's: always.
- Edge on exactly when the fill is within ΔE 10 of a layer, in the line colour: always. Edge on in
  11.7% / 4.3% of themes.

## Round four: what changed from round three, and why

- **Base blend now owns the whole base effect.** The owner found the base still tinting every
  surface at blend 0 (dark, pink base: surface `#1b050b`). The cause: in round three the blend
  only set depth. Each layer's tint was the base's slope times its own distance from black or
  white, so every layer above the canvas carried the base's hue at any blend. Now base blend also
  scales the tint, by **w = b(2 − b)**: 0 at 0, 1 at 1. At blend 0 every role is identical to a
  `mono` base, in both schemes (tested on 10,000 random inputs, no exceptions). At 1 nothing
  changes from round three, so the strongest look and the cap are the same.
- **Why that curve.** A plain `w = b` is simpler but slow to start: half the slider adds only a
  third of the tint, and three presets miss. `b(2 − b)` starts with slope 2 and eases into the
  top, so the surface's chroma grows about evenly (step sizes within 2.7× of each other in dark,
  against 5× for `b`), and 11 of 13 presets stay exact. The alternative, where only the ground
  sits on the base's line and the layers above carry its tint, jumps: by 0.01 it already has
  nearly half the tint it has at 0.1. It also flattens Nord's layer chroma. See "The tint
  rule" below.
- **Presets re-solved.** Base blend still fixes depth, so each preset keeps its blend (within
  0.02, except Solarized Light), and its base gets stronger to make up for w. Two surfaces are
  no longer exact: Solarized (ΔE 0.7) and Solarized Light (ΔE 1.4).
- **New: even tint between light and dark, behind a checkbox.** Light now reads a colour's
  strength the way dark does, so one base tints both schemes about equally. Dark doesn't change.
  This is a comparison switch, not a seventh input. I recommend it on (opinion), and the
  prototype opens with it on.
- **A rounding fix.** A fitted grey could come out one 8-bit step off-grey (`#686767`, also in
  round three), because a grey's chroma read back as 2e-8 and the fit lands on rounding splits.
  `toLch` now snaps chroma below 1e-6 to 0. Without it, a mono theme isn't purely grey.
- **Unchanged:** the six inputs, depth (the surface still moves evenly through each scheme's
  range), the slope cap, the layer step, text fitting, status and syntax derivation, the
  guarantees and the 42 roles.

## Summary (round four)

The defect is fixed and the rest holds. At base blend 0 a pink base and a `mono` base give the
same 42 roles (the owner's case is now pixel-identical to mono in the preview). The tint grows
smoothly to round three's top. The guarantees hold over 6,000 random themes at contrast 1. The cost
is one thing one slider can no longer do: put a clearly tinted colour on a very shallow ground.
Solarized Light's `#fdf6e3` is that case. Even tint fixes the lopsided scheme switch.

```
resolveTheme({ scheme, base, baseBlend, accent, accentBlend, contrast }) -> { roles (43), report, meta }
```

The prototype also passes `evenTint` (a boolean), only so the owner can compare.

## Inputs (6)

| Input | Type | Default | Meaning |
|---|---|---|---|
| `scheme` | `light` or `dark` | `dark` | Which end colour the layers start from, and which way they run. |
| `base` | sRGB colour or `mono` | `mono` | Hue and strength of the tint. `mono`, and any grey, gives pure neutrals. |
| `baseBlend` | 0 to 1 | 0.25 | How much of the base's tint. 0 adds none; 1 adds its full tint. Depth follows the larger of the two blends (round six). |
| `accent` | sRGB colour or `mono` | `mono` | The fill stays as picked where its label allows; line and text are fitted to the layers (round five). `mono` uses the neutral ink. |
| `accentBlend` | 0 to 1 | 0 | How much of the accent's tint, added to the base's (round six). Works the same as base blend. |
| `contrast` | 0.5 to 2 | 1 | Layer spacing. Text is fitted after it. |

## The tint rule

**Depth.** Lightness is flare lightness `lf = cbrt(Y + 0.01)`. The surface sits at
`lf = deep + max(baseBlend, accentBlend) × (dim − deep)` (round six; round five used base blend alone), and the other layers one step `s = 0.024 × contrast`
either side. With a grey base:

| Base blend | 0 | 0.25 | 0.5 | 0.75 | 1 |
|---|---|---|---|---|---|
| Dark surface | `#0c0c0c` | `#1a1a1a` | `#252525` | `#2f2f2f` | `#393939` |
| Light surface | `#f7f7f7` | `#f2f2f2` | `#eeeeee` | `#e9e9e9` | `#e4e4e4` |

**Tint.** A colour's slope is its OKLab chroma per unit of lightness away from the end colour
(black in dark, white in light). The base's slope and the accent's are each capped and weighted
by their own blend through `w = b(2 − b)`, then added and capped again (round six; round five
moved between them by accent blend and multiplied the result by the base's `w`). Each layer is then the point at its lightness on the line with that slope. So
within one theme every layer still lies on one straight line from black or white, and chroma
still grows with depth, which keeps Nord's ramp. Base blend changes how steep the line is.

The pink base in the owner's report, accent mono, surface by base blend:

| Base blend | 0 | 0.1 | 0.25 | 0.5 | 0.75 | 1 |
|---|---|---|---|---|---|---|
| Dark | `#0c0c0c` | `#161112` | `#251619` | `#3c1b24` | `#511e2e` | `#632338` |
| Light (even) | `#f7f7f7` | `#f9f4f5` | `#fdeff2` | `#ffe8ed` | `#ffe2e8` | `#ffdce3` |

**Candidates compared** (`tools/rules.js`, the shipped engine with only the tint step changed):

| | Round three | A: ground on the line, layers carry its chroma | B: slope × b | **B: slope × b(2 − b)** |
|---|---|---|---|---|
| Blend 0 equals mono | no | yes | yes | yes |
| Smooth start | n/a | no: dark surface reaches C 0.014 at 0.01 (top 0.084) | yes, but slow | yes |
| Even growth (largest over smallest 0.05 step, dark) | 5.5 | 27.8 | 5.0 | **2.7** |
| Grey base gives depth, no tint | yes | yes | yes | yes |
| One slider enough | yes | yes | yes | yes |
| Presets exact (round-two surfaces) | 13 | 11 | 10 | **11** |
| Nord raised, overlay ΔC to nord1, nord2 | −0.003, −0.003 | −0.007, −0.009 | — | −0.005, −0.003 |
| Special cases | none | the ground layer differs by scheme | none | none |

A's onset comes from the flare lightness: the canvas's OKLab L grows like a cube root of the
blend near black, and A ties the tint to it. B has no special case at all: one multiply, no
branch, and 0 is exact because 0 × anything is 0.

**The cap (unchanged).** No layer at the dimmest end carries more than OKLCH chroma 0.105, which
is a slope of 0.262 in dark and 1.008 in light. A base or accent above it is muted to it, keeping
its hue. Since w is 1 at full blend, the top of the slider looks exactly as in round three.

**Out of range.** The function stays total. Numbers are clamped, non-colours become `mono`, a
non-boolean `evenTint` falls back to the default, and all of it is reported.

## Even tint between light and dark

**The problem.** With plain mixing, light grounds live in a band about a quarter as deep as dark
ones, so the same base tints light about a quarter as much. How much less depends on the base's
own lightness, since light reads the slope from white and dark from black. Across the 18 swatches
and blends 0.25 to 1, the light surface's tint is 0.23 to 1.82 times the dark one's (median 0.34
to 0.44), in OKLab ΔE from the grey theme. Eclipse's base in light is `#f2f3f4` (chroma 0.002). Clay's light-preset
base in dark is a rich brown, `#2c220e`.

**The correction.** With even tint on, light reads every colour's slope from black, as dark does,
and scales it by the ratio of the two caps (1.008 / 0.262 ≈ 3.85). No constant is added. One colour
is then the same fraction of the cap in both schemes: a swatch is muted in light exactly when it's
muted in dark (Blue, Indigo, Violet, Pink), and a muted swatch is equally strong in both. Dark is
untouched.

**What it does.**

- Light/dark tint ratio: median 0.64 at blend 0.25, 0.74 at 0.5, 0.87 at 1. The range is now
  0.41 to 1.01, mostly from blues and pinks clipping near white. By eye, I'd call that about
  even (opinion). Tint shows more on a near-white page, so light reads a little stronger than
  its OKLab number, and with vivid swatches at mid blends light reads slightly stronger than dark.
- Muted and dark bases get two to four times stronger in light: Eclipse's base in light at 0.2
  becomes `#f1f4f6`, Muted blue at 0.25 becomes `#f0f2f7`. Pastels and yellows get slightly
  weaker. Yellow can reach only 78% of the light cap and Teal 65%, as they already do in dark,
  because sRGB limits their slope from black.
- More layers reach the sRGB edge. Blue, violet and pink near white can't hold the chroma the line
  asks for, so they keep their lightness and lose some tint. The report notes it: about one light
  theme in five with a random base, against almost none with even tint off. No guarantee is
  affected.
- **Presets:** dark presets are identical with it on or off. The three tinted light presets need
  different bases (muted ones, which also give a matching dark theme) and stay exact. Solarized
  Light is off by ΔE 1.6.

**Recommendation (opinion): on.** A theme should keep its character when the scheme switches, and
with even tint a base means the same thing in both schemes. The costs are the near-white clipping
and a rule that's a little less literal ("light reads the slope from black").

## Exactness

`tools/solve.js` keeps each preset's depth-implied blend (two decimals). It builds the base at
OKLab L 0.6, or the nearest lightness inside sRGB, on the line the rule needs at that blend, and
searches the 26 neighbours. When nothing hits, it finds the closest surface.

| Preset | Surface | Base, blend (plain) | Base, blend (even) | Result |
|---|---|---|---|---|
| iceberg | `#e7eef5` | `#4285c1` 0.53 | `#6b839a` 0.53 | exact both |
| clay | `#f4eee3` | `#c89514` 0.45 | `#967c4c` 0.45 | exact both |
| meadow | `#eef2ec` | `#56932b` 0.33 | `#718865` 0.33 | exact both |
| solarized-light | `#fdf6e3` | `#f3c52d` 0.23 | `#a27901` 0.27 | ΔE 1.4 (`#f7f3e7`), even 1.6 (`#f7f2e7`) |
| eclipse | `#15181b` | `#6384a3` 0.20 | same | exact |
| cobalt | `#141414` | `mono` 0.13 | same | exact |
| graphite | `#181818` | `mono` 0.21 | same | exact |
| hornet | `#0c0c0c` | `mono` 0 | same | exact |
| midnight | `#13161c` | `#5a7dcb` 0.17 | same | exact |
| forest | `#151a16` | `#5e8d69` 0.23 | same | exact |
| rose | `#1b1617` | `#a3707b` 0.20 | same | exact |
| nord (contrast 1.35) | `#2e3440` | `#73809a` 0.98 | same | exact |
| solarized (contrast 1.4) | `#002b36` | `#038eaf` 0.61 | same | ΔE 0.7 (`#092b35`) |

- **Solarized.** base03 is an sRGB-edge colour. Its slope, 0.182, is the most any sRGB colour at
  its hue (220°) has, so only a weight of 1 reaches it. At blend 0.61 the weight is 0.85. Any rule
  that keeps blend 0 clean and grows smoothly has the same miss. ΔE 0.7 is below what most people
  see side by side.
- **Solarized Light.** Its surface is nearly white (depth 0.04) yet clearly tinted (chroma 0.026).
  With blend owning the tint, a ground that shallow gets almost none. The solver trades a little
  lightness for tint, at blend 0.23. As a pair, it fits the canonical palette better than
  round three did: canvas ΔE 3.1 → 0.7 against base2, largest layer error 3.1 → 1.4.
- Nord is exact with the same base as round three, at the "deep, strongly tinted" end: at 0.98 the
  weight is 0.9996.

## One slider or two

**Still one**, with one new limit. Base blend now moves tint and depth together from zero, and the
base's saturation sets the ratio between them. A very shallow, strongly tinted ground is out of
reach, because at small blends the weight is small. That limit is Solarized Light's surface, and
light themes in that style generally. A second slider, with depth separate from base blend, would
bring it back exactly, at the cost of a seventh input. That's the owner's call (see the open
questions). Otherwise, for more tint at the same depth, pick a stronger base, as before. At blend
0.2, Nord's base gives `#16181a` (C 0.005), Muted blue `#15181c` (C 0.009) and Blue `#131821`
(C 0.020).

## Defaults

A fresh theme is `base: mono`, `baseBlend: 0.25`, unchanged:

| | Canvas | Surface | Raised | Overlay |
|---|---|---|---|---|
| Dark | `#101010` | `#1a1a1a` | `#222222` | `#2a2a2a` |
| Light | `#eaeaea` | `#f2f2f2` | `#fafafa` | `#ffffff` |

*Opinion:* `mono` is still the right fresh base. For today's cool feel, Muted blue at 0.25 gives
`#171a1f` in dark and `#f0f2f7` in light with even tint (weaker than round three's `#141a26`,
because the weight at 0.25 is 0.44).

## Derivation

All maths is in OKLab and OKLCH with the matrices from `headless/src/color.rs`. Every colour is
quantised to 8 bits before any check.

1. **Tint.** Each colour's slope is read from this scheme's end colour (plain) or from black and
   scaled by 3.85 in light (even). Each is capped (0.262 dark, 1.008 light), weighted by its own
   `w = b(2 − b)`, and the two are added and capped again (round six).
2. **Layers.** For each target flare lightness, a 24-step search finds the OKLab L whose colour
   (L, slope × distance from the end, hue) has that luminance, gamut-clipped by chroma.
3. **Surface tint.** The surface's chroma and hue drive text chroma, ink, fills, the light scrim
   and shadow, and the half-tint pull on status and syntax. At blend 0 that chroma is exactly 0,
   which is why every role matches `mono`.
4. **Ink, status, syntax, text:** unchanged from round three. **Accent:** see "What changed from
   round four" (fill, on-colour, line, text, edge).

**Report.** As in round three. The muted clamp is still reported at blend 0 (the base is muted;
it just isn't applied). The hue-cancellation note needs a non-zero weight.

## Guarantees

Round three's guarantees hold for any input, with round five's accent change: text 7:1 and 4.5:1,
accent line and strong border 3:1, accent text 4.5:1, text on the accent fill 4.5:1 in every
state (7:1 in 99.5%), status and syntax 4.5:1, a 0.06 text hierarchy gap, and adjacent layers
ΔL ≥ 0.02. The fill's old 3:1 against the layers is gone. The table below is round four's run;
round five's is at the top.

| Test | Cases | Result |
|---|---:|---|
| Base blend 0 equals `mono`, all 42 roles byte-identical (`tools/invariant.js`) | 10,000 (2,500 per scheme × tint mode; random base, accent, accent blend, contrast) | 0 failures |
| Accent blend 0, mono base: every neutral role is pure grey | 10,000 | 0 failures |
| Accent blend 0: the neutral tint ignores the accent | 10,000 | 0 failures |
| Growth along base blend, 0 to 1 in 0.01 steps | 1,200 bases × 101 blends | never falls beyond rounding (largest dip 0.42 ΔE); tint at 0.01 at most 0.14 ΔE (one 8-bit step) |
| Random themes, contrast 1 (`tools/sweep.js`) | 6,000 | no failures; smallest margin 0.050, hierarchy 0.060 |
| Random themes, contrast 0.9 to 2 | 4,000 | 1 failure: light ΔL floor at contrast 0.91 (round three had 4) |
| Garbage inputs (`tools/test-engine.js`) | 5,000 | none threw, all returned 42 valid roles |

The sweeps draw scheme, tint mode, base, accent and both blends at random, as in round three. The
contrast miss is the known rounding limit near white below contrast 0.98, and the readout flags
it. A resolve takes about 0.3 ms in Node.

## Fidelity

See `fidelity.md`. ΔE is the OKLab distance × 100.

| Reference | Layers (intended) | Text | Accent | Status | Syntax | Change from round three |
|---|---:|---:|---:|---:|---:|---|
| Nord | 1.2 | 10.3 | 0.0 | 13.2 | 13.3 | raised 1.6 → 1.7 (rounding) |
| Solarized Dark | 0.6 | 19.0 | 3.7 | 18.5 | 21.5 | surface 0.0 → 0.7; base02 unchanged at 0.4 |
| Solarized Light (plain) | 1.0 | 18.1 | 7.4 | 14.4 | 18.1 | surface 0.0 → 1.4, canvas 3.1 → 0.7 |
| Solarized Light (even) | 1.1 | 18.2 | 7.5 | 14.5 | 18.2 | surface 1.6, canvas 0.7 |

Against today's twelve themes, the layer ΔE per preset is within 0.1 of round three's.

**Round five** changes only the accent fill column. Solarized Dark's accent goes from 3.7 to 6.1 and
Solarized Light (even) from 7.5 to 9.2, because the fill now aims for 7:1. Nord stays at 0.0.
The per-preset accent fill against today's themes is in "What changed from round four".

## Awkward inputs

Captures are in `shots/`.

- **The owner's case** (`sheet-owner-case.png`): dark, Pink base, blend 0, Blue accent. The
  preview is pixel-identical to the `mono` base beside it. The readout says the base has no effect
  at 0 and is also muted, so the muting is visible before the slider moves.
- **Base blend 0 with accent blend above 0** (round six): the accent tints the neutrals exactly as
  it would as the base at that blend, and sets depth. Round five tinted nothing here.
- **Vivid bases at full blend:** unchanged from round three in dark (red `#64251e`, blue
  `#21386a`). In light with even tint, every vivid swatch reaches the cap or the sRGB edge: Pink
  `#ffdce3`, Blue `#d7e5ff`, Yellow `#f2e4b5`. *Opinion:* the loudest the system goes, the same as
  dark's maroon, and fine to allow.
- **Same hue, opposite hues, greys:** as in round three, at reduced strength below full blend.
  Round six: see "Both blends above 0" at the top. Grey, black and white bases equal `mono`.

## Presets

`[id, scheme, base, baseBlend, accent, accentBlend, contrast]`, plain, with the even-tint base for
light presets:

```
iceberg          light  #4285c1  0.53  #1668d6  0  1      even: #6b839a 0.53
clay             light  #c89514  0.45  #c96442  0  1      even: #967c4c 0.45
meadow           light  #56932b  0.33  #188f52  0  1      even: #718865 0.33
solarized-light  light  #f3c52d  0.23  #268bd2  0  1      even: #a27901 0.27
eclipse          dark   #6384a3  0.20  #f0b24d  0  1
cobalt           dark   mono     0.13  #5b9dff  0  1
graphite         dark   mono     0.21  #97d14d  0  1
hornet           dark   mono     0     #ffd400  0  1
midnight         dark   #5a7dcb  0.17  #6d8cff  0  1
forest           dark   #5e8d69  0.23  #4dc98a  0  1
rose             dark   #a3707b  0.20  #f65c8a  0  1
nord             dark   #73809a  0.98  #88c0d0  0  1.35
solarized        dark   #038eaf  0.61  #b58900  0  1.4
```

Dark bases are stronger than round three's (the weight at 0.2 is 0.36). Nord's is unchanged.

## The prototype

As round three, plus one checkbox under the scheme toggle: **Even tint in light and dark**, with
a one-line hint that says what it does in its current state. The preset row follows it (light
presets switch to their even-tint bases). Turning it on or off keeps the current base and blend, so
the owner can compare one base in both schemes with either rule. The base readout now says the
tint strength ("Tint at 44% of full strength, depth 25% …") and, at 0, "the base has no effect".
URLs carry `even=1` or `even=0`.

**Round five.** The accent readout shows picked, fill, text on fill with its ratio, line and
edge, then one line on how the fill was fitted (as picked, or lightened or darkened by ΔE n, or
short of 7:1) and one on the edge (the fill's ΔE to the nearest layer against the threshold of 10).
In the checks table, "Accent fill on layers" and "Focus ring on layers" give way to "Text on
accent fill (aim 7)", "Text on fill, hover and active" and "Accent line on layers", plus an
unscored "Fill edge" row. Only role bindings changed in the preview's CSS: the focus ring, tab
underline, selected-row bar and current nav icon read `accent.line`, and the primary button,
checked checkbox and switch draw `inset 0 0 0 1px accent.edge`, which is transparent unless the
edge is on. For ordinary accents nothing extra is painted.

**Round six.** The base readout gives the base's tint strength and the depth, and says when accent
blend sets the depth. The accent readout gives the accent's tint strength, whether it sets depth
("as the larger blend"), and whether its tint is muted for the ground. The checks table renames
"On-danger text" to "Text on danger fill (aim 7)" and adds an unscored "Danger fill edge" row.
The danger button draws `inset 0 0 0 1px status.danger.edge`, transparent unless the edge is on.

## Simplicity

| | Round three | Round four | Round five |
|---|---|---|---|
| Inputs | 6 | 6 (plus the comparison switch until it's decided) | the same |
| Special cases | mono; zero-slope bases | the same; the tint weight is one multiply, no branch | the same |
| Taste constants for the ground | 1 top chroma, 1 default blend | the same; the even-tint factor is the cap ratio, and the curve has no constant | the same |
| Accent constants | — | — | 7:1 aim, ΔE 12 reach, edge below ΔE 10, dark ink L 0.18 |
| Roles | 42 | 42 | 43 (44 in round six: `status.danger.edge`) |
| Resolver lines, model part / total | 169 / 367 | 177 / 376 | 184 / 432 |

**Rust port.** Round six: `slope = cap(w(base_blend) * cap(base) + w(accent_blend) * cap(accent))`
with `w(b) = b * (2.0 - b)`, and `depth = base_blend.max(accent_blend)`, and the even rule is a choice of end colour and one ratio. If the
switch is decided, it disappears from the port. Round five adds one fit,
`fit_accent_fill(picked) -> { fill, hover, active, on }`, which reads only the picked colour (no
layers), and the edge test, which is a minimum over four distances. Round six retires the old fill fit: `status.danger.solid` uses
`fit_accent_fill` too.

## What the owner gives up

Everything round three gave up, minus the lopsided scheme switch if even tint goes in. New with
this round: a shallow ground can't be strongly tinted, so Solarized Light's surface isn't exact,
and Solarized's base03 misses by ΔE 0.7 because it sits on the sRGB edge. With even tint, near-white
blues and pinks clip at the sRGB edge more often.

**Round five:** mid-tone accents move up to ΔE 12 from the picked colour to reach 7:1, so
Iceberg, Clay, Meadow and Solarized Light look less like today's themes. Three of them switch
from white text to dark. A fill no longer stands 3:1 from the layers, so in greyscale a yellow
button on white is found by its label alone. Two neighbouring picks can land on different exits
(`#c020f8` goes darker with white text, `#c818f8` lighter with dark text), so dragging a picker
through saturated violets can flip the ink.

## Open questions for the owner

1. **Even tint: on or off?** I recommend on (opinion). It changes only light themes.
2. **Shallow, tinted grounds.** Accept that Solarized Light lands at ΔE 1.4, or split depth from
   base blend (seven inputs) to get it exact?
3. **Curve shape.** `b(2 − b)` puts noticeable tint in the first quarter of the slider (Pink at
   0.25: `#251619`). `b` would be gentler early but uneven, and it loses Midnight's exactness. I
   prefer `b(2 − b)` (opinion). The sweep captures show it.
4. Round three's questions on the muted swatch row, the cap level and the fresh default are
   unchanged.
5. **Round five: 7:1 for mid-tones, or keep authored fills closer?** At 7:1, Clay, Meadow and
   Solarized Light swap white text for dark, and Iceberg's blue deepens (ΔE 6.8). A white-first
   rule (darken with white, stop at ΔE 12) would keep white text, short of 7:1: Clay `#a1401c` at
   6.4:1, Meadow `#006938` at 6.8:1, Solarized Light `#0067a3` at 6.1:1. Those darker fills drift
   toward rust and navy. I prefer 7:1 (opinion): mid-tones are exactly where text is hard to read.
6. **Round five: should yellow get an edge?** Not under this threshold (ΔE 20, chroma separates
   it). A luminance rule (edge below 1.5:1) would add one to yellow and pale fills, but not to
   amber. I'd leave yellow without one (opinion), in line with minimal borders.
7. **Round five: hover and active.** They darken while text stays at 7:1 or more (see above). Say
   so if "keep or improve" was meant literally.
8. **Round five: `status.danger.solid`** still uses the old fit. Answered: it follows the accent
   rule (round six).

## Risks and what I could not verify

- **Rust is not written.** As before.
- **"About the same strength" in light and dark is judged by eye** on the captures, backed by an
  OKLab ratio that's knowingly not perceptual for tints near white. There's no user test.
- **The curve and the even factor were chosen** by the invariant, evenness, exactness and the
  captures. The owner should drag the slider.
- **Only Chromium** was checked headlessly (captures plus a click-through of the checkbox and the
  preset lists). GPUI and other browsers are unverified, and contrast is WCAG 2, as before.
- **Round five.** The ΔE 12 reach, the ΔE 10 edge threshold and the dark ink's lightness were
  chosen from the numbers and the captures, not a user test. Whether a yellow button on white
  needs an edge is a judgement. The radio rule isn't captured (the preview has no radio). Hover
  and active were checked on a states sheet (`shots/fill-states.png`), not by hovering in the page.
