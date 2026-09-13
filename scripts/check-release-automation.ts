// g18.032 / spec 071: the release-automation guard proves structure, not
// incidental workflow steps.
//
// It requires: one Effigy npm certificate entry in candidate mode; no
// aggregate/Rust/native/GPUI/Jetstream selector or setup; a Linux runner and a
// hard ten-minute timeout; candidate artifact and publish-time identity checks;
// publication of archives rather than package directories; tag plus explicit
// publish-mode mutation guards; and core/Svelte only in the publication set,
// derived from `packages/release-manifest.json`.
//
// It is deliberately static: no dispatch, no registry, no workflow execution.

import fs from "node:fs";
import path from "node:path";

import { readNpmPublicationAuthority } from "./npm-publication";

const root = process.cwd();
const retainedWorkflows = [
  "ci-web.yml",
  "ci-rust.yml",
  "ci-native.yml",
  "ci-visual.yml",
  "release.yml",
];

const failures: string[] = [];

function read(relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function assert(condition: boolean, message: string): void {
  if (!condition) failures.push(message);
}

function withoutComments(source: string): string {
  return source
    .split("\n")
    .filter((line) => !line.trimStart().startsWith("#"))
    .join("\n");
}

function runCommands(source: string): string[] {
  return withoutComments(source)
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.startsWith("run:"))
    .map((line) => line.slice("run:".length).trim());
}

function requireRun(source: string, command: string, file: string): void {
  assert(runCommands(source).includes(command), `${file} must run ${command}`);
}

function triggerKeys(source: string): string[] {
  const active = withoutComments(source);
  const on = /^on:\s*$/m.exec(active);
  if (!on) return [];
  const keys: string[] = [];
  for (const line of active.slice(on.index).split("\n").slice(1)) {
    if (/^\S/.test(line)) break;
    const child = /^ {2}([a-z_]+):/.exec(line);
    if (child) keys.push(child[1]);
  }
  return keys;
}

function eventBranchTargets(source: string, event: string): string[] {
  const active = withoutComments(source);
  const on = /^on:\s*$/m.exec(active);
  if (!on) return [];
  const targets: string[] = [];
  let inEvent = false;
  for (const line of active.slice(on.index).split("\n").slice(1)) {
    if (/^\S/.test(line)) break;
    if (/^ {2}[a-z_]+:/.test(line)) {
      inEvent = line.startsWith(`  ${event}:`);
      continue;
    }
    if (!inEvent) continue;
    const inline = /branches:\s*\[([^\]]*)\]/.exec(line);
    if (inline) {
      targets.push(...inline[1].split(",").map((branch) => branch.trim()).filter(Boolean));
      continue;
    }
    const item = /^\s*-\s*(.+)$/.exec(line);
    if (item) targets.push(item[1].trim());
  }
  return targets;
}

export type NpmPublication = ReturnType<typeof readNpmPublicationAuthority>;

const RELEASE_CERTIFICATE_SELECTOR = "effigy release:web-certificate";
const RELEASE_VERIFY = "scripts/verify-npm-candidate.ts";
const RELEASE_ARTIFACT_NAME = "poodle-npm-candidate";
const RELEASE_FORBIDDEN = [
  { label: "aggregate qa", pattern: /\beffigy\s+qa\b/ },
  { label: "aggregate ci", pattern: /\beffigy\s+ci(?::|\s|$)/ },
  { label: "docs or validation board", pattern: /\beffigy\s+(?:docs|test:|check:|audit:)/ },
  { label: "Rust toolchain setup", pattern: /rust-toolchain|cargo\s|Cargo\.toml/ },
  { label: "native/GPUI selector", pattern: /\bgpui\b|\bjetstream\b|check:gpui|ci:native/ },
  { label: "macOS runner", pattern: /macos-/ },
];

/**
 * The npm lane is a short packaging operation over an already reviewed web
 * candidate. This collector is the law that keeps it that way.
 */
