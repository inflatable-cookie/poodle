// The blend invariants, tested on random inputs, in both schemes and both tint modes.
// Round four (kept):
//  1. Base blend 0: every one of the 44 roles is identical to the same input with base `mono`.
//  2. Accent blend 0, base mono: every neutral role is a pure grey, whatever the accent.
//  3. Accent blend 0, any base: the neutral tint is identical to the same input with accent `mono`.
//  4. Growth along base blend (accent mono): the tint never falls beyond rounding, small first step.
// Round six (the two pairs are independent):
//  R1  Each blend works alone. Base blend 0, accent blend 0 → 1: the tint grows smoothly and
//      monotonically, and the neutrals equal those of the same colour used as the base at the
//      same blend with accent blend 0. (Accent blend 0 matching round five is tools/unchanged.js.)
//  R2  Zero means zero, separately. Base blend 0: the base colour changes no role, with accent
//      blend above 0. Accent blend 0: the accent colour changes no neutral, with base blend above 0.
//  R3  Symmetry. Swapping (base, base blend) with (accent, accent blend) gives the same neutrals.
//  R4  Both together. No jump as either slider leaves 0; no dip along either slider while the hues
//      are within 90°; the single chroma cap holds; opposite hues cancel and the report says so.
//  R5  Depth follows the larger blend: base blend 0, accent blend t sits at depth t.
// "Neutral" means the tint-carrying neutrals: layers, ink fills, neutral selection, both borders,
// disabled text, scrim and shadow, plus the tint vector and depth that text and derived colours
// read. Primary and secondary text are fitted against the accent's selection and status tints as
// well, so their lightness can follow the accent; R2 and R3 count those cases separately.
// Usage: node tools/invariant.js [cases per scheme and tint mode, default 2500]
const T = require("./extract.cjs");
const N = +(process.argv[2] || 2500);
let seed = 7;
const rnd = () => ((seed = (seed * 1103515245 + 12345) % 2147483648) / 2147483648);
const randRgb = () => "#" + [0, 1, 2].map(() => Math.floor(rnd() * 256).toString(16).padStart(2, "0")).join("");
function randColour() { // the sweep's generator: uniform RGB, vivid, muted, and greys/black/white
  const k = rnd();
  if (k < 0.4) return randRgb();
  if (k < 0.7) return T.hex(T.lch(0.3 + rnd() * 0.65, 0.12 + rnd() * 0.25, rnd() * 360));
  if (k < 0.95) return T.hex(T.lch(rnd(), rnd() * 0.06, rnd() * 360));
  const g = Math.floor(rnd() * 256).toString(16).padStart(2, "0"); return "#" + g + g + g;
}
const vivid = () => T.hex(T.lch(0.45 + rnd() * 0.45, 0.1 + rnd() * 0.2, rnd() * 360));
const unit = () => Math.round(rnd() * 100) / 100;
const pos = () => Math.round((0.01 + rnd() * 0.99) * 100) / 100; // 0.01 to 1
const k1 = () => (rnd() < 0.5 ? 1 : Math.round((0.5 + rnd() * 1.5) * 100) / 100);
const same = (a, b) => a.length === b.length && a.every((x, i) => x === b[i]);
const NEUTRAL = ["layer.canvas", "layer.surface", "layer.raised", "layer.overlay", "fill.rest", "fill.hover", "fill.active", "select.neutral", "border.subtle", "border.strong", "text.primary", "text.secondary", "text.disabled", "scrim", "shadow"];
const TINT_ONLY = ["layer.canvas", "layer.surface", "layer.raised", "layer.overlay", "fill.rest", "fill.hover", "fill.active", "select.neutral", "border.subtle", "scrim", "shadow"];
const TINTED = TINT_ONLY.concat(["border.strong", "text.disabled"]);
const TEXT = ["text.primary", "text.secondary"];
const LAYERS = ["layer.canvas", "layer.surface", "layer.raised", "layer.overlay"];
const dE = (x, y) => { const a = T.toLab(x), b = T.toLab(y); return 100 * Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]); };
const R = (o) => T.resolveTheme(o);
// Same tint vector and depth (the values text and derived colours read).
const sameMeta = (x, y) => x.meta.depth === y.meta.depth && x.meta.chroma === y.meta.chroma && (x.meta.chroma === 0 || x.meta.hue === y.meta.hue);
const diffKeys = (x, y, keys) => keys.filter((k) => !same(x.roles[k], y.roles[k]));

