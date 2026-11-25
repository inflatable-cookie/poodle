// Spot checks: every preset (and its exact surface), the awkward inputs, and a totality fuzz.
// Usage: node test-engine.js [-v]
const T = require("./extract.cjs");
const verbose = process.argv.includes("-v");
const failsOf = (t) => {
  const a = T.audit(t), f = [];
  a.matrix.forEach((m) => m.values.forEach((v, i) => { if (v < m.target[i]) f.push(`${m.role}@${i}=${v.toFixed(2)}`); }));
  a.checks.forEach((x) => { if (!x.pass) f.push(`${x.label}=${x.value.toFixed(2)}`); });
  a.steps.forEach((s) => { if (!s.pass) f.push(`step ${s.label} dL=${s.dL.toFixed(3)}`); });
  if (!a.hierarchy.pass) f.push(`hierarchy ${a.hierarchy.value.toFixed(3)}`);
  return f;
};
const show = (label, input, expectSurface) => {
  const t = T.resolveTheme(input), h = (k) => T.hex(t.roles[k]), f = failsOf(t), r = t.report;
  const exact = expectSurface ? (h("layer.surface") === expectSurface ? "surface exact" : `surface MISS (want ${expectSurface})`) : "";
  console.log(label.padEnd(26), "fails:", f.length ? f.join("; ") : "none", exact);
  console.log("  layers", ["canvas", "surface", "raised", "overlay"].map((k) => h("layer." + k)).join(" "), `C ${t.meta.chroma.toFixed(3)}`, "| text", h("text.primary"), h("text.secondary"), h("text.disabled"), "| accent", h("accent.fill"), "on", h("accent.on"), t.meta.accent.onContrast.toFixed(1) + ":1", "line", h("accent.line"), "text", h("accent.text"), t.meta.accent.edge ? "EDGE" : "");
  if (verbose) console.log("  status", T.STATUS.map((s) => h(`status.${s}.fg`)).join(" "), "| danger solid", h("status.danger.solid"), "| syntax", T.SYNTAX.map((s) => h("syntax." + s)).join(" "));
  const rep = [...r.clamped.map((c) => `clamped ${c.input} ${c.from} → ${c.to} (${c.why})`), ...r.adjusted.filter((x) => x.why).map((x) => `${x.role} ${x.why}`), ...r.warnings, ...r.notes];
  if (rep.length) console.log("  report:", rep.join("; "));
};
console.log("── Presets, plain tint");
T.Presets.PRESETS.forEach((p) => show(p.id, p, T.Presets.ROUND2_SURFACE[p.id]));
console.log("\n── Presets, even tint (dark presets are the same as plain)");
T.Presets.PRESETS_EVEN.filter((p) => p.scheme === "light").forEach((p) => show(p.id, p, T.Presets.ROUND2_SURFACE[p.id]));
console.log("\n── Awkward inputs");
const AWKWARD = {
  "vivid red, dark, blend 1": { scheme: "dark", base: "#ff0000", baseBlend: 1, accent: "#ffd400" },
  "vivid blue, dark, blend 1": { scheme: "dark", base: "#0000ff", baseBlend: 1, accent: "#f0a92e" },
  "yellow, light, blend 1": { scheme: "light", base: "#ffd400", baseBlend: 1, accent: "#2f6fed" },
  "pure yellow, light, blend 1": { scheme: "light", base: "#ffff00", baseBlend: 1, accent: "#2f6fed" },
  "pink, light, blend .8": { scheme: "light", base: "#ff416c", baseBlend: 0.8, accent: "#2f6fed" },
  "same hue, dark": { scheme: "dark", base: "#2f6fed", baseBlend: 0.5, accent: "#2f6fed" },
  "same hue, light": { scheme: "light", base: "#2f6fed", baseBlend: 0.5, accent: "#2f6fed" },
  "same hue, accent blend .5": { scheme: "dark", base: "#2f6fed", baseBlend: 0.5, accent: "#2f6fed", accentBlend: 0.5 },
  "opposite hues, dark": { scheme: "dark", base: "#e5652e", baseBlend: 0.5, accent: "#2f6fed" },
  "opposite, accent blend .5": { scheme: "dark", base: "#e5652e", baseBlend: 0.5, accent: "#2f6fed", accentBlend: 0.5 },
  "opposite, light, ab .5": { scheme: "light", base: "#e5652e", baseBlend: 0.5, accent: "#2f6fed", accentBlend: 0.5 },
  "red base, red accent": { scheme: "dark", base: "#e5487f", baseBlend: 0.75, accent: "#e5487f" },
  "grey #808080, dark": { scheme: "dark", base: "#808080", baseBlend: 0.5, accent: "#2f6fed" },
  "mono, dark": { scheme: "dark", base: "mono", baseBlend: 0.5, accent: "#2f6fed" },
  "black base, light": { scheme: "light", base: "#000000", baseBlend: 0.5, accent: "#2f6fed" },
  "white base, dark": { scheme: "dark", base: "#ffffff", baseBlend: 0.5, accent: "#2f6fed" },
  "Nord base, light, plain": { scheme: "light", base: "#73809a", baseBlend: 0.98, accent: "#88c0d0", evenTint: false },
  "Nord base, light, even": { scheme: "light", base: "#73809a", baseBlend: 0.98, accent: "#88c0d0", evenTint: true },
  "Eclipse base, light, plain": { scheme: "light", base: "#6384a3", baseBlend: 0.2, accent: "#f0b24d", evenTint: false },
  "Eclipse base, light, even": { scheme: "light", base: "#6384a3", baseBlend: 0.2, accent: "#f0b24d", evenTint: true },
  "Clay base, dark": { scheme: "dark", base: "#c89514", baseBlend: 0.45, accent: "#c96442" },
  "Clay even base, dark": { scheme: "dark", base: "#967c4c", baseBlend: 0.45, accent: "#c96442" },
  "pink, dark, blend 0": { scheme: "dark", base: "#e5487f", baseBlend: 0, accent: "#1668d6" },
  "pink, light, blend 0": { scheme: "light", base: "#e5487f", baseBlend: 0, accent: "#1668d6" },
  "mono base, accent blend 1, base blend 0": { scheme: "dark", base: "mono", baseBlend: 0, accent: "#e5487f", accentBlend: 1 },
  "garbage": { scheme: "blue", base: "nope", baseBlend: 7, accent: 42, accentBlend: -3, contrast: -1, evenTint: "yes" },
};
Object.entries(AWKWARD).forEach(([k, v]) => show(k, v));
console.log("\n── Totality fuzz");
let seed = 3;
const rnd = () => ((seed = (seed * 1103515245 + 12345) % 2147483648) / 2147483648);
const junk = () => [undefined, null, NaN, Infinity, -1, 0.5, 3, "", "dark", "light", "#", "#12", "#abc", "#zzzzzz", "#12345678", "red", [1, 2], [0.5, 0.5, 0.5], [NaN, 0, 0], {}, true, false, "mono"][Math.floor(rnd() * 23)];
let thrown = 0, bad = 0;
for (let i = 0; i < 5000; i++) {
  const input = rnd() < 0.1 ? junk() : { scheme: junk(), base: rnd() < 0.5 ? junk() : "#" + Math.floor(rnd() * 16777216).toString(16).padStart(6, "0"), baseBlend: junk(), accent: junk(), accentBlend: junk(), contrast: junk(), evenTint: rnd() < 0.5 ? junk() : rnd() < 0.5 };
  try {
    const t = T.resolveTheme(input);
    const ok = Object.values(t.roles).every((c) => Array.isArray(c) && c.length === 4 && c.every((x) => Number.isFinite(x) && x >= 0 && x <= 1));
    if (!ok || Object.keys(t.roles).length !== 44) bad++;
  } catch (e) { thrown++; if (thrown < 3) console.log("threw on", input, e.message); }
}
console.log(`5000 garbage inputs: ${thrown} threw, ${bad} returned an invalid role set`);
