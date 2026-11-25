// Write the shared TypeScript oracle with `effigy theme-resolver:vectors`.
import { writeFileSync } from "node:fs";
import { hex } from "../src/theme-color";
import { resolveTheme } from "../src/theme-resolver";
import { themeCases } from "./theme-cases";

const cases = themeCases();
const roles = Object.keys(resolveTheme({ scheme: "dark" }).roles).sort();
const vectors = cases.map(({ id, settings }) => {
  const actual = resolveTheme(settings);
  return JSON.stringify({ id, settings, colors: Object.entries(actual.roles).sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0).map(([, color]) => hex(color)), report: actual.report });
});
const output = [
  "{",
  `  "roles": ${JSON.stringify(roles)},`,
  '  "vectors": [',
  vectors.map((caseJson) => "    " + caseJson).join(",\n"),
  "  ]",
  "}",
  "",
].join("\n");
writeFileSync(new URL("../../contracts/headless/vectors/theme-resolver.json", import.meta.url), output);
console.log(`Wrote ${vectors.length} theme vectors with ${roles.length} roles each.`);
