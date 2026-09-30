import { spawnSync } from "node:child_process";

type LsofResult = {
  status: number | null;
  signal: NodeJS.Signals | null;
  error?: Error;
  stdout: string | null;
  stderr: string | null;
};

type LsofSpawn = (
  command: "lsof",
  args: string[],
  options: { encoding: "utf8" },
) => LsofResult;

function parsePids(stdout: string): number[] {
  return stdout
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean)
    .map(Number)
    .filter((pid) => Number.isInteger(pid) && pid > 0);
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

/** Read listening process IDs, failing closed if lsof cannot complete its scan. */
export function listeningPidsOnPort(port: number, spawn: LsofSpawn = spawnSync): number[] {
  let failure = "lsof returned no exit status, signal, or spawn error";

  for (let attempt = 0; attempt < 2; attempt += 1) {
    let result: LsofResult;
    try {
      result = spawn(
        "lsof",
        ["-nP", `-iTCP:${port}`, "-sTCP:LISTEN", "-t"],
        { encoding: "utf8" },
      );
    } catch (error) {
      failure = errorMessage(error);
      continue;
    }

    if (result.error || result.status === null || result.signal !== null) {
      failure = result.error
        ? errorMessage(result.error)
        : result.signal !== null
          ? `terminated by signal ${result.signal}`
          : "returned status null without a spawn error";
      continue;
    }

    if (result.status === 0) return parsePids(result.stdout ?? "");
    if (result.status === 1 && !(result.stdout ?? "").trim()) return [];

    failure =
      (result.stderr ?? "").trim() ||
      `exited with status ${result.status}${(result.stdout ?? "").trim() ? ` and output: ${(result.stdout ?? "").trim()}` : ""}`;
  }

  throw new Error(`lsof failed after retry while checking port ${port}: ${failure}`);
}