const out = {};
const bucket = (k) => (out[k] = out[k] || { cases: 0, fails: 0, ex: null, extra: {} });
const fail = (k, input, detail) => { const b = bucket(k); b.fails++; if (!b.ex) b.ex = { input, detail }; };
const tick = (k) => bucket(k).cases++;
const extra = (k, name) => { const b = bucket(k); b.extra[name] = (b.extra[name] || 0) + 1; };

for (const scheme of ["dark", "light"]) for (const even of [false, true]) {
  for (let i = 0; i < N; i++) {
    const common = { scheme, contrast: k1(), evenTint: even };
    const base = randColour(), accent = rnd() < 0.15 ? "mono" : randColour();
    const bb = unit(), ab = unit();

    // Round four 1: base blend 0 equals mono base at blend 0, all 44 roles (any accent blend).
    { const abx = rnd() < 0.3 ? 0 : ab;
      const a = R({ ...common, base, baseBlend: 0, accent, accentBlend: abx }).roles, m = R({ ...common, base: "mono", baseBlend: 0, accent, accentBlend: abx }).roles;
      tick("inv1"); const d = Object.keys(m).filter((k) => !same(a[k], m[k]));
      if (d.length || Object.keys(a).length !== 44) fail("inv1", { ...common, base, accent, accentBlend: abx }, d.join(", ")); }
    // Round four 2: accent blend 0, mono base: every neutral is grey.
    { const g = R({ ...common, base: "mono", baseBlend: bb, accent, accentBlend: 0 }).roles;
      tick("inv2"); const t = NEUTRAL.filter((k) => !(g[k][0] === g[k][1] && g[k][1] === g[k][2]));
      if (t.length) fail("inv2", { ...common, baseBlend: bb, accent }, t.map((k) => `${k} ${T.hex(g[k])}`).join(", ")); }

    // R2a: base blend 0, accent blend above 0: the base changes no role.
    { const abp = pos(), ac = rnd() < 0.1 ? "mono" : randColour();
      const a = R({ ...common, base, baseBlend: 0, accent: ac, accentBlend: abp }).roles, m = R({ ...common, base: "mono", baseBlend: 0, accent: ac, accentBlend: abp }).roles;
      tick("R2a"); const d = Object.keys(m).filter((k) => !same(a[k], m[k]));
      if (d.length) fail("R2a", { ...common, base, accent: ac, accentBlend: abp }, d.join(", ")); }
    // R2b (and round four 3): accent blend 0, base blend above 0: the accent changes no neutral.
    { const bbp = pos();
      const x = R({ ...common, base, baseBlend: bbp, accent, accentBlend: 0 }), y = R({ ...common, base, baseBlend: bbp, accent: "mono", accentBlend: 0 });
      tick("R2b"); const d = diffKeys(x, y, TINTED);
      if (d.length || !sameMeta(x, y)) fail("R2b", { ...common, base, baseBlend: bbp, accent }, d.join(", ") || "tint vector or depth");
      if (diffKeys(x, y, TEXT).length) extra("R2b", "text lightness refitted (accent selection or status tint)"); }

    // R3: swap the pairs. Accent roles differ by design; the neutrals must not.
    { const c1 = randColour(), c2 = randColour(), x1 = unit(), x2 = unit();
      const p = R({ ...common, base: c1, baseBlend: x1, accent: c2, accentBlend: x2 }), q = R({ ...common, base: c2, baseBlend: x2, accent: c1, accentBlend: x1 });
      tick("R3"); const d = diffKeys(p, q, TINTED);
      if (d.length || !sameMeta(p, q) || p.meta.slope !== q.meta.slope) fail("R3", { ...common, base: c1, baseBlend: x1, accent: c2, accentBlend: x2 }, d.join(", ") || "tint vector or depth");
      if (diffKeys(p, q, TEXT).length) extra("R3", "text lightness differs (fitted against the accent's tints)"); }

    // R1 (point form): base blend 0, accent c at t equals base c at t, accent blend 0 (neutrals).
    { const c = randColour(), t = unit();
      const a = R({ ...common, base: randColour(), baseBlend: 0, accent: c, accentBlend: t }), b = R({ ...common, base: c, baseBlend: t, accent: rnd() < 0.5 ? "mono" : randColour(), accentBlend: 0 });
      tick("R1"); const d = diffKeys(a, b, TINTED);
      if (d.length || !sameMeta(a, b)) fail("R1", { ...common, c, t }, d.join(", ") || "tint vector or depth"); }

    // R4: the cap holds with both blends above 0 (slope never above the scheme's maximum).
    { const p = R({ ...common, base: rnd() < 0.5 ? vivid() : randColour(), baseBlend: pos(), accent: rnd() < 0.5 ? vivid() : randColour(), accentBlend: pos() });
      tick("R4cap"); if (p.meta.slope > p.meta.smax * (1 + 1e-9)) fail("R4cap", { scheme, even }, `slope ${p.meta.slope} > ${p.meta.smax}`);
      const cmax = Math.max(...LAYERS.map((k) => T.toLch(p.roles[k])[1]));
      bucket("R4cap").extra.maxLayerChroma = Math.max(bucket("R4cap").extra.maxLayerChroma || 0, cmax); }

    // R5: depth is the larger blend.
    { const x1 = unit(), x2 = unit(), p = R({ ...common, base: randColour(), baseBlend: x1, accent, accentBlend: x2 });
      tick("R5"); if (p.meta.depth !== Math.max(x1, x2)) fail("R5", { x1, x2 }, `depth ${p.meta.depth}`); }
  }
}
const line = (k, label) => { const b = out[k]; console.log(`${label}: ${b.cases} cases, ${b.fails} failures`, b.ex ? JSON.stringify(b.ex) : "", Object.keys(b.extra).length ? JSON.stringify(b.extra) : ""); };
line("inv1", "round four 1 (base blend 0 = mono, all 44 roles, any accent blend)");
line("inv2", "round four 2 (accent blend 0, mono base: neutrals grey)");
line("R1", "R1 point form (base blend 0 + accent c at t = base c at t + accent blend 0, neutrals)");
line("R2a", "R2a (base blend 0, accent blend > 0: base changes no role)");
line("R2b", "R2b = round four 3 (accent blend 0, base blend > 0: accent changes no neutral)");
line("R3", "R3 (swap the pairs: same neutrals)");
line("R4cap", "R4 cap (both blends > 0: slope within the cap)");
line("R5", "R5 (depth = larger blend)");

