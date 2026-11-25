// Fidelity: derived roles against canonical Nord and Solarized, and every preset against today's
// Poodle theme. ΔE is the OKLab distance × 100 (about 2 is the smallest difference people notice
// side by side). Prints Markdown. Usage: node fidelity.js [--even] [--today] [reference ids]
const T = require("./extract.cjs");
const P = T.Presets;
const ROLES = ["layer.canvas", "layer.surface", "layer.raised", "layer.overlay", "text.primary", "text.secondary", "text.disabled", "border.strong", "accent.fill", "accent.text",
  "status.success.fg", "status.warning.fg", "status.danger.fg", "status.info.fg"].concat(T.SYNTAX.map((s) => "syntax." + s));
const lab = (c) => T.toLab(c);
const dE = (a, b) => { const x = lab(a), y = lab(b); return 100 * Math.hypot(x[0] - y[0], x[1] - y[1], x[2] - y[2]); };
const lchDelta = (a, b) => { const x = T.toLch(a), y = T.toLch(b); let dh = Math.abs(x[2] - y[2]); if (dh > 180) dh = 360 - dh; return { dL: x[0] - y[0], dC: x[1] - y[1], dh: Math.min(x[1], y[1]) < 0.02 ? null : dh }; };
const group = (r) => r.startsWith("layer") ? "layers" : r.startsWith("text") || r === "border.strong" ? "text" : r.startsWith("accent") ? "accent" : r.startsWith("status") ? "status" : "syntax";
const EVEN = process.argv.includes("--even");
const LIST = EVEN ? P.PRESETS_EVEN : P.PRESETS;
const resolve = (id) => { const p = LIST.find((x) => x.id === id); return T.resolveTheme(p); };
const fmt = (v) => v.toFixed(1);
const short = (r) => r.replace("layer.", "").replace(".fg", "");

function canonical(id) {
  const ref = P.CANONICAL[id], t = resolve(id), pal = ref.palette.map(([n, h]) => ({ n, h, c: T.parseHex(h) }));
  console.log(`\n### ${ref.title} (preset \`${id}\`, ${EVEN ? "even" : "plain"} tint)\n`);
  console.log("| Role | Derived | Intended | ΔE | ΔL · ΔC · Δh | Nearest | ΔE |\n|---|---|---|---:|---|---|---:|");
  const sums = {};
  ROLES.forEach((r) => {
    const c = t.roles[r];
    const near = pal.map((p) => ({ ...p, d: dE(c, p.c) })).sort((a, b) => a.d - b.d)[0];
    const iname = ref.map[r], ip = pal.find((p) => p.n === iname);
    const di = ip ? dE(c, ip.c) : null;
    const ld = ip ? lchDelta(c, ip.c) : null;
    const g = group(r); sums[g] = sums[g] || { i: [], n: [] }; if (di !== null) sums[g].i.push(di); sums[g].n.push(near.d);
    console.log(`| ${short(r)} | \`${T.hex(c)}\` | ${ip ? iname + " `" + ip.h + "`" : "none"} | ${di === null ? "" : fmt(di)} | ${ld ? `${ld.dL >= 0 ? "+" : ""}${ld.dL.toFixed(2)} · ${ld.dC >= 0 ? "+" : ""}${ld.dC.toFixed(3)} · ${ld.dh === null ? "n/a" : Math.round(ld.dh) + "°"}` : ""} | ${near.n} | ${fmt(near.d)} |`);
  });
  console.log("\n| Group | Mean ΔE to intended | Mean ΔE to nearest |\n|---|---:|---:|");
  Object.entries(sums).forEach(([g, s]) => { const m = (a) => a.length ? fmt(a.reduce((x, y) => x + y, 0) / a.length) : "n/a"; console.log(`| ${g} | ${m(s.i)} | ${m(s.n)} |`); });
  console.log("\nCanonical colours, nearest derived role:\n\n| Canonical | Nearest role | ΔE |\n|---|---|---:|");
  pal.forEach((p) => {
    const best = ROLES.map((r) => ({ r, d: dE(t.roles[r], p.c) })).sort((a, b) => a.d - b.d)[0];
    console.log(`| ${p.n} \`${p.h}\` | ${short(best.r)} \`${T.hex(t.roles[best.r])}\` | ${fmt(best.d)}${best.d > 10 ? " (lost)" : best.d > 5 ? " (shifted)" : ""} |`);
  });
}

function today() {
  console.log("\n### Presets against today's Poodle themes (mean ΔE by group, by role)\n");
  console.log("| Preset | Layers (max) | Text | Accent fill | Status | Syntax |\n|---|---:|---:|---:|---:|---:|");
  Object.keys(P.TODAY).forEach((id) => {
    const t = resolve(id), ref = P.TODAY[id], g = {};
    Object.keys(ref).forEach((r) => { const d = dE(t.roles[r], T.parseHex(ref[r])); const k = group(r); (g[k] = g[k] || []).push(d); });
    const m = (a) => fmt(a.reduce((x, y) => x + y, 0) / a.length);
    console.log(`| ${id} | ${m(g.layers)} (${fmt(Math.max(...g.layers))}) | ${m(g.text)} | ${m(g.accent)} | ${m(g.status)} | ${m(g.syntax)} |`);
  });
}

const only = process.argv.filter((a) => P.CANONICAL[a]);
if (process.argv.includes("--today")) today();
else { (only.length ? only : ["nord", "solarized", "solarized-light"]).forEach(canonical); if (!only.length) today(); }
