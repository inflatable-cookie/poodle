/**
 * Runs every member of an Effigy task list, one at a time and past any failure,
 * then reports each result and exits non-zero if any failed. An Effigy task
 * list stops at its first failure, which would hide every probe after it.
 *
 *   bun test/browser-probe/run-all.ts <task list>
 */
import { fileURLToPath } from "node:url";

type Step = { task?: string };

const REPO_ROOT = fileURLToPath(new URL("../..", import.meta.url));
const manifest = Bun.TOML.parse(await Bun.file(`${REPO_ROOT}/tasks/effigy.tasks.toml`).text()) as {
  tasks: Record<string, unknown>;
};

/** Expands nested lists too, so one engine's failure cannot skip the other. */
function expand(name: string): string[] {
  const task = manifest.tasks[name];
  if (task === undefined) throw new Error(`Unknown task ${name}`);
  if (!Array.isArray(task)) return [name];
  return (task as Step[]).flatMap((step) => {
    if (step.task === undefined) throw new Error(`${name}: every member must be a task reference`);
    return expand(step.task);
  });
}

const list = process.argv[2];
if (list === undefined) throw new Error("Usage: run-all.ts <task list>");
const selectors = expand(list);
if (selectors.length === 0) throw new Error(`${list} has no members`);

const results: Array<{ selector: string; code: number; seconds: number }> = [];
for (const selector of selectors) {
  const started = Date.now();
  const child = Bun.spawn(["effigy", selector], { cwd: REPO_ROOT, stdio: ["inherit", "inherit", "inherit"] });
  const code = await child.exited;
  results.push({ selector, code, seconds: Math.round((Date.now() - started) / 1000) });
}

const failed = results.filter((result) => result.code !== 0);
console.log(`\n${list}: ${results.length - failed.length} of ${results.length} passed`);
for (const { selector, code, seconds } of results) {
  console.log(`  ${code === 0 ? "ok  " : "FAIL"}  ${selector}  (${seconds}s${code === 0 ? "" : `, exit ${code}`})`);
}
process.exit(failed.length > 0 ? 1 : 0);