// Tint along one slider: surface ΔE from the same input with both colours mono (same depth, no tint).
const tintOf = (inp) => dE(R(inp).roles["layer.surface"], R({ ...inp, base: "mono", accent: "mono" }).roles["layer.surface"]);
const sweep = (inp, key) => { const v = []; for (let j = 0; j <= 100; j++) v.push(tintOf({ ...inp, [key]: j / 100 })); return v; };
const hueGap = (a, b) => { const x = T.toLch(T.parseHex(a))[2], y = T.toLch(T.parseHex(b))[2]; return Math.abs(((x - y) % 360 + 540) % 360 - 180); };

// Round four 4 and R1 (sweep form): growth along base blend (accent mono) and along accent blend
// (base blend 0), for the same 300 random colours per scheme and mode. The two must be identical.
{
  let n = 0, drops = 0, worstDrop = 0, worstFirst = 0, worstRatio = 0, mirrorMax = 0, dropEx = null, ratioMin = Infinity, ratioMax = 0;
  for (const scheme of ["dark", "light"]) for (const even of [false, true]) for (let i = 0; i < 300; i++) {
    const c = randColour();
    const gb = sweep({ scheme, base: c, accent: "mono", accentBlend: 0, evenTint: even }, "baseBlend");
    const ga = sweep({ scheme, base: rnd() < 0.5 ? "mono" : randColour(), baseBlend: 0, accent: c, evenTint: even }, "accentBlend");
    n++;
    for (let j = 0; j <= 100; j++) { mirrorMax = Math.max(mirrorMax, Math.abs(gb[j] - ga[j])); if (gb[j] > 2) { ratioMin = Math.min(ratioMin, ga[j] / gb[j]); ratioMax = Math.max(ratioMax, ga[j] / gb[j]); } }
    let peakA = ga[0]; for (let j = 1; j <= 100; j++) { peakA = Math.max(peakA, ga[j]); const dd = peakA - ga[j]; if (dd > 0.6) { drops++; if (!dropEx) dropEx = { scheme, c, even, at: j / 100, drop: dd.toFixed(2) }; } worstDrop = Math.max(worstDrop, dd); }
    if (ga[100] > 8) { worstFirst = Math.max(worstFirst, ga[1]); worstRatio = Math.max(worstRatio, ga[1] / (ga[100] / 100)); }
  }
  console.log(`R1 sweep (base blend 0, accent blend 0 → 1 in 0.01 steps): ${n} colours × 101; falls below the running peak over 0.6 ΔE: ${drops}${dropEx ? " " + JSON.stringify(dropEx) : ""}; largest drop ${worstDrop.toFixed(2)} ΔE;`,
    `tint at 0.01 at most ${worstFirst.toFixed(2)} ΔE; first step over mean step, worst ${worstRatio.toFixed(2)};`,
    `accent-alone tint over base-alone tint at the same blend: ${ratioMin.toFixed(3)} to ${ratioMax.toFixed(3)} (largest gap ${mirrorMax.toFixed(3)} ΔE)`);
}

