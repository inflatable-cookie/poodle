// Round five accent guarantees, over the full input space (scheme, tint mode, base, both blends,
// accent, contrast 0.5 to 2), using the sweep's colour generator.
//  G1  text on the fill is at least 4.5:1 at rest, hover and active; share at 7:1 reported
//  G2  hover and active never fall below min(rest, 7:1)
//  G3  accent line at least 3:1 on every layer; accent text at least 4.5:1 on every layer
//  G4  the fill stays within FILL_REACH (OKLab) of the picked colour; shares reported
//  G5  the fill is exactly the picked colour whenever the picked colour carries 7:1 with either ink
//  G6  a moved fill is lighter with dark ink or darker with white, and its text beats the picked colour's
//  G7  the edge is on exactly when the fill is within EDGE_NEAR of a layer, and is the line colour
//  G8  which swatches ever get an edge, per scheme (accent blend 0, then 0.5 and 1: round six lets the
//      accent tint the ground toward its own hue, which brings a muted fill closer to the layers)
//  G9  the danger button follows the accent fill's rule (round six ruling): text at least 4.5:1,
//      share at 7:1, and status.danger.edge on exactly when the fill is within EDGE_NEAR of a
//      layer, in the danger text colour
// Usage: node tools/accent.js [count, default 20000]
const T = require("./extract.cjs");
const N = +(process.argv[2] || 20000);
let seed = 23;
const rnd = () => ((seed = (seed * 1103515245 + 12345) % 2147483648) / 2147483648);
const randRgb = () => "#" + [0, 1, 2].map(() => Math.floor(rnd() * 256).toString(16).padStart(2, "0")).join("");
const grey = () => { const g = Math.floor(rnd() * 256).toString(16).padStart(2, "0"); return "#" + g + g + g; };
function randColour() { // the sweep's generator
  const k = rnd();
  if (k < 0.25) return randRgb();
  if (k < 0.5) return T.hex(T.lch(0.3 + rnd() * 0.65, 0.12 + rnd() * 0.25, rnd() * 360));
  if (k < 0.75) return T.hex(T.lch(rnd(), rnd() * 0.06, rnd() * 360));
  const e = rnd();
  return e < 0.4 ? "mono" : e < 0.8 ? grey() : e < 0.9 ? "#000000" : "#ffffff";
}
const unit = () => { const r = rnd(); return r < 0.15 ? 0 : r < 0.25 ? 1 : Math.round(rnd() * 100) / 100; };
const LAYERS = ["layer.canvas", "layer.surface", "layer.raised", "layer.overlay"];
const minC = (c, bgs) => Math.min(...bgs.map((b) => T.contrast(c, b)));
const W = T.rgb(1, 1, 1);

const fails = {}, ex = {};
const fail = (k, input, detail) => { fails[k] = (fails[k] || 0) + 1; if (!ex[k]) ex[k] = { input, detail }; };
const tally = { all: newTally(), uniform: newTally() };
const dangerT = { n: 0, strong: 0, minOn: Infinity, minEdge: Infinity, white: 0, edge: 0 };
function newTally() { return { n: 0, strong: 0, exact: 0, moved: 0, movedStrong: 0, within: { 0: 0, 0.02: 0, 0.05: 0, 0.08: 0, 0.1: 0, 0.12: 0 }, maxDist: 0, minOn: Infinity, edge: 0, lighter: 0, darker: 0 }; }

