import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";

export function emitDeclarations(options: {
  repoRoot: string;
  packageRoot: string;
  tsconfigPath: string;
}): void {
  const tsconfig = join(options.packageRoot, options.tsconfigPath);
  if (!existsSync(tsconfig)) {
    throw new Error(`missing declaration tsconfig ${options.tsconfigPath}`);
  }
  const result = spawnSync(
    "bun",
    ["x", "tsc", "-p", options.tsconfigPath, "--pretty", "false"],
    {
      cwd: options.packageRoot,
      encoding: "utf8",
    },
  );
  if (result.status !== 0) {
    throw new Error(
      `declaration emit failed:\n${result.stdout ?? ""}${result.stderr ?? ""}`,
    );
  }
}
