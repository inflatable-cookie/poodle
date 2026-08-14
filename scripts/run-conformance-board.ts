/**
 * Keep the stable conformance CI entrypoint safe for an operator's desktop.
 * GitHub's isolated macOS runner can execute the foreground AppKit boundary;
 * local callers receive the headless board unless they name the windowed
 * selector themselves.
 */

import { spawnSync } from "node:child_process";

const selector =
  process.env.GITHUB_ACTIONS === "true"
    ? "ci:conformance-windowed"
    : "ci:conformance-headless";

console.log(`ci:conformance -> ${selector}`);
const result = spawnSync("effigy", [selector], { stdio: "inherit" });

if (result.error) throw result.error;
process.exit(result.status ?? 1);
