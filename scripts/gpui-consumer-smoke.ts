#!/usr/bin/env bun
/** Build a clean downstream GPUI consumer pinned to this commit's local tag. */

import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

function run(command: string, args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): string {
  const result = spawnSync(command, args, {
    cwd,
    env,
    encoding: "utf8",
    maxBuffer: 32 * 1024 * 1024,
  });
  if (result.error) throw result.error;
  if (result.status !== 0) {
    throw new Error(
      `${command} ${args.join(" ")} failed (${result.status ?? "signal"})\n${result.stdout}\n${result.stderr}`,
    );
  }
  return result.stdout;
}

function main(): void {
  const repo = process.cwd();
  const status = run("git", ["status", "--porcelain"], repo).trim();
  if (status) throw new Error("consumer smoke requires a clean release-candidate worktree");
  const sourceCommit = run("git", ["rev-parse", "HEAD"], repo).trim();
  const rootManifest = JSON.parse(readFileSync(join(repo, "package.json"), "utf8")) as {
    version?: string;
  };
  const version = rootManifest.version;
  if (!version || !/^0\.\d+\.\d+$/.test(version)) {
    throw new Error(`root package.json must declare a pre-1.0 release version, found ${String(version)}`);
  }

  const temp = mkdtempSync(join(tmpdir(), "poodle-gpui-consumer-"));
  try {
    const mirror = join(temp, "poodle.git");
    const consumer = join(temp, "consumer");
    const target = join(temp, "target");
    mkdirSync(consumer, { recursive: true });
    run("git", ["init", "--bare", mirror], repo);
    run("git", ["push", mirror, `${sourceCommit}:refs/heads/candidate`], repo);
    const tag = `v${version}`;
    run("git", ["--git-dir", mirror, "tag", tag, sourceCommit], repo);

    const poodleUrl = pathToFileURL(mirror).href;
    writeFileSync(
      join(consumer, "Cargo.toml"),
      [
        "[package]",
        'name = "poodle-gpui-release-smoke"',
        'version = "0.1.0"',
        'edition = "2021"',
        "",
        "[lib]",
        'path = "lib.rs"',
        "",
        "[dependencies]",
        'gpui = { package = "gpui-unofficial", version = "=1.22.0" }',
        `poodle-node = { git = "${poodleUrl}", tag = "${tag}", package = "poodle-node" }`,
        `poodle-gpui-node-backend = { git = "${poodleUrl}", tag = "${tag}", package = "poodle-gpui-node-backend" }`,
        "",
      ].join("\n"),
    );
    writeFileSync(
      join(consumer, "lib.rs"),
      [
        "pub fn render(node: &poodle_node::Node) -> gpui::AnyElement {",
        "    poodle_gpui_node_backend::to_gpui(node)",
        "}",
        "",
      ].join("\n"),
    );
    run(
      "cargo",
      ["check", "--manifest-path", join(consumer, "Cargo.toml")],
      consumer,
      { ...process.env, CARGO_TARGET_DIR: target },
    );
    console.log(
      `Built a clean GPUI consumer against ${sourceCommit} via local tag ${tag}; gpui-unofficial =1.22.0 identity compiled.`,
    );
  } finally {
    rmSync(temp, { recursive: true, force: true });
  }
}

try {
  main();
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
}
