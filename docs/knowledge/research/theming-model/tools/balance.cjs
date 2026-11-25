// Scheme balance: for each swatch, the surface's tint (OKLab ΔE from the mono theme at the same
// blend) in light divided by the same in dark, plain and even tint. 1 means equal in OKLab terms.
// Usage: node tools/balance.js
const T = require("./extract.cjs");
const lab = (c) => T.toLab(c);
const dE = (x, y) => { const a = lab(x), b = lab(y); return 100 * Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]); };
const tint = (scheme, base, b, even) => { const i = { scheme, base, baseBlend: b, accent: "mono", evenTint: even }; return dE(T.resolveTheme(i).roles["layer.surface"], T.resolveTheme({ ...i, base: "mono" }).roles["layer.surface"]); };
const sw = T.Presets.SWATCHES.flat().filter((s) => s && s[1] !== "mono");
const med = (a) => { const s = [...a].sort((x, y) => x - y); return s[Math.floor(s.length / 2)]; };
for (const b of [0.25, 0.5, 1]) for (const even of [false, true]) {
  const r = sw.map(([n, h]) => ({ n, r: tint("light", h, b, even) / tint("dark", h, b, even) }));
  const lo = r.reduce((a, x) => (x.r < a.r ? x : a)), hi = r.reduce((a, x) => (x.r > a.r ? x : a));
  console.log(`blend ${b} ${even ? "even " : "plain"}: light/dark tint median ${med(r.map((x) => x.r)).toFixed(2)}, range ${lo.r.toFixed(2)} (${lo.n}) to ${hi.r.toFixed(2)} (${hi.n})`);
}
