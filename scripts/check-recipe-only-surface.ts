/** Fail when the retired Treatment system leaks back into active surfaces. */

const ROOT = new URL("..", import.meta.url).pathname;
const SELF = "scripts/check-recipe-only-surface.ts";
const RETIRED_ARCHITECTURE =
  "docs/architecture/005-treatment-system-and-recipe-variables.md";
const RETIRED_ARCHITECTURE_LINK = "005-treatment-system-and-recipe-variables.md";
const HISTORICAL_PREFIXES = ["docs/logs/", "docs/parity/", "docs/roadmaps/"];
const SCANNED_EXTENSIONS = new Set([
  ".css",
  ".json",
  ".md",
  ".rs",
  ".svelte",
  ".toml",
  ".ts",
  ".tsx",
]);

const retiredName = "treatment";
const forbidden = [
  new RegExp(`--poodle-${retiredName}-`, "i"),
  new RegExp(`data-appearance-${retiredName}`, "i"),
  new RegExp(`Appearance${retiredName[0].toUpperCase()}${retiredName.slice(1)}`),
  new RegExp(`${retiredName[0].toUpperCase()}${retiredName.slice(1)}Tokens`),
  new RegExp(`Section::${retiredName[0].toUpperCase()}${retiredName.slice(1)}s`),
  new RegExp(`\\b${retiredName}[- ](?:tokens?|roles?|system|interactive|surface)\\b`, "i"),
  new RegExp(`["']${retiredName}s["']`, "i"),
];

function excluded(path: string): boolean {
  return (
    path === SELF ||
    path === RETIRED_ARCHITECTURE ||
    HISTORICAL_PREFIXES.some((prefix) => path.startsWith(prefix)) ||
    path.includes("/node_modules/") ||
    path.includes("/target/") ||
    path.includes("/dist/") ||
    path.startsWith(".git/") ||
    path.startsWith("dist/") ||
    path.startsWith("node_modules/") ||
    path.startsWith("target/")
  );
}

const glob = new Bun.Glob("**/*");
const failures: string[] = [];
let checked = 0;

for await (const path of glob.scan({ cwd: ROOT, onlyFiles: true })) {
  if (excluded(path)) continue;
  const dot = path.lastIndexOf(".");
  if (dot < 0 || !SCANNED_EXTENSIONS.has(path.slice(dot))) continue;

  checked += 1;
  const lines = (await Bun.file(`${ROOT}/${path}`).text()).split("\n");
  for (const [index, line] of lines.entries()) {
    if (
      path === "docs/architecture/007-appearance-recipe-contract.md" &&
      line.includes(RETIRED_ARCHITECTURE_LINK)
    ) {
      continue;
    }
    if (forbidden.some((pattern) => pattern.test(line))) {
      failures.push(`${path}:${index + 1}: ${line.trim()}`);
    }
  }
}

if (failures.length > 0) {
  console.error(
    `retired Treatment drift: ${failures.length} active reference(s) found:\n${failures
      .map((failure) => `  ${failure}`)
      .join("\n")}`,
  );
  process.exit(1);
}

console.log(`retired Treatment drift: checked ${checked} active files, 0 references`);
