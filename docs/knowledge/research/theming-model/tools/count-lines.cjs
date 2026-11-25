// Counts code lines (no blanks, no comment-only lines) in each round's resolver, by part.
// Usage: node count-lines.js <round-two index.html> <round-three index.html>
const fs = require("fs");
const code = (s) => s.split("\n").filter((l) => l.trim() && !/^\s*\/\//.test(l)).length;
const parts = (file, modelStart) => {
  const src = fs.readFileSync(file, "utf8").match(/<script id="resolve-theme">([\s\S]*?)<\/script>/)[1];
  const a = src.indexOf(modelStart), b = src.indexOf("// ── Audit");
  return { shared: code(src.slice(0, a)), model: code(src.slice(a, b)), audit: code(src.slice(b)), total: code(src) };
};
const r1 = parts(process.argv[2], "  function parseColor"), r2 = parts(process.argv[3], "  function parseColor");
console.log("part                         round 2  round 3");
[["colour maths, search, fits", "shared"], ["model: data, constants, resolver", "model"], ["audit and exports", "audit"], ["total", "total"]].forEach(([l, k]) => console.log(l.padEnd(29), String(r1[k]).padStart(6), String(r2[k]).padStart(8)));