// R4: no jump as either slider leaves 0 with the other above 0, and no dip along either slider
// while the hues are within 90°. Opposite hues (over 150° apart) are counted, not failed.
{
  let n = 0, firstMax = 0, firstEx = null, dips = 0, dipEx = null, worstDip = 0, oppDip = 0, oppN = 0, clipDips = 0, askedDips = 0, askedEx = null, askedDipMax = 0;
  for (const scheme of ["dark", "light"]) for (const even of [false, true]) for (let i = 0; i < 300; i++) {
    const c1 = rnd() < 0.6 ? vivid() : randColour(), c2 = rnd() < 0.6 ? vivid() : randColour(), other = Math.round((0.05 + rnd() * 0.95) * 100) / 100;
    for (const [key, inp] of [["accentBlend", { scheme, base: c1, baseBlend: other, accent: c2, evenTint: even }], ["baseBlend", { scheme, base: c1, accent: c2, accentBlend: other, evenTint: even }]]) {
      n++;
      // first step: every neutral, 0 → 0.01
      const a = R({ ...inp, [key]: 0 }), b = R({ ...inp, [key]: 0.01 });
      const jump = Math.max(...TINTED.filter((k) => a.roles[k][3] === 1).map((k) => dE(a.roles[k], b.roles[k])));
      if (jump > firstMax) { firstMax = jump; firstEx = { ...inp, key }; }
      const g = sweep(inp, key), gap = hueGap(c1, c2);
      // The tint the rule asks for (the surface's requested chroma) and the visible tint (ΔE).
      // A dip is any fall below the highest value reached so far along the slider (cumulative).
      // A visible dip where the asked-for tint did not fall is sRGB clipping: the hue turned
      // toward one sRGB can't hold at that lightness (the report's sRGB note).
      const asked = [], clip = [];
      for (let j = 0; j <= 100; j++) { const t = R({ ...inp, [key]: j / 100 }); asked.push(t.meta.chroma); clip.push(t.meta.chroma - T.toLch(t.roles["layer.surface"])[1]); }
      let worst = 0, peak = g[0], worstAsked = 0, peakA = asked[0], at = 0;
      for (let j = 1; j <= 100; j++) {
        peak = Math.max(peak, g[j]); if (peak - g[j] > worst) { worst = peak - g[j]; at = j; }
        peakA = Math.max(peakA, asked[j]); worstAsked = Math.max(worstAsked, (peakA - asked[j]) / Math.max(peakA, 1e-9));
      }
      if (gap <= 90) {
        worstDip = Math.max(worstDip, worst); askedDipMax = Math.max(askedDipMax, worstAsked);
        if (worstAsked > 0.01) { askedDips++; if (!askedEx) askedEx = { ...inp, key, gap: gap.toFixed(0), fall: (100 * worstAsked).toFixed(1) + "%" }; }
        if (worst > 0.6) { dips++; if (Math.max(...clip.slice(Math.max(0, at - 30), at + 1)) > 0.003) clipDips++; else if (!dipEx) dipEx = { ...inp, key, gap: gap.toFixed(0), dip: worst.toFixed(2) }; }
      }
      else if (gap > 150) { oppN++; if (worst > 0.6) oppDip++; }
    }
  }
  console.log(`R4 both blends: ${n} slider sweeps; largest change in any neutral as a slider leaves 0: ${firstMax.toFixed(2)} ΔE ${JSON.stringify(firstEx)};`,
    `hues within 90°: asked-for tint falls by over 1% in ${askedDips} (largest ${(100 * askedDipMax).toFixed(2)}%)${askedEx ? " " + JSON.stringify(askedEx) : ""}; visible tint falls below its running peak by over 0.6 ΔE in ${dips} (largest ${worstDip.toFixed(2)}), of which ${clipDips} at the sRGB edge${dipEx ? "; first other: " + JSON.stringify(dipEx) : ""};`,
    `hues over 150° apart: ${oppDip} of ${oppN} sweeps dip by over 0.6 ΔE (cancellation, expected)`);
}

