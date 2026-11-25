// Round six changes the tint only when accent blend is above 0, plus the danger button (operator
// ruling: it follows the accent fill's rule, including the edge, which adds status.danger.edge).
// So at accent blend 0 every one of round five's 43 roles, except status.danger.solid and
// status.danger.on, must be byte-identical to round five, over the full input space, and every
// preset (plain and even) must be identical apart from that pair.
// Usage: node tools/unchanged.js [count, default 10000]
const fs = require("fs"), path = require("path");
const blk = (f) => fs.readFileSync(f, "utf8").match(/<script id="resolve-theme">([\s\S]*?)<\/script>/)[1];
const prs = (f) => fs.readFileSync(f, "utf8").match(/<script id="presets">([\s\S]*?)<\/script>/)[1];
const V5 = path.join(__dirname, "..", "..", "theming-prototype-v5", "index.html"), V6 = path.join(__dirname, "..", "index.html");
const A = new Function(blk(V5) + "\nreturn PoodleTheme;")();
const B = new Function(blk(V6) + "\nreturn PoodleTheme;")();
const P5 = new Function(prs(V5) + "\nreturn PoodlePresets;")(), P6 = new Function(prs(V6) + "\nreturn PoodlePresets;")();
const N = +(process.argv[2] || 10000);
let seed = 31; const rnd = () => ((seed = (seed * 1103515245 + 12345) % 2147483648) / 2147483648);
const col = () => { const k = rnd(); if (k < 0.1) return "mono"; return "#" + [0, 1, 2].map(() => Math.floor(rnd() * 256).toString(16).padStart(2, "0")).join(""); };
const DANGER = ["status.danger.solid", "status.danger.on"];
const diff = (a, b, skip) => Object.keys(a).filter((k) => !skip.includes(k) && A.hex(a[k]) !== B.hex(b[k]));

let fails = 0, ex = null, dangerChanged = 0, countMismatch = 0, edgeBlend = 0;
for (let i = 0; i < N; i++) {
  // Blends drawn with weight on the ends (0 and 1), where rounding at the cap could show.
  const bbr = rnd(), bb = bbr < 0.15 ? 0 : bbr < 0.3 ? 1 : Math.round(rnd() * 100) / 100;
  if (bb === 1) edgeBlend++;
  const input = { scheme: rnd() < 0.5 ? "light" : "dark", base: col(), baseBlend: bb, accent: col(), accentBlend: 0, contrast: Math.round((0.5 + rnd() * 1.5) * 100) / 100, evenTint: rnd() < 0.5 };
  const a = A.resolveTheme(input).roles, b = B.resolveTheme(input).roles;
  if (Object.keys(b).length !== Object.keys(a).length + 1 || !b["status.danger.edge"]) countMismatch++; // round six adds status.danger.edge
  const d = diff(a, b, DANGER);
  if (d.length) { fails++; if (!ex) ex = { input, d: d.map((k) => `${k} ${A.hex(a[k])} → ${B.hex(b[k])}`) }; }
  if (DANGER.some((k) => A.hex(a[k]) !== B.hex(b[k]))) dangerChanged++;
}
console.log(`accent blend 0, ${N} random inputs (${edgeBlend} at base blend 1): ${fails} with a change outside the danger pair${ex ? " " + JSON.stringify(ex) : ""}; role sets other than round five's plus status.danger.edge: ${countMismatch}; danger pair changed in ${dangerChanged}`);

// Every preset, plain and even: all roles but the danger pair identical, and the surface exact.
for (const list of ["PRESETS", "PRESETS_EVEN"]) {
  const rows = P6[list].map((p, i) => {
    const q = P5[list][i], same = JSON.stringify(p) === JSON.stringify(q);
    const a = A.resolveTheme(q).roles, b = B.resolveTheme(p).roles, d = diff(a, b, DANGER);
    const dz = DANGER.map((k) => A.hex(a[k]) + "→" + B.hex(b[k])).join(" ");
    return `${p.id.padEnd(16)} inputs ${same ? "same" : "CHANGED"}, surface ${B.hex(b["layer.surface"])} ${A.hex(a["layer.surface"]) === B.hex(b["layer.surface"]) ? "unchanged" : "MOVED"}, other roles changed: ${d.length ? d.join(",") : "none"}; danger solid/on ${dz}`;
  });
  console.log(`\n${list}:\n` + rows.join("\n"));
}

// For information: how much of the space does round six change when accent blend is above 0?
let changedNeutral = 0, M = 2000;
for (let i = 0; i < M; i++) {
  const input = { scheme: rnd() < 0.5 ? "light" : "dark", base: col(), baseBlend: Math.round(rnd() * 100) / 100, accent: col(), accentBlend: Math.round((0.01 + rnd() * 0.99) * 100) / 100, contrast: 1, evenTint: rnd() < 0.5 };
  const a = A.resolveTheme(input).roles, b = B.resolveTheme(input).roles;
  if (A.hex(a["layer.surface"]) !== B.hex(b["layer.surface"])) changedNeutral++;
}
console.log(`\naccent blend above 0, ${M} random inputs: surface differs from round five in ${changedNeutral} (expected: the fix)`);
