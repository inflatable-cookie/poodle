// Keep the retained manual workflows as thin, immutable Effigy launchers.
// This is deliberately static: it catches workflow drift without dispatching
// GitHub Actions or reaching a registry.

import fs from "node:fs";
import path from "node:path";

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

function requireRun(source: string, command: string, file: string): void {
  const runs = withoutComments(source)
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.startsWith("run:"))
    .map((line) => line.slice("run:".length).trim());
  assert(runs.includes(command), `${file} must run ${command}`);
}

const actionRefPattern = /^\s*(?:-\s+)?uses:\s*([^\s#]+)(?:\s+#\s*(.*))?$/gm;

for (const file of retainedWorkflows) {
  const relativePath = `.github/workflows/${file}`;
  const source = read(relativePath);
  const active = withoutComments(source);

  assert(
    /^\s*workflow_dispatch:/m.test(active),
    `${relativePath} must remain manually dispatched`,
  );
  assert(
    !/^\s*(?:push|pull_request|schedule):/m.test(active),
    `${relativePath} must not add an automatic trigger`,
  );

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

requireRun(web, "effigy ci:web", ".github/workflows/ci-web.yml");
requireRun(rust, "effigy ci:rust", ".github/workflows/ci-rust.yml");
requireRun(native, "effigy ci:native", ".github/workflows/ci-native.yml");
requireRun(release, "effigy release gates", ".github/workflows/release.yml");

const visualActive = withoutComments(visual);
for (const selector of ["effigy test:visual-smoke", "effigy ci:visual", "effigy test:visual-sweep"]) {
  assert(visualActive.includes(selector), `ci-visual.yml must map an input to ${selector}`);
}
assert(visualActive.includes('case "$TIER"'), "ci-visual.yml must select from the tier input");
assert(visualActive.includes("*)") && visualActive.includes("exit 1"), "ci-visual.yml must reject unknown tiers");

for (const source of [web, rust, native, visual, release]) {
  assert(!source.includes("ci:conformance"), "retained workflows must not use ci:conformance");
  assert(
    !source.includes("packages/gpui/components/Cargo.toml"),
    "retained workflows must not name the deleted GPUI components crate",
  );
}

const manifest = read("effigy.toml");
assert(
  manifest.includes('minimum_effigy_version = "0.11.0"'),
  "effigy.toml must require Effigy 0.11.0",
);
assert(manifest.includes("[release.gates.headless]"), "headless release gate must be configured");
assert(manifest.includes('command = "effigy qa"'), "headless release gate must run effigy qa");
assert(
  manifest.includes('description = "Run Poodle\'s complete self-contained headless release board"'),
  "headless release gate must describe the complete board",
);

const taskManifest = read("tasks/effigy.tasks.toml");
assert(!taskManifest.includes("ci:conformance"), "the stale ci:conformance alias must be removed");
assert(fs.existsSync(path.join(root, ".github/workflows/ci-conformance.yml")) === false, "stale conformance workflow must be deleted");

const releaseActive = withoutComments(release);
assert(releaseActive.includes("default: true"), "release dry-run must default to true");
assert(releaseActive.includes("id-token: write"), "release publishing must retain job-local OIDC permission");
assert(releaseActive.includes('node-version: "22.22.2"'), "release Node version must be exact");
assert(releaseActive.includes("npm@12.0.2"), "release npm CLI version must be exact");
assert(!releaseActive.includes("run: effigy ci"), "release must not maintain the old partial CI gate");

const publishStart = releaseActive.indexOf("- name: Publish");
const publishBlock = publishStart === -1 ? "" : releaseActive.slice(publishStart);
assert(publishBlock.includes("packages/core"), "release must publish core");
assert(publishBlock.includes("packages/svelte/components"), "release must publish Svelte");
assert(!publishBlock.includes("packages/react/components"), "release must not publish React");

if (failures.length > 0) {
  console.error("release automation static check: FAIL");
  for (const failure of failures) console.error(`- ${failure}`);
  process.exitCode = 1;
} else {
  console.log("release automation static check: pass");
  console.log(`checked ${retainedWorkflows.length} retained workflows, Effigy gate, alias, and publish set`);
}
