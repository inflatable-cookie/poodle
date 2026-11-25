// Compare candidate rules for Fix 1 (base blend owns the whole base effect). Each candidate is the
// shipped engine with its tint step rewritten, so everything else is identical.
//   r3      round three: tint fixed per lightness, blend sets depth only (the defect)
//   A       the ground (the layer at black or white when blend is 0) sits on the base's line; the
//           other layers change lightness only and carry the ground's chroma
//   B-lin   every layer's slope is scaled by w(b) = b
//   B-quad  every layer's slope is scaled by w(b) = b(2 - b)   (the shipped rule)
// Usage: node tools/rules.js
const fs = require("fs");
const path = require("path");
const html = fs.readFileSync(path.join(__dirname, "..", "index.html"), "utf8");
const src = html.match(/<script id="resolve-theme">([\s\S]*?)<\/script>/)[1];
const presetsSrc = html.match(/<script id="presets">([\s\S]*?)<\/script>/)[1];
const P = new Function(presetsSrc + "\nreturn PoodlePresets;")();
const swap = (s, a, b) => { if (!s.includes(a)) throw new Error("pattern not found: " + a); return s.replace(a, b); };
const build = (s) => new Function(s + "\nreturn PoodleTheme;")();
const W = "function tintWeight(b) { return b * (2 - b); }";
const VARIANTS = {
  r3: build(swap(src, W, "function tintWeight(b) { return 1; }")),
  A: build([
    [W, "function tintWeight(b) { return 1; }"],
    ["var lumAt = function (L) { var v = lchLin(L, slope * dist(L), h);",
      "var anchorDist = dist(M.cbrt(clamp(lfToY(surfaceLf + (scheme === \"dark\" ? -s : s)), 0, 1))), CA = slope * anchorDist;\n    var lumAt = function (L) { var v = lchLin(L, CA, h);"],
    ["C = slope * dist(L), c = lch(L, C, h);", "C = CA, c = lch(L, C, h);"],
    ["var C = slope * dist(ramp[1].L), tv", "var C = CA, tv"],
  ].reduce((s, [a, b]) => swap(s, a, b), src)),
  "B-lin": build(swap(src, W, "function tintWeight(b) { return b; }")),
  "B-quad": build(src),
};
const T = VARIANTS["B-quad"];
const lab = (c) => T.toLab(typeof c === "string" ? T.parseHex(c) : c);
const dE = (x, y) => { const a = lab(x), b = lab(y); return 100 * Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]); };
const C = (c) => T.toLch(c)[1];

console.log("── Owner's case: dark, base #e5487f, base blend 0, accent #1668d6");
for (const [name, E] of Object.entries(VARIANTS)) {
  const r = E.resolveTheme({ scheme: "dark", base: "#e5487f", baseBlend: 0, accent: "#1668d6", evenTint: false }).roles;
  console.log(name.padEnd(7), ["canvas", "surface", "raised", "overlay"].map((k) => T.hex(r["layer." + k])).join(" "), "fill.rest", T.hex(r["fill.rest"]), "text.secondary", T.hex(r["text.secondary"]));
}

console.log("\n── Onset and evenness: surface chroma along base blend (pink #e5487f, accent mono, plain)");
const BS = [0, 0.01, 0.05, 0.1, 0.25, 0.5, 0.75, 1];
for (const scheme of ["dark", "light"]) {
  for (const [name, E] of Object.entries(VARIANTS)) {
    const at = (b, k = "layer.surface") => E.resolveTheme({ scheme, base: "#e5487f", baseBlend: b, accent: "mono", evenTint: false }).roles[k];
    const cs = BS.map((b) => C(at(b)));
    // step evenness: chroma gained per 0.05 of slider, largest over smallest (ignoring the last step)
    const steps = []; for (let i = 0; i < 20; i++) steps.push(C(at((i + 1) / 20)) - C(at(i / 20)));
    const pos = steps.filter((x) => x > 1e-4);
    const tintAt = (b) => dE(at(b), VARIANTS.r3.resolveTheme({ scheme, base: "mono", baseBlend: b, accent: "mono" }).roles["layer.surface"]);
    console.log(`${scheme.padEnd(5)} ${name.padEnd(7)} C at ${BS.join("/")}: ${cs.map((c) => c.toFixed(3)).join(" ")} | tint ΔE vs mono at 0.01: ${tintAt(0.01).toFixed(2)}, at 0.1: ${tintAt(0.1).toFixed(2)} | 0.05-step gain max/min ${(Math.max(...pos) / Math.min(...pos)).toFixed(1)}, first step ${steps[0].toFixed(4)}, monotone ${steps.every((x) => x >= -1e-4)}`);
  }
}

// Exactness: the shared solver (tools/solve.js) run with each candidate's engine and weight.
const { solvePreset } = require("./solve.cjs");
const OPTS = {
  r3: { E: VARIANTS.r3, weight: () => 1 },
  A: { E: VARIANTS.A, weight: () => 1, ground: (bb, scheme, contrast) => { const s = 0.024 * contrast, rg = T.depthRange(scheme, s), lfS = rg[0] + (rg[1] - rg[0]) * bb, Lg = Math.cbrt(Math.min(1, Math.max(0, T.lfToY(lfS + (scheme === "dark" ? -s : s))))); return scheme === "dark" ? Lg : 1 - Lg; } },
  "B-lin": { E: VARIANTS["B-lin"], weight: (b) => b },
  "B-quad": { E: VARIANTS["B-quad"], weight: (b) => b * (2 - b) },
};
const solve = (name, id, scheme, target, accent, contrast) => solvePreset({ scheme, target, contrast }, false, OPTS[name]);
const ROUND2 = P.PRESETS.map((p) => [p.id, p.scheme, P.ROUND2_SURFACE[p.id], p.accent, p.contrast]);
console.log("\n── Preset exactness (plain)");
for (const name of ["r3", "A", "B-lin", "B-quad"]) {
  const res = ROUND2.map(([id, scheme, target, accent, contrast]) => ({ id, ...solve(name, id, scheme, target, accent, contrast) }));
  const miss = res.filter((r) => !r.exact);
  console.log(name.padEnd(7), `${res.length - miss.length}/13 exact`, miss.map((r) => `${r.id} ΔE ${r.de.toFixed(2)} (${r.base} @ ${r.bb} → ${r.got})`).join("; "));
}

console.log("\n── Nord layers against nord1 and nord2, each rule's own Nord preset");
for (const name of ["r3", "A", "B-quad"]) {
  const s = solve(name, "nord", "dark", "#2e3440", "#88c0d0", 1.35);
  const r = VARIANTS[name].resolveTheme({ scheme: "dark", base: s.base, baseBlend: s.bb, accent: "#88c0d0", contrast: 1.35, evenTint: false }).roles;
  console.log(name.padEnd(7), "surface", T.hex(r["layer.surface"]), "raised", T.hex(r["layer.raised"]), `ΔE ${dE(r["layer.raised"], "#3b4252").toFixed(1)} ΔC ${(C(r["layer.raised"]) - C(T.parseHex("#3b4252"))).toFixed(3)}`, "overlay", T.hex(r["layer.overlay"]), `ΔE ${dE(r["layer.overlay"], "#434c5e").toFixed(1)} ΔC ${(C(r["layer.overlay"]) - C(T.parseHex("#434c5e"))).toFixed(3)}`);
}
module.exports = { VARIANTS };
