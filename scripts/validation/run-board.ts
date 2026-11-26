#!/usr/bin/env bun
// g18.032 / spec 071: the repository validation board runner.
//
// `effigy <board>` nests composite tasks and re-executes shared leaves once per
// reference (verified against Effigy 0.12.1). It also gives no live child
// output and no per-child execution bound. That combination let the `0.4.0`
// release sit inside `probe:gpui-specimens` for 2h37m with no progress.
//
// This runner expands the same `tasks/effigy.tasks.toml` graph, executes every
// owned unit exactly once, streams a start/completion/elapsed line per child,
// bounded-kills an over-budget child with its whole process group, and writes a
// machine-readable summary naming the slowest leaves, reused work and failure.
//
// It is deliberately a thin executor over the declared graph. It does not
// own selectors, rewrite commands, or decide policy.

import { existsSync, readFileSync } from "node:fs";
import { constants } from "node:os";
import { join, resolve } from "node:path";

export type Step =
  | { kind: "run"; command: string; env: Record<string, string> }
  | { kind: "task"; task: string };

export type TaskNode =
  | { name: string; kind: "command"; command: string; env: Record<string, string> }
  | { name: string; kind: "composite"; steps: Step[] };

export type ValidationBounds = {
  boardTimeoutMs: number;
  childTimeoutMs: number;
  tasks: Record<string, number>;
  // Task name -> marker file the task touches once it is actually running.
  // The child bound for that task starts when the marker appears, not when
  // `bash` is spawned; see `runUnit`.
  readyFiles?: Record<string, string>;
};

export type BoardUnit = {
  label: string;
  name: string | null;
  command: string;
  env: Record<string, string>;
  timeoutMs: number;
  readyFile: string | null;
  reused: boolean;
};

export type BoardSummary = {
  schema: "poodle.validation-board.v1";
  selector: string;
  ok: boolean;
  boardBudgetMs: number;
  elapsedMs: number;
  unitCount: number;
  reusedCount: number;
  units: { label: string; elapsedMs: number; status: string; timeoutMs: number }[];
  slowest: { label: string; elapsedMs: number }[];
  reused: string[];
  failure: string | null;
};

