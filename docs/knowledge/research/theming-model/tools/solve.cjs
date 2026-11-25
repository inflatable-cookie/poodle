// Solve (base colour, base blend) so each preset's surface lands on its round-two surface, for
// the shipped rule, in plain and even tint. Base blend is pinned by the surface's lightness (it
// sets depth), so the blend is searched near that value; the base is built on the line the rule
// needs at that blend (slope = target slope / w(blend), read from white or black), at OKLab L 0.6
// or the nearest lightness inside sRGB, and tried with its 26 neighbours. When no base hits, the
// closest surface is found over blend, slope and a few degrees of hue.
// Usage: node tools/solve.js            (prints both modes and the preset table)
const T = require("./extract.cjs");
const lab = (c) => T.toLab(typeof c === "string" ? T.parseHex(c) : c);
const dE = (x, y) => { const a = lab(x), b = lab(y); return 100 * Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]); };

// opts: { E: engine, weight: (b) => number, ground: optional (bb, scheme, contrast) => distance of
// the layer whose distance sets the chroma (rule A), defaults to the surface's own line }
function solvePreset({ scheme, target, contrast }, even, opts = {}) {
  const E = opts.E || T, weight = opts.weight || T.tintWeight;
  const [L, a, b] = lab(target), d = scheme === "dark" ? L : 1 - L, Ct = Math.hypot(a, b), h0 = Math.atan2(b, a);
  const sStar = Ct < 2e-3 ? 0 : Ct / d;
  const rg = T.depthRange(scheme, 0.024 * contrast), bstar = (T.lf(T.lum(T.parseHex(target))) - rg[0]) / (rg[1] - rg[0]);
  const from = even ? "dark" : scheme, ratio = T.slopeMax(scheme) / T.slopeMax(from);
  const surf = (base, bb) => T.hex(E.resolveTheme({ scheme, base, baseBlend: bb, accent: "mono", contrast, evenTint: even }).roles["layer.surface"]);
  // the base on the line with slope `need` (in its own frame) and hue h, nearest L 0.6 inside sRGB
  const baseOn = (need, h) => {
    const at = (LB) => { const dB = from === "dark" ? LB : 1 - LB, lin = T.labToLin(LB, need * dB * Math.cos(h), need * dB * Math.sin(h)); return T.inGamut(lin) ? T.rgb(...lin.map(T.toGam)) : null; };
    for (let i = 0; i <= 45; i++) for (const x of [0.6 + i * 0.01, 0.6 - i * 0.01]) if (x > 0.02 && x < 0.99 && at(x)) return at(x);
    return null;
  };
  const order = [];
  for (let i = 0; i <= 100; i++) for (const sg of [1, -1]) { const bb = Math.round((bstar + sg * i / 100) * 100) / 100; if (bb >= 0 && bb <= 1 && !order.includes(bb)) order.push(bb); }
  const needAt = (bb) => {
    const wv = weight(bb); if (wv <= 0) return null;
    let need = sStar / ratio;
    if (opts.ground) { const dg = opts.ground(bb, scheme, contrast); if (dg <= 1e-6) return null; need = Ct / dg / ratio; }
    return need / wv;
  };
  for (const bb of order.slice(0, 15)) {
    if (sStar === 0) { if (surf("mono", bb) === target) return { base: "mono", bb, got: target, de: 0, exact: true }; continue; }
    const need = needAt(bb);
    if (need === null || need * ratio > T.slopeMax(scheme) * 1.0001) continue; // past the cap
    const ideal = baseOn(need, h0);
    if (!ideal) continue; // past the edge of sRGB at this hue
    const c = ideal.map((v) => Math.round(v * 255)), cands = [];
    for (let dr = -1; dr <= 1; dr++) for (let dg = -1; dg <= 1; dg++) for (let db = -1; db <= 1; db++) cands.push("#" + [c[0] + dr, c[1] + dg, c[2] + db].map((v) => Math.max(0, Math.min(255, v)).toString(16).padStart(2, "0")).join(""));
    cands.sort((x, y) => dE(x, ideal) - dE(y, ideal));
    for (const base of cands) if (surf(base, bb) === target) return { base, bb, got: target, de: 0, exact: true };
  }
  // Not exact: closest surface. Slope up to the smaller of the cap and what the target asks for
  // (and a little beyond), hue within ±8°, blends within 0.25 of the depth-implied blend.
  let best = null;
  const smaxFrom = T.slopeMax(scheme) / ratio;
  for (const bb of order.slice(0, 51)) {
    for (let dh = -8; dh <= 8; dh += 2) for (let f = 0.02; f <= 1.0001; f += 0.02) {
      const ideal = baseOn(smaxFrom * f, h0 + dh * Math.PI / 180);
      if (!ideal) continue;
      const base = T.hex(ideal), got = surf(base, bb), de = dE(got, target);
      if (!best || de < best.de - 1e-9) best = { base, bb, got, de, exact: false };
    }
  }
  return best;
}

module.exports = { solvePreset, dE };

if (require.main === module) {
  const P = T.Presets;
  const specs = P.PRESETS.map((p) => ({ id: p.id, scheme: p.scheme, target: P.ROUND2_SURFACE[p.id], contrast: p.contrast, accent: p.accent }));
  for (const even of [false, true]) {
    console.log(`\n── ${even ? "even" : "plain"} tint`);
    for (const s of specs) {
      if (even && s.scheme === "dark") { console.log(s.id.padEnd(16), "dark: same as plain"); continue; }
      const r = solvePreset(s, even);
      console.log(s.id.padEnd(16), s.scheme.padEnd(5), s.target, "→ base", r.base, "baseBlend", r.bb, "surface", r.got, r.exact ? "exact" : "ΔE " + r.de.toFixed(2));
    }
  }
}
