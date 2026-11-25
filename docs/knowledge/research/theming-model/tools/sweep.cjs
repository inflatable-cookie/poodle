// Random sweep over the whole input space: scheme × base × base blend × accent × accent blend (× contrast).
// Reports guarantee failures, warnings, notes and how close passes sit to their targets.
// Usage: node sweep.js [count] [--contrast]   (--contrast also randomises contrast in 0.9..2)
const T = require("./extract.cjs");
const N = +(process.argv[2] || 6000);
const randomContrast = process.argv.includes("--contrast");
let seed = 11;
const rnd = () => ((seed = (seed * 1103515245 + 12345) % 2147483648) / 2147483648);
const randRgb = () => "#" + [0, 1, 2].map(() => Math.floor(rnd() * 256).toString(16).padStart(2, "0")).join("");
const grey = () => { const g = Math.floor(rnd() * 256).toString(16).padStart(2, "0"); return "#" + g + g + g; };
// Bases and accents come from the same picker, so they share one generator: a quarter uniform RGB
// (what a free picker reaches), a quarter vivid (OKLCH L 0.3–0.95, C 0.12–0.37, gamut-clipped),
// a quarter muted at any lightness, and the rest greys, mono, black and white.
function randColour() {
  const k = rnd();
  if (k < 0.25) return randRgb();
  if (k < 0.5) return T.hex(T.lch(0.3 + rnd() * 0.65, 0.12 + rnd() * 0.25, rnd() * 360));
  if (k < 0.75) return T.hex(T.lch(rnd(), rnd() * 0.06, rnd() * 360));
  const e = rnd();
  return e < 0.4 ? "mono" : e < 0.8 ? grey() : e < 0.9 ? "#000000" : "#ffffff";
}
const unit = () => { const r = rnd(); return r < 0.15 ? 0 : r < 0.25 ? 1 : Math.round(rnd() * 100) / 100; };
const fails = {}, warns = {}, notes = {}, clamps = {};
let minMargin = Infinity, cases = 0, tight = 0, worstHier = Infinity, ms = 0, maxChroma = 0;
const examples = {};
for (let i = 0; i < N; i++) {
  const scheme = rnd() < 0.5 ? "light" : "dark";
  const input = { scheme, base: randColour(), baseBlend: unit(), accent: randColour(), accentBlend: rnd() < 0.4 ? 0 : unit(),
    contrast: randomContrast ? Math.round((0.9 + rnd() * 1.1) * 100) / 100 : 1, evenTint: rnd() < 0.5 };
  const t0 = performance.now();
  const t = T.resolveTheme(input);
  ms += performance.now() - t0;
  const a = T.audit(t);
  cases++;
  maxChroma = Math.max(maxChroma, ...["canvas", "surface", "raised", "overlay"].map((k) => T.toLch(t.roles["layer." + k])[1]));
  const note = (k) => { k = `${input.evenTint ? "even" : "plain"} ${k}`; fails[k] = (fails[k] || 0) + 1; if (!examples[k]) examples[k] = input; };
  a.matrix.forEach((m) => m.values.forEach((v, j) => { if (v < m.target[j]) note(`${scheme}: ${m.role} layer ${j}`); else { minMargin = Math.min(minMargin, v - m.target[j]); if (v - m.target[j] < 0.02) tight++; } }));
  a.checks.forEach((c) => { const label = c.label.replace(/ \(lowest.*\)/, ""); if (!c.pass) note(`${scheme}: ${label}`); else { minMargin = Math.min(minMargin, c.value - c.target); if (c.value - c.target < 0.02) tight++; } });
  a.steps.forEach((s) => { if (!s.pass) note(`${scheme}: step ${s.label}`); });
  if (!a.hierarchy.pass) note(`${scheme}: text hierarchy`);
  worstHier = Math.min(worstHier, a.hierarchy.value);
  t.report.warnings.forEach((w) => { warns[`${scheme}: ${w}`] = (warns[`${scheme}: ${w}`] || 0) + 1; });
  t.report.notes.forEach((w) => { notes[w] = (notes[w] || 0) + 1; });
  t.report.clamped.forEach((c) => { const k = `${c.input}: ${c.why}`; clamps[k] = (clamps[k] || 0) + 1; });
}
console.log(`cases ${cases}${randomContrast ? " (contrast 0.9..2)" : " (contrast 1)"}, mean ${(ms / cases).toFixed(2)} ms per resolve`);
console.log("failures", Object.keys(fails).length ? fails : "none");
if (Object.keys(examples).length) console.log("first failing input per check", examples);
console.log("warnings", Object.keys(warns).length ? warns : "none");
console.log("notes", notes);
console.log("clamps", clamps);
console.log(`smallest pass margin ${minMargin.toFixed(4)}; passes within 0.02 of target: ${tight}; smallest text hierarchy gap ${worstHier.toFixed(3)}; most chroma on any layer ${maxChroma.toFixed(3)}`);