function isTable(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function parseStep(value: unknown, label: string): Step {
  if (typeof value === "string") return { kind: "run", command: value, env: {} };
  if (isTable(value)) {
    if (typeof value.task === "string" && !("run" in value)) {
      return { kind: "task", task: value.task };
    }
    if (typeof value.run === "string" && !("task" in value)) {
      const env = value.env;
      if (env === undefined) return { kind: "run", command: value.run, env: {} };
      if (!isTable(env)) throw new Error(`${label} env must be a table`);
      const entries = Object.entries(env).map(([key, entry]) => {
        if (typeof entry !== "string") throw new Error(`${label} env.${key} must be a string`);
        return [key, entry];
      });
      return { kind: "run", command: value.run, env: Object.fromEntries(entries) };
    }
  }
  throw new Error(`${label} must declare a run command or a task reference`);
}

/** Parse the Effigy task catalog into task nodes. */
export function parseTaskGraph(source: string): Map<string, TaskNode> {
  const manifest: unknown = Bun.TOML.parse(source);
  if (!isTable(manifest) || !isTable(manifest.tasks)) {
    throw new Error("validation board manifest must declare a [tasks] table");
  }
  const nodes = new Map<string, TaskNode>();
  for (const [name, value] of Object.entries(manifest.tasks)) {
    const label = `validation board task ${name}`;
    if (Array.isArray(value)) {
      const steps = value.map((step, index) => parseStep(step, `${label} step ${index + 1}`));
      nodes.set(name, { name, kind: "composite", steps });
      continue;
    }
    const step = parseStep(value, label);
    if (step.kind === "task") nodes.set(name, { name, kind: "composite", steps: [step] });
    else nodes.set(name, { name, kind: "command", command: step.command, env: step.env });
  }
  return nodes;
}

function substitute(value: string, root: string): string {
  // Shell parameter expansions (${NAME}) belong to bash.
  return value.replace(/(?<!\$)\{([A-Za-z_][\w:-]*)\}/g, (placeholder, name: string) => {
    if (name === "repo") return root;
    if (name === "args") return "";
    throw new Error(`unknown validation board placeholder ${placeholder} in ${JSON.stringify(value)}`);
  });
}

/**
 * Flatten a board into its owned units. A named command task is a unit; a
 * composite contributes its task steps recursively and its own run steps
 * inline. Identical named tasks and identical inline commands are executed
 * once and reported as reused.
 */
export function collectUnits(
  nodes: Map<string, TaskNode>,
  selector: string,
  root: string,
  bounds: ValidationBounds,
): { units: BoardUnit[]; reused: string[] } {
  const units: BoardUnit[] = [];
  const reused: string[] = [];
  const seenNames = new Set<string>();
  const seenCommands = new Set<string>();

  const substituteEnv = (env: Record<string, string>): Record<string, string> =>
    Object.fromEntries(
      Object.entries(env).map(([key, value]) => [key, substitute(value, root)]),
    );

  const emit = (unit: Omit<BoardUnit, "reused">, key: string, seen: Set<string>): void => {
    if (seen.has(key)) {
      reused.push(unit.label);
      return;
    }
    seen.add(key);
    units.push({ ...unit, reused: false });
  };

  const visit = (name: string): void => {
    const node = nodes.get(name);
    if (!node) throw new Error(`validation board references unknown task ${name}`);
    if (node.kind === "command") {
      emit(
        {
          label: name,
          name,
          command: substitute(node.command, root),
          env: substituteEnv(node.env),
          timeoutMs: bounds.tasks[name] ?? bounds.childTimeoutMs,
          readyFile: bounds.readyFiles?.[name] ?? null,
        },
        `task:${name}`,
        seenNames,
      );
      return;
    }
    if (node.steps.length === 0) {
      throw new Error(`validation board task ${name} declares no steps`);
    }
    for (const step of node.steps) {
      if (step.kind === "task") {
        visit(step.task);
        continue;
      }
      const command = substitute(step.command, root);
      const env = substituteEnv(step.env);
      emit(
        {
          label: `${name}::${step.command}`,
          name: null,
          command,
          env,
          timeoutMs: bounds.childTimeoutMs,
          readyFile: null,
        },
        JSON.stringify([command, Object.entries(env).sort()]),
        seenCommands,
      );
    }
  };

  visit(selector);
  return { units, reused };
}

export const DEFAULT_BOUNDS: ValidationBounds = {
  boardTimeoutMs: 15 * 60 * 1000,
  childTimeoutMs: 5 * 60 * 1000,
  tasks: {},
  readyFiles: {},
};

export function readBounds(path: string): ValidationBounds {
  if (!existsSync(path)) return DEFAULT_BOUNDS;
  const raw = JSON.parse(readFileSync(path, "utf8")) as Partial<ValidationBounds>;
  return {
    boardTimeoutMs: raw.boardTimeoutMs ?? DEFAULT_BOUNDS.boardTimeoutMs,
    childTimeoutMs: raw.childTimeoutMs ?? DEFAULT_BOUNDS.childTimeoutMs,
    tasks: raw.tasks ?? {},
    readyFiles: raw.readyFiles ?? {},
  };
}

export function formatDuration(ms: number): string {
  if (ms < 1000) return `${Math.round(ms)}ms`;
  const seconds = ms / 1000;
  if (seconds < 60) return `${seconds.toFixed(1)}s`;
  const minutes = Math.floor(seconds / 60);
  return `${minutes}m${(seconds - minutes * 60).toFixed(0)}s`;
}

type ChildOutcome = { status: "ok" | "failed" | "timeout" | "board-timeout"; exitCode: number };

async function runUnit(
  unit: BoardUnit,
  root: string,
  remainingBoardMs: number,
  log: (line: string) => void,
): Promise<ChildOutcome> {
  const timeoutMs = Math.max(1, Math.min(unit.timeoutMs, remainingBoardMs));
  const child = Bun.spawn(["bash", "-lc", unit.command], {
    cwd: root,
    env: { ...globalThis.process.env, ...unit.env },
    stdout: "inherit",
    stderr: "inherit",
    // Own process group so a timeout kills every descendant the child owns.
    detached: true,
  });
  const killGroup = () => {
    try {
      process.kill(-child.pid, "SIGKILL");
    } catch {
      child.kill("SIGKILL");
    }
  };
  const interrupt = (signal: NodeJS.Signals) => {
    killGroup();
    process.exit(128 + constants.signals[signal]);
  };
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", interrupt);
  try {
    // A unit that declares `readyFile` does not arm its child bound until that
    // file appears. On a loaded host a login `bash` can still be starting when a
    // short bound fires, so the bound would time process start rather than the
    // work: run-board.test.ts's planted hang hit exactly that at 1500ms. Readiness
    // is itself bounded by the remaining board budget, so a task that never
    // signals is still killed and named.
    if (unit.readyFile !== null) {
      const deadline = Date.now() + remainingBoardMs;
      while (child.exitCode === null && !existsSync(unit.readyFile) && Date.now() < deadline) {
        await new Promise((resolve) => setTimeout(resolve, 20));
      }
      if (!existsSync(unit.readyFile)) {
        const ownExit = child.exitCode;
        killGroup();
        const killedExit = await child.exited;
        if (ownExit !== null && ownExit !== 0) {
          log(`  ✘ ${unit.label} failed with exit ${ownExit} before readiness`);
          return { status: "failed", exitCode: ownExit };
        }
        log(`  ✘ ${unit.label} killed before it signalled readiness (${unit.readyFile})`);
        return { status: "timeout", exitCode: killedExit };
      }
    }
    let timedOut = false;
    const timer = setTimeout(() => {
      timedOut = true;
      killGroup();
    }, timeoutMs);
    const exitCode = await child.exited;
    clearTimeout(timer);
    if (timedOut) {
      log(`  ✘ ${unit.label} killed after ${formatDuration(timeoutMs)} (child bound)`);
      return { status: "timeout", exitCode };
    }
    if (exitCode !== 0) {
      log(`  ✘ ${unit.label} failed with exit ${exitCode}`);
      return { status: "failed", exitCode };
    }
    return { status: "ok", exitCode };
  } finally {
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", interrupt);
  }
}

export async function runBoard(args: {
  nodes: Map<string, TaskNode>;
  selector: string;
  root: string;
  bounds: ValidationBounds;
  log?: (line: string) => void;
}): Promise<BoardSummary> {
  const log = args.log ?? ((line: string) => process.stderr.write(`${line}\n`));
  const { units, reused } = collectUnits(args.nodes, args.selector, args.root, args.bounds);
  log(
    `board ${args.selector}: ${units.length} owned units, ${reused.length} transitive repeats removed, ` +
      `board cap ${formatDuration(args.bounds.boardTimeoutMs)}`,
  );
  const started = performance.now();
  const results: BoardSummary["units"] = [];
  let failure: string | null = null;
  for (let index = 0; index < units.length; index += 1) {
    const unit = units[index];
    const elapsedBoard = performance.now() - started;
    const remainingBoardMs = args.bounds.boardTimeoutMs - elapsedBoard;
    if (remainingBoardMs <= 0) {
      failure = `board cap ${formatDuration(args.bounds.boardTimeoutMs)} exceeded before ${unit.label}`;
      log(`  ✘ ${failure}`);
      break;
    }
    const unitStarted = performance.now();
    log(`  ▶ [${index + 1}/${units.length}] ${unit.label} (bound ${formatDuration(unit.timeoutMs)})`);
    const outcome = await runUnit(unit, args.root, remainingBoardMs, log);
    const elapsedMs = performance.now() - unitStarted;
    log(`  ${outcome.status === "ok" ? "✔" : "✘"} ${unit.label} ${formatDuration(elapsedMs)}`);
    results.push({ label: unit.label, elapsedMs, status: outcome.status, timeoutMs: unit.timeoutMs });
    if (outcome.status !== "ok") {
      failure = `${unit.label} ${outcome.status}`;
      break;
    }
  }
  const elapsedMs = performance.now() - started;
  const summary: BoardSummary = {
    schema: "poodle.validation-board.v1",
    selector: args.selector,
    ok: failure === null,
    boardBudgetMs: args.bounds.boardTimeoutMs,
    elapsedMs,
    unitCount: units.length,
    reusedCount: reused.length,
    units: results,
    slowest: [...results]
      .sort((left, right) => right.elapsedMs - left.elapsedMs)
      .slice(0, 10)
      .map(({ label, elapsedMs: ms }) => ({ label, elapsedMs: ms })),
    reused,
    failure,
  };
  log(
    `${summary.ok ? "board passed" : "board failed"}: ${results.length}/${units.length} units in ${formatDuration(elapsedMs)}`,
  );
  if (summary.slowest.length > 0) {
    log(`slowest: ${summary.slowest.map(({ label, elapsedMs: ms }) => `${label} ${formatDuration(ms)}`).join(", ")}`);
  }
  return summary;
}

async function main(): Promise<void> {
  const argv = process.argv.slice(2);
  const options: Record<string, string> = {};
  const positionals: string[] = [];
  let json = false;
  let plan = false;
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--json") {
      json = true;
      continue;
    }
    if (arg === "--plan") {
      plan = true;
      continue;
    }
    if (arg === "--root" || arg === "--tasks" || arg === "--bounds") {
      options[arg] = argv[index + 1] ?? "";
      index += 1;
      continue;
    }
    positionals.push(arg);
  }
  const root = resolve(options["--root"] ?? process.cwd());
  const tasksPath = resolve(options["--tasks"] ?? join(root, "tasks/effigy.tasks.toml"));
  const boundsPath = resolve(options["--bounds"] ?? join(root, "quality/validation-bounds.json"));
  const selector = positionals.pop();
  if (!selector) {
    process.stderr.write(
      "usage: run-board.ts [--root DIR] [--tasks FILE] [--bounds FILE] [--json] <board-selector>\n",
    );
    process.exit(2);
  }
  if (!existsSync(tasksPath)) {
    process.stderr.write(`validation board tasks file not found: ${tasksPath}\n`);
    process.exit(2);
  }
  const nodes = parseTaskGraph(readFileSync(tasksPath, "utf8"));
  const bounds = readBounds(boundsPath);
  if (plan) {
    const { units, reused } = collectUnits(nodes, selector, root, bounds);
    process.stdout.write(
      `${JSON.stringify(
        { selector, units: units.map(({ label, timeoutMs }) => ({ label, timeoutMs })), reused },
        null,
        2,
      )}\n`,
    );
    return;
  }
  const summary = await runBoard({ nodes, selector, root, bounds });
  if (json) process.stdout.write(`${JSON.stringify(summary, null, 2)}\n`);
  if (!summary.ok) process.exit(1);
}

if (import.meta.main) {
  await main();
}
