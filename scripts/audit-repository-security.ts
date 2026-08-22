import { existsSync, readFileSync } from "node:fs";
import { basename } from "node:path";
import {
  validateCargoLockSources,
  validateCargoManifestSources,
} from "./repository-security-policy.ts";

const errors: string[] = [];
const lifecycleHooks = new Set([
  "install",
  "postinstall",
  "postpack",
  "preinstall",
  "prepack",
  "prepare",
  "prepublish",
  "prepublishOnly",
]);
const secretPatterns = [
  ["private key", /-----BEGIN (?:RSA |EC |OPENSSH |DSA |PGP )?PRIVATE KEY-----/],
  ["AWS access key", /(?:AKIA|ASIA)[A-Z0-9]{16}/],
  ["GitHub token", /gh[pousr]_[A-Za-z0-9]{30,}/],
  ["OpenAI token", /sk-(?:proj-)?[A-Za-z0-9_-]{20,}/],
  ["Slack token", /xox[baprs]-[A-Za-z0-9-]{10,}/],
  ["Stripe live key", /[rs]k_live_[A-Za-z0-9]{16,}/],
  ["JWT", /eyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}/],
  ["credential URL", /[A-Za-z][A-Za-z0-9+.-]*:\/\/[^/@\s]+:[^/@\s]+@/],
] as const;
const sensitiveName =
  /(?:^|\/)(?:\.env(?:\..+)?|credentials?|secrets?)(?:$|\.)|\.(?:jks|kdbx|key|keystore|p12|pem|pfx)$/i;
const environmentExample = /(?:^|\/)\.env(?:\.[^/]+)?\.example$/;

const tracked = Bun.spawnSync([
  "git",
  "ls-files",
  "--cached",
  "--others",
  "--exclude-standard",
  "-z",
], {
  stdout: "pipe",
  stderr: "pipe",
});
if (tracked.exitCode !== 0) {
  throw new Error("Unable to enumerate tracked files with git ls-files.");
}

const paths = new TextDecoder()
  .decode(tracked.stdout)
  .split("\0")
  .filter(Boolean);

// Cargo.lock is ignored for library crates, but the two GPUI graph lockfiles
// are reviewed source evidence even when a local checkout generated one
// before it was staged. Keep their exact Git pairs inside this audit surface.
for (const lockPath of [
  "packages/gpui/node-backend/Cargo.lock",
  "packages/gpui/preview/Cargo.lock",
]) {
  if (existsSync(lockPath) && !paths.includes(lockPath)) paths.push(lockPath);
}

for (const path of paths) {
  // `git ls-files --cached` includes paths deleted in an uncommitted change.
  // Audit the working tree that would be committed without crashing on them.
  if (!existsSync(path)) continue;

  if (sensitiveName.test(path) && !environmentExample.test(path)) {
    errors.push(`${path}: credential-like filename is tracked`);
  }

  const bytes = readFileSync(path);
  if (bytes.includes(0)) continue;
  const source = bytes.toString("utf8");
  for (const [label, pattern] of secretPatterns) {
    if (pattern.test(source)) errors.push(`${path}: contains a ${label} pattern`);
  }

  if (basename(path) === "Cargo.toml") {
    errors.push(...validateCargoManifestSources(path, source));
  }

  if (basename(path) === "Cargo.lock") {
    errors.push(...validateCargoLockSources(path, source));
  }

  if (basename(path) !== "package.json") continue;
  const manifest = JSON.parse(source) as {
    scripts?: Record<string, string>;
    dependencies?: Record<string, string>;
    devDependencies?: Record<string, string>;
    optionalDependencies?: Record<string, string>;
    peerDependencies?: Record<string, string>;
  };
  for (const hook of Object.keys(manifest.scripts ?? {})) {
    if (lifecycleHooks.has(hook)) {
      errors.push(`${path}: declares lifecycle hook "${hook}"`);
    }
  }
  for (const dependencies of [
    manifest.dependencies,
    manifest.devDependencies,
    manifest.optionalDependencies,
    manifest.peerDependencies,
  ]) {
    for (const [name, source] of Object.entries(dependencies ?? {})) {
      if (/^(?:git\+|github:|https?:\/\/)/.test(source)) {
        errors.push(`${path}: dependency ${name} uses a remote source`);
      }
    }
  }
}

if (errors.length > 0) {
  throw new Error(errors.join("\n"));
}

console.log(
  `Security hygiene clean: ${paths.length} repository files, no credential patterns, lifecycle hooks, or remote dependencies.`,
);
