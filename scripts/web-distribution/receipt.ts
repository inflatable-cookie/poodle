import { spawnSync } from "node:child_process";
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, posix, relative } from "node:path";

import { sha256File, stableJson } from "./hash";
import type { BuildReceipt, PackageBuildSpec, ReceiptOutput } from "./types";

const RECEIPT_NAME = ".poodle-build.json";

function sourceCommit(repoRoot: string): string {
  const result = spawnSync("git", ["rev-parse", "HEAD"], {
    cwd: repoRoot,
    encoding: "utf8",
  });
  if (result.status !== 0) {
    throw new Error(`git rev-parse HEAD failed: ${result.stderr}`);
  }
  const commit = result.stdout.trim().toLowerCase();
  if (!/^[0-9a-f]{40}$/.test(commit)) {
    throw new Error(`sourceCommit must be 40-char lowercase hex, got ${commit}`);
  }
  return commit;
}

function walk(root: string): string[] {
  const files: string[] = [];
  const visit = (directory: string) => {
    for (const entry of readdirSync(directory).sort()) {
      const full = join(directory, entry);
      if (statSync(full).isDirectory()) visit(full);
      else files.push(full);
    }
  };
  if (statSync(root).isDirectory()) visit(root);
  return files;
}

function packageRelative(packageRoot: string, abs: string): string {
  return relative(packageRoot, abs).split("\\").join("/");
}

export function collectInputs(packageRoot: string, spec: PackageBuildSpec): string[] {
  const inputs = new Set<string>();
  for (const entry of spec.entries) inputs.add(entry.source);
  for (const asset of spec.assets) inputs.add(asset.from);
  for (const extra of spec.extraDeclarationCopies ?? []) inputs.add(extra.from);
  for (const abs of walk(join(packageRoot, "src"))) {
    const rel = packageRelative(packageRoot, abs);
    if (rel.endsWith(".js") && !rel.endsWith(".mjs")) continue;
    if (/\.(?:[cm]?tsx?|mjs|css)$/.test(rel)) inputs.add(rel);
  }
  const sorted = [...inputs].sort();
  return sorted;
}

export function collectOutputs(packageRoot: string): ReceiptOutput[] {
  const distDir = join(packageRoot, "dist");
  const outputs: ReceiptOutput[] = [];
  for (const abs of walk(distDir)) {
    const rel = packageRelative(packageRoot, abs);
    if (rel === posix.join("dist", RECEIPT_NAME)) continue;
    outputs.push({ path: rel, sha256: sha256File(abs) });
  }
  outputs.sort((left, right) => (left.path < right.path ? -1 : left.path > right.path ? 1 : 0));
  return outputs;
}

export function writeReceipt(options: {
  repoRoot: string;
  packageRoot: string;
  spec: PackageBuildSpec;
  tools: BuildReceipt["tools"];
}): BuildReceipt {
  const receipt: BuildReceipt = {
    cssPolicy: options.spec.cssPolicy,
    inputs: collectInputs(options.packageRoot, options.spec),
    lanes: [...options.spec.lanes],
    markdownPolicy: options.spec.markdownPolicy,
    outputs: collectOutputs(options.packageRoot),
    package: options.spec.packageName,
    schemaVersion: 1,
    sourceCommit: sourceCommit(options.repoRoot),
    sourceMaps: false,
    tools: options.tools,
    version: options.spec.version,
  };
  const encoded = stableJson(receipt);
  if (encoded.includes("timestamp") || /\d{4}-\d{2}-\d{2}T/.test(encoded)) {
    throw new Error("receipt serialization introduced a timestamp");
  }
  if (encoded.includes(options.repoRoot) || encoded.includes("/Users/")) {
    throw new Error("receipt serialization introduced an absolute path");
  }
  writeFileSync(join(options.packageRoot, "dist", RECEIPT_NAME), encoded);
  return receipt;
}

export function receiptPath(packageRoot: string): string {
  return join(packageRoot, "dist", RECEIPT_NAME);
}

export function readReceipt(packageRoot: string): string {
  return readFileSync(receiptPath(packageRoot), "utf8");
}