export function collectReleaseWorkflowFailures(
  source: string,
  publication: NpmPublication,
): string[] {
  const found: string[] = [];
  const active = withoutComments(source);
  const commands = runCommands(source).join("\n");

  const effigyInvocations = [...active.matchAll(/\beffigy\s+([A-Za-z0-9:_-]+)/g)].map(
    (match) => match[0],
  );
  const uniqueEffigy = [...new Set(effigyInvocations)];
  if (uniqueEffigy.length !== 1 || uniqueEffigy[0] !== RELEASE_CERTIFICATE_SELECTOR) {
    found.push(
      `release.yml must invoke exactly one Effigy certificate entry (${RELEASE_CERTIFICATE_SELECTOR}); found ${uniqueEffigy.join(", ") || "none"}`,
    );
  }
  for (const { label, pattern } of RELEASE_FORBIDDEN) {
    if (pattern.test(active)) found.push(`release.yml must not carry ${label}`);
  }
  if (!/runs-on:\s*ubuntu-latest/.test(active)) {
    found.push("release.yml must run on ubuntu-latest");
  }
  if (!/timeout-minutes:\s*10\b/.test(active)) {
    found.push("release.yml must declare a ten-minute hard job timeout");
  }
  if (!active.includes("id-token: write")) {
    found.push("release.yml must retain job-local OIDC permission for trusted publishing");
  }
  if (!active.includes("actions: read")) {
    found.push("release.yml must retain actions:read to download the candidate run artifact");
  }
  if (!active.includes("--access public")) {
    found.push("release.yml must publish with the public-package guard");
  }
  if (/dry-run/.test(active)) {
    found.push("release.yml must not retain a tag dry-run phase");
  }

  const modes = [...active.matchAll(/^\s{6}([a-z-]+):$/gm)].map((match) => match[1]);
  if (!active.includes("options: [candidate, publish]") || !active.includes("default: candidate")) {
    found.push("release.yml must declare exactly the candidate and publish modes");
  }
  if (modes.includes("dry-run")) {
    found.push("release.yml must not declare a dry-run mode");
  }

  if (!commands.includes("effigy release:web-certificate")) {
    found.push("release.yml must run the one Effigy npm certificate entry in candidate mode");
  }
  if (!active.includes(RELEASE_VERIFY)) {
    found.push("release.yml must verify the candidate identity before publish");
  }
  if (!/gh run download "\$\{\{\s*inputs\.candidate-run-id\s*\}\}"/.test(active)) {
    found.push("publish mode must download the named candidate run artifact");
  }
  if (!active.includes(`--name ${RELEASE_ARTIFACT_NAME}`)) {
    found.push("publish mode must download the certified candidate artifact by name");
  }
  if (!active.includes("--source-commit")) {
    found.push("publish mode must bind the candidate source commit to the tag commit");
  }
  if (!active.includes("--tag-version")) {
    found.push("publish mode must bind the candidate version to the tag");
  }
  if (!/npm publish "\$tarball"/.test(active) && !/npm publish "\$\{?tarball\}?"/.test(active)) {
    found.push("publish mode must publish archives (npm publish <tarball>)");
  }
  if (/cd\s+packages\//.test(active) && /npm publish/.test(active)) {
    found.push("publish mode must publish archives rather than package directories");
  }
  if (!/upload-artifact/.test(active) || !/release-artifacts\/\*\*/.test(active)) {
    found.push("candidate mode must upload the archive set and its identity manifest");
  }
  if (!/inputs\.mode\s*==\s*'publish'/.test(active)) {
    found.push("publish steps must be guarded on the explicit publish mode");
  }
  if (!/inputs\.mode\s*==\s*'candidate'/.test(active)) {
    found.push("candidate steps must be guarded on the explicit candidate mode");
  }
  if (!/refs\/tags\/v/.test(active)) {
    found.push("publish mode must require a versioned release tag");
  }
  if (!active.includes("candidate-run-id")) {
    found.push("publish mode must require the candidate run ID input");
  }

  const manifestNames = publication.packages.map((entry) => entry.name).sort();
  const expectedNames = [
    "@inflatable-cookie/poodle-core",
    "@inflatable-cookie/poodle-svelte",
  ].sort();
  if (JSON.stringify(manifestNames) !== JSON.stringify(expectedNames)) {
    found.push(
      `release manifest npm publication set must be exactly core and Svelte; found ${manifestNames.join(", ") || "none"}`,
    );
  }
  // Package paths derive from the manifest; the workflow must not repeat them.
  if (/packages\/(?:core|svelte|react)/.test(active)) {
    found.push("release.yml must derive package paths from the release manifest, not repeat them");
  }
  if (active.includes("@inflatable-cookie/poodle-react")) {
    found.push("release.yml must not publish the private React package");
  }
  for (const entry of publication.packages) {
    if (!fs.existsSync(path.join(root, entry.path, "package.json"))) {
      found.push(`release manifest package path ${entry.path} has no package.json`);
    }
  }
  return found;
}

