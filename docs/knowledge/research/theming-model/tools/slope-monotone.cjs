// R4 companion to invariant.js: along either slider, with the other blend above 0 and the hues within
// 90°, the tint slope the rule asks for never falls. Reports how far the surface chroma can still fall
// at the cap as the hue turns (a layer sits at fixed luminance, so its lightness depends on hue).
// Usage: node tools/slope-monotone.js
const T = require("./extract.cjs");
let seed = 99; const rnd = () => ((seed = (seed * 1103515245 + 12345) % 2147483648) / 2147483648);
const vivid = () => T.hex(T.lch(0.45 + rnd() * 0.45, 0.1 + rnd() * 0.2, rnd() * 360));
const gapOf = (a, b) => { const x = T.toLch(T.parseHex(a))[2], y = T.toLch(T.parseHex(b))[2]; return Math.abs(((x - y) % 360 + 540) % 360 - 180); };
let n = 0, slopeFalls = 0, worstSlope = 0, worstC = 0, ex = null, worstL = null;
for (const scheme of ["dark", "light"]) for (const even of [false, true]) for (let i = 0; i < 300; i++) {
  const c1 = vivid(), c2 = vivid(); if (gapOf(c1, c2) > 90) { i--; continue; }
  const other = Math.round((0.05 + rnd() * 0.95) * 100) / 100;
  for (const [key, inp] of [["accentBlend", { scheme, base: c1, baseBlend: other, accent: c2, evenTint: even }], ["baseBlend", { scheme, base: c1, accent: c2, accentBlend: other, evenTint: even }]]) {
    n++; let pS = 0, pC = 0, pL = 0;
    for (let j = 0; j <= 100; j++) {
      const t = T.resolveTheme({ ...inp, [key]: j / 100 }), L = T.toLch(t.roles["layer.surface"])[0];
      if (t.meta.slope < pS * (1 - 1e-12)) { slopeFalls++; worstSlope = Math.max(worstSlope, (pS - t.meta.slope) / pS); }
      if (pC > 0 && (pC - t.meta.chroma) / pC > worstC) { worstC = (pC - t.meta.chroma) / pC; ex = { ...inp, key, at: j / 100, slopeBefore: pS.toFixed(4), slopeNow: t.meta.slope.toFixed(4), hue: t.meta.hue.toFixed(0) }; }
      pS = Math.max(pS, t.meta.slope); pC = Math.max(pC, t.meta.chroma);
    }
  }
}
console.log(`${n} sweeps, hues within 90°: slope ever falls: ${slopeFalls} (worst ${(100 * worstSlope).toFixed(3)}%); surface chroma falls at most ${(100 * worstC).toFixed(2)}%`, JSON.stringify(ex));