for (let i = 0; i < N; i++) {
  const uniform = rnd() < 0.3;
  const accent = uniform ? randRgb() : randColour();
  const input = { scheme: rnd() < 0.5 ? "light" : "dark", base: randColour(), baseBlend: unit(), accent, accentBlend: rnd() < 0.4 ? 0 : unit(),
    contrast: rnd() < 0.4 ? 1 : Math.round((0.5 + rnd() * 1.5) * 100) / 100, evenTint: rnd() < 0.5 };
  const t = T.resolveTheme(input), r = t.roles, m = t.meta.accent, layers = LAYERS.map((k) => r[k]);
  const on = r["accent.on"], fill = r["accent.fill"];
  const rest = T.contrast(on, fill), hov = T.contrast(on, r["accent.fill-hover"]), act = T.contrast(on, r["accent.fill-active"]);
  // G1
  if (Math.min(rest, hov, act) < T.ON_MIN) fail("G1 text on fill < 4.5", input, [rest, hov, act].map((x) => x.toFixed(2)).join("/"));
  // G2
  if (Math.min(hov, act) < Math.min(rest, T.ON_STRONG) - 1e-9) fail("G2 hover/active below min(rest, 7)", input, [rest, hov, act].map((x) => x.toFixed(2)).join("/"));
  // G3
  const lineMin = minC(r["accent.line"], layers), textMin = minC(r["accent.text"], layers);
  if (lineMin < 3) fail("G3 accent line < 3 on a layer", input, lineMin.toFixed(2));
  if (textMin < 4.5) fail("G3 accent text < 4.5 on a layer", input, textMin.toFixed(2));
  // The picked colour as the engine sees it (mono resolves to the neutral ink).
  const picked = t.picked;
  const dist = T.labDist(fill, picked);
  if (dist > T.FILL_REACH + 1e-9) fail("G4 fill beyond FILL_REACH", input, dist.toFixed(3));
  // G5: the best of the two inks on the picked colour (white, or the deep ink of its hue).
  const s = T.toLch(picked), dark = T.lch(0.18, Math.min(s[1] * 0.3, 0.04), s[2]);
  const best0 = Math.max(T.contrast(picked, W), T.contrast(picked, dark));
  if (best0 >= T.ON_STRONG + 0.05 && dist !== 0) fail("G5 fill moved though the picked colour carries 7:1", input, `${T.hex(picked)} → ${T.hex(fill)}`);
  // G6
  if (dist > 0) {
    const darkInk = on[0] < 0.5, dL = T.toLab(fill)[0] - T.toLab(picked)[0];
    if (darkInk ? dL <= 0 : dL >= 0) fail("G6 moved the wrong way for its ink", input, `${T.hex(picked)} → ${T.hex(fill)} ink ${T.hex(on)}`);
    if (rest + 1e-9 < best0) fail("G6 move lowered text contrast", input, `${best0.toFixed(2)} → ${rest.toFixed(2)}`);
  }
  // G7
  const edgeDist = Math.min(...layers.map((l) => T.flareDist(fill, l))), edgeOn = r["accent.edge"][3] > 0;
  if (edgeOn !== edgeDist < T.EDGE_NEAR) fail("G7 edge disagrees with the threshold", input, edgeDist.toFixed(3));
  if (edgeOn && T.hex(r["accent.edge"]) !== T.hex(r["accent.line"])) fail("G7 edge is not the line colour", input, T.hex(r["accent.edge"]));
  if (!edgeOn && r["accent.edge"][3] !== 0) fail("G7 edge off but visible", input, T.hex(r["accent.edge"]));
  // G9: the danger button.
  { const ds = r["status.danger.solid"], dn = r["status.danger.on"], dc = T.contrast(dn, ds), dd = Math.min(...layers.map((l) => T.flareDist(ds, l)));
    dangerT.n++; if (dc >= T.ON_STRONG) dangerT.strong++; dangerT.minOn = Math.min(dangerT.minOn, dc); dangerT.minEdge = Math.min(dangerT.minEdge, dd);
    if (dn[0] > 0.5) dangerT.white++;
    if (dc < T.ON_MIN) fail("G9 text on danger fill < 4.5", input, dc.toFixed(2));
    const de = r["status.danger.edge"];
    if ((de[3] > 0) !== dd < T.EDGE_NEAR) fail("G9 danger edge disagrees with the threshold", input, dd.toFixed(3));
    if (de[3] > 0) { dangerT.edge++; if (T.hex(de) !== T.hex(r["status.danger.fg"])) fail("G9 danger edge is not the danger text colour", input, T.hex(de)); }
    else if (de[3] !== 0) fail("G9 danger edge off but visible", input, T.hex(de)); }
  for (const k of uniform ? ["all", "uniform"] : ["all"]) {
    if (accent === "mono" && k === "all") continue; // shares are over picked colours, not the neutral ink
    const y = tally[k];
    y.n++; if (m.strong) y.strong++; if (dist === 0) y.exact++; else { y.moved++; if (m.strong) y.movedStrong++; if (on[0] < 0.5) y.lighter++; else y.darker++; }
    for (const d in y.within) if (dist <= +d + 1e-9) y.within[d]++;
    y.maxDist = Math.max(y.maxDist, dist); y.minOn = Math.min(y.minOn, rest); if (edgeOn) y.edge++;
  }
}
console.log(`cases ${N} (contrast 0.5 to 2 or 1; both schemes and tint modes; random bases and blends)`);
console.log("failures", Object.keys(fails).length ? fails : "none");
if (Object.keys(ex).length) console.log("first failing input per guarantee", JSON.stringify(ex, null, 1));
const pc = (x, n) => (100 * x / n).toFixed(1) + "%";
for (const [k, y] of Object.entries(tally)) {
  console.log(`\n${k === "all" ? "Accents from the sweep generator (mono excluded)" : "Uniform-RGB accents (what a free picker reaches)"}: ${y.n}`);
  console.log(`  text on fill at 7:1: ${pc(y.strong, y.n)}; lowest ${y.minOn.toFixed(2)}`);
  console.log(`  fill exactly as picked: ${pc(y.exact, y.n)}; moved ${pc(y.moved, y.n)} (lighter with dark text ${y.lighter}, darker with white ${y.darker}); moved and reaching 7:1: ${pc(y.movedStrong, y.moved)}`);
  console.log(`  fill within ΔE of picked (OKLab × 100): ` + Object.entries(y.within).map(([d, v]) => `${Math.round(d * 100)}: ${pc(v, y.n)}`).join(", ") + `; largest ${(y.maxDist * 100).toFixed(1)}`);
  console.log(`  edge on: ${pc(y.edge, y.n)}`);
}

