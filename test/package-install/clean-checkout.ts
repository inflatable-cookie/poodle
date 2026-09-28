/**
 * Materialise the disposable certification checkout used by
 * `test/package-install/web-preview.ts`.
 *
 * A bare clone of the shared git dir copies objects but does not advertise
 * remote-tracking refs. `git fetch <git-dir> <sha>` then updates FETCH_HEAD,
 * which the subsequent `--no-local` clone does not advertise, so a commit
 * reachable only through `origin/main` checks out with `unable to read tree`.
 * Fetch `origin/main` and the candidate SHAs onto advertised heads so the
 * second clone transfers their trees. A non-mirror clone does not copy
 * `refs/remotes/*` or FETCH_HEAD.
 */

async function run(command: string[], cwd: string): Promise<void> {
  const child = Bun.spawn(command, {
    cwd,
    stdout: "ignore",
    stderr: "pipe",
  });
  const [stderr, exitCode] = await Promise.all([
    new Response(child.stderr).text(),
    child.exited,
  ]);
  if (exitCode !== 0) {
    throw new Error(`Command failed (${exitCode}): ${command.join(" ")}\n${stderr.trim()}`);
  }
}

export async function materializeCertificationCheckout(options: {
  sourceGitDir: string;
  bareRoot: string;
  checkoutRoot: string;
  proofCommit: string;
  requiredBaseCommit: string;
}): Promise<void> {
  const { sourceGitDir, bareRoot, checkoutRoot, proofCommit, requiredBaseCommit } = options;
  await run(["git", "clone", "--quiet", "--bare", sourceGitDir, bareRoot], sourceGitDir);
  await run(
    [
      "git",
      "-C",
      bareRoot,
      "fetch",
      "--quiet",
      "--upload-pack",
      "git -c uploadpack.allowReachableSHA1InWant=true upload-pack",
      sourceGitDir,
      "+refs/remotes/origin/main:refs/heads/poodle-origin-main",
      `+${requiredBaseCommit}:refs/heads/poodle-web-pack-base`,
      `+${proofCommit}:refs/heads/poodle-web-pack-head`,
    ],
    sourceGitDir,
  );
  await run(["git", "clone", "--quiet", "--no-local", bareRoot, checkoutRoot], sourceGitDir);
  await run(["git", "checkout", "--quiet", "--detach", proofCommit], checkoutRoot);
}
