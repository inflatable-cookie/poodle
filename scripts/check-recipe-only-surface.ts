/** Fail when the retired Treatment system leaks back into active surfaces. */

export type RetirementGateResult = {
  status: number;
  output: string;
};

const SELF = "scripts/check-recipe-only-surface.ts";
const RETIRED_ARCHITECTURE =
  "docs/knowledge/architecture/005-treatment-system-and-recipe-variables.md";
const RETIRED_ARCHITECTURE_LINK = "005-treatment-system-and-recipe-variables.md";
// Superseded specs are retained as evidence and never edited to satisfy a gate.
const HISTORICAL_PREFIXES = ["docs/knowledge/specs/archive/"];
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

/**
 * Scan `root` and report every retired-Treatment reference outside the
 * historical exemptions. Exported so the planted test can exercise the real
 * logic in-process: spawning `bun <script>` once per fixture cost 7-9s on a
 * host whose temp root had grown to 237k entries, which is what timed the
 * tests out at bun's 5s default (Queue's gate for #59).
 */
export async function scanRetiredTreatment(root: string): Promise<RetirementGateResult> {
  const glob = new Bun.Glob("**/*");
  const failures: string[] = [];
  let checked = 0;

  for await (const path of glob.scan({ cwd: root, onlyFiles: true })) {
    if (excluded(path)) continue;
    const dot = path.lastIndexOf(".");
    if (dot < 0 || !SCANNED_EXTENSIONS.has(path.slice(dot))) continue;

    checked += 1;
    const lines = (await Bun.file(`${root}/${path}`).text()).split("\n");
    for (const [index, line] of lines.entries()) {
      if (
        path === "docs/knowledge/architecture/007-appearance-recipe-contract.md" &&
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
    return {
      status: 1,
      output:
        `retired Treatment drift: ${failures.length} active reference(s) found:\n` +
        `${failures.map((failure) => `  ${failure}`).join("\n")}\n`,
    };
  }
  return {
    status: 0,
    output: `retired Treatment drift: checked ${checked} active files, 0 references\n`,
  };
}

if (import.meta.main) {
  const root = new URL("..", import.meta.url).pathname;
  const result = await scanRetiredTreatment(root);
  if (result.status !== 0) process.stderr.write(result.output);
  else process.stdout.write(result.output);
  process.exit(result.status);
}
