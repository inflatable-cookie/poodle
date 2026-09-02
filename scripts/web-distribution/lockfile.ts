import { readFileSync } from "node:fs";
import { join } from "node:path";

import type { ReceiptTools } from "./types";

const LOCKED_PACKAGE = /^ {4}"([^"]+)": \["\1@([^"]+)"/;

export function readLockedTools(repoRoot: string): ReceiptTools {
  const lockfile = readFileSync(join(repoRoot, "bun.lock"), "utf8");
  const versions = new Map<string, string>();
  for (const line of lockfile.split("\n")) {
    const match = LOCKED_PACKAGE.exec(line);
    if (match) versions.set(match[1], match[2]);
  }

  const svelte = versions.get("svelte");
  const typescript = versions.get("typescript");
  const vite = versions.get("vite");
  if (!svelte || !typescript || !vite) {
    throw new Error(
      `bun.lock is missing locked tools (svelte=${svelte}, typescript=${typescript}, vite=${vite})`,
    );
  }
  return { svelte, typescript, vite };
}