/** Active release authority must not freeze one generation or one version. */
export function collectGenericCandidateFailures(
  surfaces: Record<string, string>,
): string[] {
  const found: string[] = [];
  const frozen = /g1[0-9]\.0[0-9]{2}-candidate|(?:^|[^\d.])0\.(?:2\.3|3\.0|4\.0)(?:[^\d]|$)/;
  for (const [name, source] of Object.entries(surfaces)) {
    if (frozen.test(withoutComments(source))) {
      found.push(`${name} must not freeze a generation or version in active release policy`);
    }
  }
  return found;
}

const actionRefPattern = /^\s*(?:-\s+)?uses:\s*([^\s#]+)(?:\s+#\s*(.*))?$/gm;

for (const file of retainedWorkflows) {
  const relativePath = `.github/workflows/${file}`;
  const source = read(relativePath);

  for (const match of source.matchAll(actionRefPattern)) {
    const reference = match[1];
    const comment = match[2]?.trim() ?? "";
    const at = reference.lastIndexOf("@");
    const action = at === -1 ? reference : reference.slice(0, at);
    const revision = at === -1 ? "" : reference.slice(at + 1);
    assert(
      /^[0-9a-f]{40}$/.test(revision),
      `${relativePath} has a non-immutable action reference: ${reference}`,
    );
    assert(comment.length > 0, `${relativePath} must record each action tag`);
    assert(action.includes("/"), `${relativePath} has an invalid action: ${action}`);
  }

  assert(
    !source.includes("NPM_TOKEN") && !source.includes("npm@latest"),
    `${relativePath} must not use a registry token or mutable npm version`,
  );
  assert(
    !source.includes("bun-version: latest") && !source.includes("0.9.1"),
    `${relativePath} must pin Bun and Effigy versions`,
  );
}

const web = read(".github/workflows/ci-web.yml");
const rust = read(".github/workflows/ci-rust.yml");
const native = read(".github/workflows/ci-native.yml");
const visual = read(".github/workflows/ci-visual.yml");
const release = read(".github/workflows/release.yml");

const manualOnlyWorkflows = [
  [native, ".github/workflows/ci-native.yml"],
  [visual, ".github/workflows/ci-visual.yml"],
  [release, ".github/workflows/release.yml"],
] as const;

for (const [source, relativePath] of manualOnlyWorkflows) {
  const active = withoutComments(source);
  assert(/^\s*workflow_dispatch:/m.test(active), `${relativePath} must remain manually dispatched`);
  assert(
    !/^\s*(?:push|pull_request|schedule):/m.test(active),
    `${relativePath} must not add an automatic trigger`,
  );
}

const automaticWorkflows = [
  [web, ".github/workflows/ci-web.yml"],
  [rust, ".github/workflows/ci-rust.yml"],
] as const;

for (const [source, relativePath] of automaticWorkflows) {
  const active = withoutComments(source);
  const keys = triggerKeys(active);
  const expected = ["pull_request", "push", "workflow_dispatch"];
  assert(
    keys.length === expected.length && expected.every((key) => keys.includes(key)),
    `${relativePath} must trigger on pull_request and push to main plus workflow_dispatch and nothing else`,
  );
  for (const event of ["push", "pull_request"]) {
    const targets = eventBranchTargets(active, event);
    assert(
      targets.length === 1 && targets[0] === "main",
      `${relativePath} ${event} trigger must target main only`,
    );
  }
}

requireRun(web, "effigy ci:web", ".github/workflows/ci-web.yml");
requireRun(rust, "effigy ci:rust", ".github/workflows/ci-rust.yml");
requireRun(native, "effigy ci:native", ".github/workflows/ci-native.yml");

const rustSetup = "uses: dtolnay/rust-toolchain@4360b52568e2003a75bf9bc1d59f33a8e3fc893c";
for (const [source, file] of [
  [rust, ".github/workflows/ci-rust.yml"],
  [native, ".github/workflows/ci-native.yml"],
] as const) {
  assert(source.includes(rustSetup), `${file} must use the reviewed Rust action`);
  assert(source.includes('toolchain: "1.95"'), `${file} must select Rust 1.95 explicitly`);
}
assert(
  !release.includes(rustSetup) && !release.includes('toolchain: "1.95"'),
  "release.yml must not install a Rust toolchain",
);

assert(
  native.includes("uses: oven-sh/setup-bun@0c5077e51419868618aeaa5fe8019c62421857d6") &&
    native.includes('bun-version: "1.3.14"'),
  "ci-native.yml must install the reviewed Bun 1.3.14 runtime",
);
assert(
  native.indexOf("uses: oven-sh/setup-bun@") < native.indexOf("run: effigy ci:native"),
  "ci-native.yml must install Bun before the native selector",
);

const visualActive = withoutComments(visual);
for (const selector of ["effigy test:visual-smoke", "effigy ci:visual", "effigy test:visual-sweep"]) {
  assert(visualActive.includes(selector), `ci-visual.yml must map an input to ${selector}`);
}
assert(visualActive.includes('case "$TIER"'), "ci-visual.yml must select from the tier input");
assert(
  visualActive.includes("*)") && visualActive.includes("exit 1"),
  "ci-visual.yml must reject unknown tiers",
);

for (const source of [web, rust, native, visual]) {
  assert(!source.includes("ci:conformance"), "retained workflows must not use ci:conformance");
  assert(
    !source.includes("packages/gpui/components/Cargo.toml"),
    "retained workflows must not name the deleted GPUI components crate",
  );
}

const publication = readNpmPublicationAuthority(root);

const manifest = read("effigy.toml");
assert(
  manifest.includes('minimum_effigy_version = "0.11.0"'),
  "effigy.toml must require Effigy 0.11.0",
);
assert(manifest.includes("[release.gates.headless]"), "headless release gate must be configured");
assert(
  manifest.includes(`command = "${RELEASE_CERTIFICATE_SELECTOR}"`),
  `headless release gate must run ${RELEASE_CERTIFICATE_SELECTOR}`,
);
assert(
  !/command = "effigy qa"/.test(manifest),
  "aggregate qa must not be release authority",
);

const taskManifest = read("tasks/effigy.tasks.toml");
assert(!taskManifest.includes("ci:conformance"), "the stale ci:conformance alias must be removed");
assert(
  fs.existsSync(path.join(root, ".github/workflows/ci-conformance.yml")) === false,
  "stale conformance workflow must be deleted",
);

// The npm certificate is one selector that separates admission from archive
// certification, and `qa` stays the aggregate board through the bounded runner.
assert(
  taskManifest.includes('"release:web-admission"') &&
    taskManifest.includes('"release:web-archive"') &&
    taskManifest.includes('"release:web-certificate"'),
  "the npm certificate selectors must exist and stay separate from archive certification",
);
assert(
  taskManifest.includes('qa = "bun scripts/validation/run-board.ts qa:board"'),
  "qa must execute the bounded, observable validation runner",
);
assert(
  taskManifest.includes('"qa:board" = ['),
  "the complete aggregate inventory must stay declared as qa:board",
);

const bounds = JSON.parse(read("quality/validation-bounds.json")) as {
  boardTimeoutMs: number;
  childTimeoutMs: number;
  tasks: Record<string, number>;
};
assert(
  bounds.boardTimeoutMs <= 15 * 60 * 1000,
  "the full board must hard-stop at fifteen minutes",
);
assert(
  bounds.childTimeoutMs <= 5 * 60 * 1000,
  "no child may run silently for more than five minutes",
);
assert(
  typeof bounds.tasks["probe:gpui-specimens"] === "number" &&
    bounds.tasks["probe:gpui-specimens"] <= 3 * 60 * 1000,
  "the named specimen probe must carry an explicit smaller bound",
);

for (const failure of collectReleaseWorkflowFailures(release, publication)) {
  failures.push(failure);
}
for (const failure of collectGenericCandidateFailures({
  ".github/workflows/release.yml": release,
  "effigy.toml": manifest,
})) {
  failures.push(failure);
}

// ---------------------------------------------------------------------------
// Planted negatives: each mutation of the retained surfaces must fail the law.
// ---------------------------------------------------------------------------

type Plant = { name: string; source: string; expect: RegExp };

const releasePlants: Plant[] = [
  {
    name: "restore the aggregate qa board",
    source: release.replace(
      "run: effigy release:web-certificate",
      "run: effigy qa",
    ),
    expect: /exactly one Effigy certificate entry|must not carry aggregate qa/,
  },
  {
    name: "install the Rust toolchain",
    source: release.replace(
      "      - uses: oven-sh/setup-bun@",
      "      - uses: dtolnay/rust-toolchain@4360b52568e2003a75bf9bc1d59f33a8e3fc893c # Rust 1.95\n      - uses: oven-sh/setup-bun@",
    ),
    expect: /must not carry Rust toolchain setup/,
  },
  {
    name: "move back to a macOS runner",
    source: release.replace("runs-on: ubuntu-latest", "runs-on: macos-latest"),
    expect: /must run on ubuntu-latest|must not carry macOS runner/,
  },
  {
    name: "drop the hard timeout",
    source: release.replace("    timeout-minutes: 10\n", ""),
    expect: /ten-minute hard job timeout/,
  },
  {
    name: "publish a package directory instead of archives",
    source: release.replace(
      '          for tarball in "${tarballs[@]}"; do\n            npm publish "$tarball" --access public\n          done',
      '          for dir in packages/core packages/svelte/components; do\n            (cd "$dir" && npm publish --access public)\n          done',
    ),
    expect: /publish archives|package directories/,
  },
  {
    name: "add a second Effigy entry",
    source: release.replace(
      "run: effigy release:web-certificate",
      "run: effigy release:web-certificate\n      - run: effigy docs:lint",
    ),
    expect: /exactly one Effigy certificate entry/,
  },
  {
    name: "reintroduce a tag dry run",
    source: release.replace(
      "      candidate-run-id:",
      "      dry-run:\n        description: pack without publishing\n        type: boolean\n        default: true\n      candidate-run-id:",
    ),
    expect: /dry-run/,
  },
  {
    name: "drop publish source-commit binding",
    source: release.replace(/ --source-commit "\$\(git rev-parse HEAD\)"/, ""),
    expect: /bind the candidate source commit/,
  },
  {
    name: "drop the candidate artifact upload",
    source: release.replace("          path: release-artifacts/**\n", ""),
    expect: /upload the archive set|archive set and its identity manifest/,
  },
];

for (const plant of releasePlants) {
  const planted = collectReleaseWorkflowFailures(plant.source, publication);
  const caught = planted.some((failure) => plant.expect.test(failure));
  assert(caught, `plant "${plant.name}" must fail the checker; got: ${planted.join("; ") || "no failures"}`);
  console.log(caught ? `plant ${plant.name}: failed as required` : `plant ${plant.name}: did not fail`);
}

const reactPublication = {
  ...publication,
  packages: [
    ...publication.packages,
    { name: "@inflatable-cookie/poodle-react", path: "packages/react/components" },
  ],
};
const reactFailure = collectReleaseWorkflowFailures(release, reactPublication).some((failure) =>
  failure.includes("exactly core and Svelte"),
);
assert(
  reactFailure,
  "adding React to the release authority must fail the checker",
);
console.log(
  reactFailure
    ? "plant React publication authority: failed as required"
    : "plant React publication authority: did not fail",
);

const frozenFailure = collectGenericCandidateFailures({
  "release.yml": `${release}\n# g18.006 frozen candidate\n`,
  "tasks": 'x = "0.4.0"',
}).length;
assert(frozenFailure >= 1, "a frozen generation or version in active release policy must fail");
console.log(
  frozenFailure >= 1
    ? "plant frozen release policy: failed as required"
    : "plant frozen release policy: did not fail",
);

if (failures.length > 0) {
  console.error("release automation static check: FAIL");
  for (const failure of failures) console.error(`- ${failure}`);
  process.exitCode = 1;
} else {
  console.log("release automation static check: pass");
  console.log(
    `checked ${retainedWorkflows.length} retained workflows, the npm certificate authority, generic candidate admission, validation bounds, archive/identity protocol, publication set and planted failures`,
  );
}