// G8: which swatches ever get an edge. Every swatch × scheme × tint mode × 11 base blends × 4
// bases (mono, a vivid, a muted, a pale) × 3 contrasts.
const Presets = T.Presets;
const swatches = Presets.SWATCHES.flat().filter(Boolean).filter((s) => s[1] !== "mono").concat([["White", "#ffffff"], ["Black", "#000000"], ["Pale yellow", "#fff3b0"], ["Pale blue", "#d6ecff"], ["Navy", "#1a1a40"], ["Mono", "mono"]]);
const bases = ["mono", "#e5487f", "#6379a0", "#e4d8ac"];
for (const ab of [0, 0.5, 1]) {
  const edgeBy = {};
  for (const [name, hex] of swatches) for (const scheme of ["light", "dark"]) {
    let on = 0, n = 0;
    for (const even of [false, true]) for (let b = 0; b <= 10; b++) for (const base of bases) for (const contrast of [0.5, 1, 2]) {
      const t = T.resolveTheme({ scheme, base, baseBlend: b / 10, accent: hex, accentBlend: ab, contrast, evenTint: even });
      n++; if (t.meta.accent.edge) on++;
    }
    if (on) (edgeBy[name] = edgeBy[name] || []).push(`${scheme} ${pc(on, n)}`);
  }
  console.log(`\nG8 accent blend ${ab}: swatches that ever get an edge (share of base, base blend, contrast and tint-mode combinations):`);
  console.log(Object.keys(edgeBy).length ? Object.entries(edgeBy).map(([k, v]) => `  ${k}: ${v.join(", ")}`).join("\n") : "  none");
  console.log("  every other swatch: never");
}
console.log(`\nG9 danger button over ${dangerT.n} themes: text at 7:1 in ${pc(dangerT.strong, dangerT.n)}, lowest ${dangerT.minOn.toFixed(2)}; white text in ${pc(dangerT.white, dangerT.n)}; nearest any layer comes, ΔE ${(dangerT.minEdge * 100).toFixed(1)}; edge on in ${dangerT.edge} (${pc(dangerT.edge, dangerT.n)}; threshold ΔE ${Math.round(T.EDGE_NEAR * 100)})`);
