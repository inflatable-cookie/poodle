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
};

export type BoardUnit = {
  label: string;
  name: string | null;
  command: string;
  env: Record<string, string>;
  timeoutMs: number;
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

// A task key is either a quoted key at any indentation or a bare key at column
// zero. Indented bare keys are inline-table fields (`run`, `env`) and must not
// be mistaken for tasks.
const TASK_ASSIGNMENT = /^(?:[ \t]*"([A-Za-z0-9:_-]+)"|([A-Za-z0-9:_-]+))[ \t]*=[ \t]*/gm;

function stripComments(text: string): string {
  // Comments in this manifest always start a line; run commands never carry a
  // shell comment in the shapes this repo uses.
  return text
    .split("\n")
    .map((line) => (line.trimStart().startsWith("#") ? "" : line))
    .join("\n");
}

function unescapeTomlString(value: string): string {
  return value.replace(/\\(["\\])/g, "$1");
}

function parseStringValue(text: string): string | null {
  const match = /^\s*"((?:[^"\\]|\\.)*)"/.exec(text);
  return match ? unescapeTomlString(match[1]) : null;
}

function extractBracedBlock(text: string, openIndex: number): string | null {
  if (text[openIndex] !== "{") return null;
  let depth = 0;
  let inString = false;
  for (let index = openIndex; index < text.length; index += 1) {
    const char = text[index];
    if (inString) {
      if (char === "\\") index += 1;
      else if (char === '"') inString = false;
      continue;
    }
    if (char === '"') {
      inString = true;
      continue;
    }
    if (char === "{") depth += 1;
    if (char === "}") {
      depth -= 1;
      if (depth === 0) return text.slice(openIndex + 1, index);
    }
  }
  return null;
}

function parseInlineTable(table: string): Step | null {
  const task = /task\s*=\s*"((?:[^"\\]|\\.)*)"/.exec(table);
  if (task) return { kind: "task", task: unescapeTomlString(task[1]) };
  const run = /run\s*=\s*"((?:[^"\\]|\\.)*)"/.exec(table);
  if (!run) return null;
  const env: Record<string, string> = {};
  const envKey = /env\s*=\s*/.exec(table);
  if (envKey) {
    const openIndex = envKey.index + envKey[0].length;
    const block = extractBracedBlock(table, openIndex);
    if (block !== null) {
      for (const entry of block.matchAll(/([A-Za-z_][A-Za-z0-9_]*)\s*=\s*"((?:[^"\\]|\\.)*)"/g)) {
        env[entry[1]] = unescapeTomlString(entry[2]);
      }
    }
  }
  return { kind: "run", command: unescapeTomlString(run[1]), env };
}

function splitInlineTables(value: string): string[] {
  const tables: string[] = [];
  let depth = 0;
  let start = -1;
  let inString = false;
  for (let index = 0; index < value.length; index += 1) {
    const char = value[index];
    if (inString) {
      if (char === "\\") index += 1;
      else if (char === '"') inString = false;
      continue;
    }
    if (char === '"') {
      inString = true;
      continue;
    }
    if (char === "{") {
      if (depth === 0) start = index + 1;
      depth += 1;
      continue;
    }
    if (char === "}") {
      depth -= 1;
      if (depth === 0 && start !== -1) {
        tables.push(value.slice(start, index));
        start = -1;
      }
    }
  }
  return tables;
}

/** Parse the flat `tasks/effigy.tasks.toml` shape into task nodes. */
export function parseTaskGraph(source: string): Map<string, TaskNode> {
  const text = stripComments(source);
  const matches = [...text.matchAll(TASK_ASSIGNMENT)];
  const nodes = new Map<string, TaskNode>();
  for (let index = 0; index < matches.length; index += 1) {
    const match = matches[index];
    const name = match[1] ?? match[2];
    const start = (match.index ?? 0) + match[0].length;
    const end = index + 1 < matches.length ? matches[index + 1].index ?? text.length : text.length;
    const value = text.slice(start, end).trim();
    if (value.startsWith("[")) {
      const steps: Step[] = [];
      for (const table of splitInlineTables(value)) {
        const step = parseInlineTable(table);
        if (step) steps.push(step);
      }
      nodes.set(name, { name, kind: "composite", steps });
      continue;
    }
    if (value.startsWith("{")) {
      const step = parseInlineTable(value.replace(/\n/g, " "));
      if (!step) continue;
      if (step.kind === "task") {
        nodes.set(name, { name, kind: "composite", steps: [step] });
      } else {
        nodes.set(name, { name, kind: "command", command: step.command, env: step.env });
      }
      continue;
    }
    const command = parseStringValue(value);
    if (command === null) continue;
    nodes.set(name, { name, kind: "command", command, env: {} });
  }
  return nodes;
}

function substitute(value: string, root: string): string {
  return value.replaceAll("{repo}", root);
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
      emit(
        {
          label: `${name}::${step.command}`,
          name: null,
          command: substitute(step.command, root),
          env: substituteEnv(step.env),
          timeoutMs: bounds.childTimeoutMs,
        },
        `run:${step.command}`,
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
};

export function readBounds(path: string): ValidationBounds {
  if (!existsSync(path)) return DEFAULT_BOUNDS;
  const raw = JSON.parse(readFileSync(path, "utf8")) as Partial<ValidationBounds>;
  return {
    boardTimeoutMs: raw.boardTimeoutMs ?? DEFAULT_BOUNDS.boardTimeoutMs,
    childTimeoutMs: raw.childTimeoutMs ?? DEFAULT_BOUNDS.childTimeoutMs,
    tasks: raw.tasks ?? {},
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
  let timedOut = false;
  const timer = setTimeout(() => {
    timedOut = true;
    try {
      process.kill(-child.pid, "SIGKILL");
    } catch {
      child.kill("SIGKILL");
    }
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