// R4: opposite hues at equal blends cancel toward grey and the report says so.
{
  const pairs = [["Orange", "#e5652e", "Blue", "#2f6fed"], ["Pink", "#e5487f", "Green", "#2fa86a"], ["Amber", "#f0a92e", "Indigo", "#5b5bd6"], ["Teal", "#0fa3a3", "Pink", "#e5487f"], ["Yellow", "#ffd400", "Violet", "#8b5cf6"]];
  for (const scheme of ["dark", "light"]) for (const [n1, c1, n2, c2] of pairs) {
    const row = [0.25, 0.5, 1].map((b) => {
      const p = R({ scheme, base: c1, baseBlend: b, accent: c2, accentBlend: b, evenTint: true });
      const alone = Math.max(tintOf({ scheme, base: c1, baseBlend: b, accent: "mono", accentBlend: 0 }), tintOf({ scheme, base: "mono", baseBlend: 0, accent: c2, accentBlend: b }));
      const left = tintOf({ scheme, base: c1, baseBlend: b, accent: c2, accentBlend: b });
      return `${b}: ${T.hex(p.roles["layer.surface"])} tint ${left.toFixed(1)} of ${alone.toFixed(1)}${p.report.notes.some((x) => /cancel/.test(x)) ? " (note)" : ""}`;
    });
    console.log(`R4 opposite, ${scheme}, ${n1} + ${n2} (hue gap ${hueGap(c1, c2).toFixed(0)}°): ${row.join("; ")}`);
  }
}

// Round four 4: growth along base blend alone (accent mono) is unchanged; checked by the R1 sweep's
// mirror (largest gap above) and by tools/unchanged.js, which compares every role with round five.
