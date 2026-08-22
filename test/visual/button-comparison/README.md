# Button visual comparison (g15.047)

One same-run comparator over the accepted 18-case Button inventory
(`../fixtures/button-visual-inventory.json`) in three runtimes: Svelte, React,
and GPUI. This is a **diagnostic mechanism proof**, not component authority,
not a completion gate, and not a baseline system.

```sh
effigy test:visual-button-comparison        # focused tests + full batch, disposable output
bun test/visual/button-comparison/run.ts    # the batch alone
bun test/visual/button-comparison/run.ts --out=<dir>          # explicit evidence directory
bun test/visual/button-comparison/run.ts --fixture=button/rest-secondary  # slice
```

## What happens in one run

1. Every fixture is rendered by its **real** runtime implementation from the
   fixture's fully resolved values — hand-written Button-only adapters, no
   generic schema, no code generation:
   - Svelte/React: private capture-only fixture hosts in the two previews
     (`packages/{svelte,react}/preview/src/fixture-host/`), driven by pinned
     headless Chromium at device scale 2 (`capture-web.ts`).
   - GPUI: `poodle-offscreen-capture --fixture <name>` on the adopted Metal
     `HeadlessAppContext` seam (`capture-gpui.ts` drives it).
2. Every runtime captures every fixture **twice**; the pair must be
   byte-identical or the batch stops. No averaging, no retrying away, no frame
   picking.
3. Every PNG is verified against its own typed receipt
   (`poodle.button-visual-capture.v1`, `receipt.ts`) before any comparison:
   schema, closed key sets, exact landmark set, device dimensions, SHA-256.
   Missing, stale, aliased, or hash-mismatched pairs fail closed.
4. Comparisons (`compare.ts`, policy constants in `policy.ts` — the card's
   fixed table, not negotiable in this thread):
   - **Svelte ↔ React — exact:** identical dimensions, zero logical-edge delta
     on every landmark, exactly equal role evidence, zero differing pixels.
   - **Svelte ↔ GPUI — renderer-aware:** root edges ≤0.5 logical px;
     icon/spinner centre and size ≤1; content centre ≤1, extent ≤2; role
     colours ≤1 in any 8-bit sRGB channel; border/focus-ring width ≤0.5;
     shadow layer count/inset exact, geometry ≤0.5; pixelmatch threshold 0.1,
     `includeAA: false`, ≤3% of the viewport.
   - Channels are independent: a pixel pass never hides a geometry or role
     failure.
5. Findings the current Button contract already decides (§12 known deltas,
   e.g. GPUI paints no box-shadow) are reported as **known deltas** with their
   citation — visible in every output, never silently allowlisted, and never
   per-fixture. Anything else fails the run.

## Output

One output directory per run (default `out/` is disposable; `--out` for
committed evidence):

- `captures/<runtime>/<fixture>.png` + `.json` — 54 verified pairs
- `diffs/<fixture>--svelte-react.png`, `diffs/<fixture>--svelte-gpui.png` — 36 diffs
- `summary.json` — machine-readable verdicts, hashes, environment, policy echo
- `contact-sheet.html` — the operator review surface, canonical fixture order,
  native device scale

Committed copies under `docs/logs/` are point-in-time review evidence. The
comparator never reads them; there is no update/refresh command and nothing
here can mark Button — or any component — complete.

## Scene and determinism rules

- 240×80 logical viewport at 2×, Button border-box origin at logical (16,16),
  whole viewport painted in the fixture theme's canvas background.
- Final declared state rendered directly (`disabled`/`loading`/`pressed`
  props); no input replay.
- Animations frozen at their declared initial frame on every runtime (CSS
  animation freeze + Playwright `animations: "disabled"` on web; GPUI
  reduce-motion), clocks pinned; the loading spinner is the edge this exists
  for.
- Web captures bundle the repo's Inter TTFs and GPUI loads the same files, so
  glyph differences mean text-stack differences, not missing fonts.

## Focused tests

`compare.test.ts` plants the card's failure set in memory — missing capture,
two-logical-pixel root shift, missing icon/spinner landmark, changed role
colour/shadow, PNG tamper, a pixel change beyond 3% — through the production
compare/verify functions, plus receipt shape negatives and capture-set
completeness. No broken fixture data is ever committed.
