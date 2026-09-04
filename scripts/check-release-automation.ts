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

function triggerKeys(source: string): string[] {
  // Event names declared directly under `on:`, in file order; [] when absent.
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
  // Branch names an `on:` event is restricted to; [] when unrestricted.
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

const PACK_STEP_NAME = "- name: Pack and verify contents";
const PUBLISH_STEP_NAME = "- name: Publish";
const TAG_REQUIRE_STEP_NAME = "- name: Require a versioned release tag";
const VERSIONS_TAG_STEP_NAME = "- name: Versions agree with the tag";
const VERSIONS_LOCKSTEP_STEP_NAME = "- name: Versions agree with each other";
const ORIGIN_MAIN_FETCH = "git fetch --no-tags origin main:refs/remotes/origin/main";
const TAG_REQUIRE_IF = "if: ${{ !startsWith(github.ref, 'refs/tags/v') && !inputs.dry-run }}";
const VERSIONS_TAG_IF = "if: ${{ startsWith(github.ref, 'refs/tags/v') }}";
const VERSIONS_LOCKSTEP_IF = "if: ${{ !startsWith(github.ref, 'refs/tags/v') }}";
const PUBLISH_IF = "if: ${{ startsWith(github.ref, 'refs/tags/v') && !inputs.dry-run }}";
const RECEIPT_PATTERN = "^package/dist/\\.poodle-build\\.json$";
const ICON_JS_PATTERN = "^package/dist/icons/icons/.*\\.js$";
const ICON_DTS_PATTERN = "^package/dist/icons/icons/.*\\.d\\.ts$";
const ICON_ALIAS_PATTERN = "^package/dist/icons/aliases\\.generated\\.d\\.ts$";
const TOKEN_CSS_PATTERN = "^package/dist/tokens/generated/css/.*\\.css$";
const STALE_SOURCE_PATTERN = "^package/src/icons/icons/.*\\.ts$";

function namedStepBlock(source: string, stepNameMarker: string): string {
  const start = source.indexOf(stepNameMarker);
  if (start === -1) return "";
  const lines = source.slice(start).split("\n");
  const block = [lines[0]];
  for (const line of lines.slice(1)) {
    if (/^      - /.test(line)) break;
    block.push(line);
  }
  return block.join("\n");
}

function packVerifyBlock(source: string): string {
  const start = source.indexOf(PACK_STEP_NAME);
  const publish = source.indexOf(PUBLISH_STEP_NAME);
  if (start === -1 || publish === -1 || publish <= start) return "";
  return source.slice(start, publish);
}

function requireLine(
  pack: string,
  description: string,
  pattern: string,
  minimum: number,
): boolean {
  const suffix = new RegExp(`\\s${minimum}\\s*$`);
  return pack.split("\n").some(
    (line) =>
      line.includes(`require "$tarball" "${description}"`) &&
      line.includes(pattern) &&
      suffix.test(line),
  );
}

function collectPackAssertionFailures(source: string): string[] {
  const found: string[] = [];
  const pack = packVerifyBlock(source);
  if (!pack) {
    found.push("release.yml must keep a Pack and verify contents block before Publish");
    return found;
  }

  if (!pack.includes("packages/core") || !pack.includes("packages/svelte/components")) {
    found.push("release pack verifier must still pack core and Svelte");
  }
  if (pack.includes("packages/react/components")) {
    found.push("release pack verifier must not pack React");
  }

  if (!requireLine(pack, "licence", "^package/LICENSE$", 1)) {
    found.push("release pack verifier must require package/LICENSE");
  }
  if (!requireLine(pack, "readme", "^package/README.md$", 1)) {
    found.push("release pack verifier must require package/README.md");
  }
  if (!requireLine(pack, "manifest", "^package/package.json$", 1)) {
    found.push("release pack verifier must require package/package.json");
  }
  if (!requireLine(pack, "receipt", RECEIPT_PATTERN, 1)) {
    found.push(
      "release pack verifier must require package/dist/.poodle-build.json for every packed package",
    );
  }

  const receiptAt = pack.indexOf(RECEIPT_PATTERN);
  const coreCaseAt = pack.indexOf("*poodle-core*");
  if (receiptAt === -1 || coreCaseAt === -1 || receiptAt > coreCaseAt) {
    found.push("release pack verifier must require the compiled receipt before the core-only case");
  }

  if (!requireLine(pack, "icon modules", ICON_JS_PATTERN, 50)) {
    found.push("release pack verifier must require at least 50 package/dist/icons/icons/*.js modules");
  }
  if (!requireLine(pack, "icon declarations", ICON_DTS_PATTERN, 50)) {
    found.push(
      "release pack verifier must require at least 50 package/dist/icons/icons/*.d.ts declarations",
    );
  }
  if (!requireLine(pack, "icon aliases", ICON_ALIAS_PATTERN, 1)) {
    found.push("release pack verifier must require package/dist/icons/aliases.generated.d.ts");
  }
  if (!requireLine(pack, "token css", TOKEN_CSS_PATTERN, 20)) {
    found.push(
      "release pack verifier must require at least 20 package/dist/tokens/generated/css/*.css files",
    );
  }

  for (const [pattern, label] of [
    [ICON_JS_PATTERN, "icon modules"],
    [ICON_DTS_PATTERN, "icon declarations"],
    [ICON_ALIAS_PATTERN, "icon aliases"],
    [TOKEN_CSS_PATTERN, "token css"],
  ] as const) {
    const at = pack.indexOf(pattern);
    if (coreCaseAt === -1 || at === -1 || at < coreCaseAt) {
      found.push(`release pack verifier must keep ${label} inside the core archive case`);
    }
  }

  if (pack.includes("package/src/")) {
    found.push("release pack verifier must not require stale package/src members");
  }

  return found;
}

function collectReleaseProtocolFailures(source: string): string[] {
  const found: string[] = [];
  const active = withoutComments(source);

  if (!active.includes("fetch-depth: 0")) {
    found.push("release checkout must unshallow with fetch-depth: 0");
  }
  if (!active.includes(ORIGIN_MAIN_FETCH)) {
    found.push("release must fetch origin/main for the pack-install scope classifier");
  }

  const tagRequire = namedStepBlock(source, TAG_REQUIRE_STEP_NAME);
  if (!tagRequire.includes(TAG_REQUIRE_IF)) {
    found.push("Require a versioned release tag must fail only when dry-run is false");
  }
  if (!tagRequire.includes("Release workflow must be dispatched against refs/tags/v*")) {
    found.push("release must fail closed on a non-tag publish attempt");
  }

  const versionsTag = namedStepBlock(source, VERSIONS_TAG_STEP_NAME);
  if (!versionsTag.includes(VERSIONS_TAG_IF)) {
    found.push("Versions agree with the tag must run only on tag refs");
  }
  if (!versionsTag.includes('tag="${GITHUB_REF_NAME#v}"')) {
    found.push("Versions agree with the tag must compare manifests to the tag");
  }

  const versionsLockstep = namedStepBlock(source, VERSIONS_LOCKSTEP_STEP_NAME);
  if (!versionsLockstep.includes(VERSIONS_LOCKSTEP_IF)) {
    found.push("Versions agree with each other must run only on non-tag refs");
  }
  if (!versionsLockstep.includes('expected="$version"')) {
    found.push("Versions agree with each other must compare manifests to each other");
  }

  const publish = namedStepBlock(source, PUBLISH_STEP_NAME);
  if (!publish.includes(PUBLISH_IF)) {
    found.push("Publish must require a versioned release tag and an explicit non-dry-run input");
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

// Trigger shape is per workflow: the two ubuntu-only Linux boards run
// automatically on pull requests targeting main and pushes to main; the
// native, visual, and release lanes stay dispatch-only (operator decision
// 2026-09-02, g16.096).
const manualOnlyWorkflows = [
  [native, ".github/workflows/ci-native.yml"],
  [visual, ".github/workflows/ci-visual.yml"],
  [release, ".github/workflows/release.yml"],
] as const;

for (const [source, relativePath] of manualOnlyWorkflows) {
  const active = withoutComments(source);
  assert(
    /^\s*workflow_dispatch:/m.test(active),
    `${relativePath} must remain manually dispatched`,
  );
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
requireRun(release, "effigy release gates", ".github/workflows/release.yml");

const rustSetup = "uses: dtolnay/rust-toolchain@4360b52568e2003a75bf9bc1d59f33a8e3fc893c";
for (const [source, file] of [
  [rust, ".github/workflows/ci-rust.yml"],
  [native, ".github/workflows/ci-native.yml"],
  [release, ".github/workflows/release.yml"],
] as const) {
  assert(source.includes(rustSetup), `${file} must use the reviewed Rust action`);
  assert(source.includes('toolchain: "1.95"'), `${file} must select Rust 1.95 explicitly`);
}
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
assert(
  releaseActive.includes('npm install --prefix "$npm_cli" --no-save npm@12.0.2') &&
    releaseActive.includes('echo "$npm_cli/node_modules/.bin" >> "$GITHUB_PATH"'),
  "release must install the reviewed npm CLI into an isolated runner prefix",
);
assert(!releaseActive.includes("npm install --global npm@"), "release must not replace its running npm CLI in place");
assert(!releaseActive.includes("run: effigy ci"), "release must not maintain the old partial CI gate");
assert(
  releaseActive.includes("cargo install cargo-deny --version 0.19.4 --locked") &&
    releaseActive.indexOf("cargo install cargo-deny --version 0.19.4 --locked") <
      releaseActive.indexOf("run: effigy release gates"),
  "release must install the reviewed cargo-deny CLI before its gate",
);
for (const failure of collectReleaseProtocolFailures(release)) failures.push(failure);

const publishStart = releaseActive.indexOf("- name: Publish");
const publishBlock = publishStart === -1 ? "" : releaseActive.slice(publishStart);
assert(
  publishBlock.includes("if: ${{ startsWith(github.ref, 'refs/tags/v') && !inputs.dry-run }}"),
  "Publish must require a versioned release tag and an explicit non-dry-run input",
);
assert(publishBlock.includes("packages/core"), "release must publish core");
assert(publishBlock.includes("packages/svelte/components"), "release must publish Svelte");
assert(!publishBlock.includes("packages/react/components"), "release must not publish React");

for (const failure of collectPackAssertionFailures(release)) failures.push(failure);

const omittedReceipt = release.replaceAll(RECEIPT_PATTERN, "");
const omittedFailures = collectPackAssertionFailures(omittedReceipt);
const omitReceiptFailed = omittedFailures.some(
  (failure) =>
    failure.includes("package/dist/.poodle-build.json") || failure.includes("compiled receipt"),
);
assert(
  omitReceiptFailed,
  `omitting the compiled receipt assertion must fail the checker; got: ${
    omittedFailures.join("; ") || "no failures"
  }`,
);

const restoredSource = release.replace(
  PACK_STEP_NAME,
  `${PACK_STEP_NAME}\n          require "$tarball" "stale source" "${STALE_SOURCE_PATTERN}" 50`,
);
const restoredFailures = collectPackAssertionFailures(restoredSource);
const restoreSourceFailed = restoredFailures.some((failure) => failure.includes("package/src"));
assert(
  restoreSourceFailed,
  `restoring a stale package/src assertion must fail the checker; got: ${
    restoredFailures.join("; ") || "no failures"
  }`,
);

console.log(
  omitReceiptFailed
    ? "plant omit compiled receipt: failed as required"
    : "plant omit compiled receipt: did not fail",
);
console.log(
  restoreSourceFailed
    ? "plant restore package/src: failed as required"
    : "plant restore package/src: did not fail",
);

const omittedFetch = release.replace(ORIGIN_MAIN_FETCH, "git fetch --no-tags origin HEAD");
const omittedFetchFailures = collectReleaseProtocolFailures(omittedFetch);
const omittedFetchFailed = omittedFetchFailures.some((failure) =>
  failure.includes("origin/main"),
);
assert(
  omittedFetchFailed,
  `omitting the origin/main fetch must fail the checker; got: ${
    omittedFetchFailures.join("; ") || "no failures"
  }`,
);

const tagRequireAlways = release.replace(
  TAG_REQUIRE_IF,
  "if: ${{ !startsWith(github.ref, 'refs/tags/v') }}",
);
const tagRequireAlwaysFailures = collectReleaseProtocolFailures(tagRequireAlways);
const tagRequireAlwaysFailed = tagRequireAlwaysFailures.some((failure) =>
  failure.includes("fail only when dry-run is false"),
);
assert(
  tagRequireAlwaysFailed,
  `dropping the dry-run exception from the tag-require step must fail the checker; got: ${
    tagRequireAlwaysFailures.join("; ") || "no failures"
  }`,
);

const versionsTagUnconditional = release.replace(`        ${VERSIONS_TAG_IF}\n`, "");
const versionsTagUnconditionalFailures = collectReleaseProtocolFailures(versionsTagUnconditional);
const versionsTagUnconditionalFailed = versionsTagUnconditionalFailures.some((failure) =>
  failure.includes("must run only on tag refs"),
);
assert(
  versionsTagUnconditionalFailed,
  `running Versions agree with the tag on every ref must fail the checker; got: ${
    versionsTagUnconditionalFailures.join("; ") || "no failures"
  }`,
);

const publishTagOnly = release.replace(PUBLISH_IF, "if: ${{ startsWith(github.ref, 'refs/tags/v') }}");
const publishTagOnlyFailures = collectReleaseProtocolFailures(publishTagOnly);
const publishTagOnlyFailed = publishTagOnlyFailures.some((failure) =>
  failure.includes("versioned release tag and an explicit non-dry-run input"),
);
assert(
  publishTagOnlyFailed,
  `dropping the dry-run guard from Publish must fail the checker; got: ${
    publishTagOnlyFailures.join("; ") || "no failures"
  }`,
);

const publishDryRunOnly = release.replace(PUBLISH_IF, "if: ${{ !inputs.dry-run }}");
const publishDryRunOnlyFailures = collectReleaseProtocolFailures(publishDryRunOnly);
const publishDryRunOnlyFailed = publishDryRunOnlyFailures.some((failure) =>
  failure.includes("versioned release tag and an explicit non-dry-run input"),
);
assert(
  publishDryRunOnlyFailed,
  `dropping the tag-ref guard from Publish must fail the checker; got: ${
    publishDryRunOnlyFailures.join("; ") || "no failures"
  }`,
);

console.log(
  omittedFetchFailed
    ? "plant omit origin/main fetch: failed as required"
    : "plant omit origin/main fetch: did not fail",
);
console.log(
  tagRequireAlwaysFailed
    ? "plant tag-require without dry-run exception: failed as required"
    : "plant tag-require without dry-run exception: did not fail",
);
console.log(
  versionsTagUnconditionalFailed
    ? "plant versions-agree-with-tag on every ref: failed as required"
    : "plant versions-agree-with-tag on every ref: did not fail",
);
console.log(
  publishTagOnlyFailed
    ? "plant publish without dry-run guard: failed as required"
    : "plant publish without dry-run guard: did not fail",
);
console.log(
  publishDryRunOnlyFailed
    ? "plant publish without tag guard: failed as required"
    : "plant publish without tag guard: did not fail",
);

if (failures.length > 0) {
  console.error("release automation static check: FAIL");
  for (const failure of failures) console.error(`- ${failure}`);
  process.exitCode = 1;
} else {
  console.log("release automation static check: pass");
  console.log(
    `checked ${retainedWorkflows.length} retained workflows, Effigy gate, alias, publish set, compiled pack assertions, release protocol, and planted failures`,
  );
}
