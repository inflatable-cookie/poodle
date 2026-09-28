import { afterAll, describe, expect, test } from "bun:test";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { materializeCertificationCheckout } from "./clean-checkout";
import { outputIncludes, stripAnsi } from "./strip-ansi";

const PACKED_TYPE_PROOF_DIAGNOSTIC =
  "error TS2339: Property 'branchCount' does not exist on type 'HistoryEntry'.";

const plantRoots: string[] = [];

afterAll(() => {
  for (const root of plantRoots) {
    rmSync(root, { recursive: true, force: true });
  }
});

async function runGit(root: string, args: string[]): Promise<string> {
  const child = Bun.spawn(["git", "-C", root, ...args], {
    stdout: "pipe",
    stderr: "pipe",
  });
  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(child.stdout).text(),
    new Response(child.stderr).text(),
    child.exited,
  ]);
  if (exitCode !== 0) {
    throw new Error(`git ${args.join(" ")} failed: ${stderr.trim()}`);
  }
  return stdout;
}

describe("packed tsc diagnostic colour", () => {
  test("a coloured diagnostic still matches the plain expected text", () => {
    const coloured =
      "\u001B[96merror\u001B[0m \u001B[90mTS2339: \u001B[0mProperty 'branchCount' does not exist on type 'HistoryEntry'.";
    expect(coloured.includes(PACKED_TYPE_PROOF_DIAGNOSTIC)).toBe(false);
    expect(outputIncludes(coloured, PACKED_TYPE_PROOF_DIAGNOSTIC)).toBe(true);
    expect(stripAnsi(coloured)).toContain("error TS2339:");
  });
});

describe("certification checkout from origin/main-only commits", () => {
  test("a commit reachable only through origin/main certifies", async () => {
    const root = mkdtempSync(join(tmpdir(), "poodle-web-pack-origin-main-"));
    plantRoots.push(root);
    await runGit(root, ["init", "--quiet", "--initial-branch=task"]);
    await runGit(root, ["config", "user.email", "poodle-certification@example.invalid"]);
    await runGit(root, ["config", "user.name", "Poodle Certification"]);

    await Bun.write(join(root, "shared.txt"), "base\n");
    await runGit(root, ["add", "shared.txt"]);
    await runGit(root, ["commit", "--quiet", "-m", "base"]);
    const base = (await runGit(root, ["rev-parse", "HEAD"])).trim();

    await Bun.write(join(root, "only-on-origin.txt"), "origin-only\n");
    await runGit(root, ["add", "only-on-origin.txt"]);
    await runGit(root, ["commit", "--quiet", "-m", "origin-only"]);
    const originMain = (await runGit(root, ["rev-parse", "HEAD"])).trim();
    await runGit(root, ["update-ref", "refs/remotes/origin/main", originMain]);
    await runGit(root, ["reset", "--hard", "--quiet", base]);

    const ancestorCheck = Bun.spawn(
      ["git", "-C", root, "merge-base", "--is-ancestor", originMain, "HEAD"],
      { stdout: "ignore", stderr: "ignore" },
    );
    expect(await ancestorCheck.exited).not.toBe(0);

    const absGitDir = (await runGit(root, ["rev-parse", "--absolute-git-dir"])).trim();
    const bareRoot = mkdtempSync(join(tmpdir(), "poodle-web-pack-bare-"));
    const checkoutRoot = mkdtempSync(join(tmpdir(), "poodle-web-pack-checkout-"));
    plantRoots.push(bareRoot, checkoutRoot);

    await materializeCertificationCheckout({
      sourceGitDir: absGitDir,
      bareRoot,
      checkoutRoot,
      proofCommit: originMain,
      requiredBaseCommit: base,
    });

    const checkedOut = (
      await runGit(checkoutRoot, ["rev-parse", "HEAD"])
    ).trim();
    expect(checkedOut).toBe(originMain);
    const planted = await Bun.file(join(checkoutRoot, "only-on-origin.txt")).text();
    expect(planted).toBe("origin-only\n");
  });
});
