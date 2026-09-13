// g18.032 / spec 071: focused laws for the bounded, observable validation
// runner. A planted hang must name the child, be killed with its descendants
// and be reported as a process failure instead of waiting.

import { afterAll, describe, expect, test } from "bun:test";
import { existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import {
  collectUnits,
  parseTaskGraph,
  readBounds,
  runBoard,
  type ValidationBounds,
} from "./run-board";

const roots: string[] = [];

afterAll(() => {
  for (const root of roots) rmSync(root, { recursive: true, force: true });
});

function tempRoot(): string {
  const root = mkdtempSync(join(tmpdir(), "poodle-validation-runner-test-"));
  roots.push(root);
  return root;
}

const bounds = (overrides: Partial<ValidationBounds> = {}): ValidationBounds => ({
  boardTimeoutMs: 15 * 60 * 1000,
  childTimeoutMs: 5 * 60 * 1000,
  tasks: {},
  ...overrides,
});

function alive(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch {
    return false;
  }
}

describe("validation board repository inventory", () => {
  const repoRoot = new URL("../..", import.meta.url).pathname;
  const nodes = parseTaskGraph(readFileSync(join(repoRoot, "tasks/effigy.tasks.toml"), "utf8"));
  const policy = readBounds(join(repoRoot, "quality/validation-bounds.json"));

  test("every owned unit executes exactly once and stays bounded", () => {
    const { units } = collectUnits(nodes, "qa:board", repoRoot, policy);
    const labels = units.map((unit) => unit.label);
    expect(new Set(labels).size).toBe(labels.length);
    expect(units.length).toBeGreaterThan(40);
    for (const unit of units) {
      expect(unit.timeoutMs).toBeLessThanOrEqual(policy.childTimeoutMs);
      expect(unit.timeoutMs).toBeGreaterThan(0);
    }
  });

  test("the named specimen hang carries an explicit smaller bound", () => {
    expect(policy.tasks["probe:gpui-specimens"]).toBeLessThanOrEqual(180_000);
    expect(policy.boardTimeoutMs).toBeLessThanOrEqual(15 * 60 * 1000);
    const { units } = collectUnits(nodes, "qa:board", repoRoot, policy);
    const specimen = units.find((unit) => unit.name === "probe:gpui-specimens");
    expect(specimen?.timeoutMs).toBe(180_000);
  });

  test("the npm release gate is not the aggregate board", () => {
    const release = collectUnits(nodes, "release:web-certificate", repoRoot, policy);
    const labels = release.units.map((unit) => unit.label);
    expect(labels).toEqual(["release:web-admission", "release:web-archive"]);
  });
});

describe("validation board graph", () => {
  test("parses quoted keys, bare keys, string, array and inline-table shapes", () => {
    const nodes = parseTaskGraph(`
# comment
"a:one" = "echo one"
b = [
  { task = "a:one" },
  { run = "echo two", env = { FOO = "{repo}/foo" } },
]
"c:three" = { run = "echo three", env = { BAR = "baz" } }
`);
    expect(nodes.get("a:one")).toMatchObject({ kind: "command", command: "echo one" });
    expect(nodes.get("b")).toMatchObject({ kind: "composite" });
    expect(nodes.get("c:three")).toMatchObject({
      kind: "command",
      command: "echo three",
      env: { BAR: "baz" },
    });
    const composite = nodes.get("b");
    expect(composite?.kind === "composite" && composite.steps.length).toBe(2);
  });

  test("deduplicates transitive repeats and keeps first-occurrence order", () => {
    const nodes = parseTaskGraph(`
"leaf" = "echo leaf"
"left" = [{ task = "leaf" }, { run = "echo left" }]
"right" = [{ task = "leaf" }, { run = "echo right" }]
"board" = [{ task = "left" }, { task = "right" }]
`);
    const { units, reused } = collectUnits(nodes, "board", "/repo", bounds());
    const labels = units.map((unit) => unit.label);
    expect(labels).toEqual(["leaf", "left::echo left", "right::echo right"]);
    expect(reused).toEqual(["leaf"]);
  });

  test("bounds come from policy per owned unit", () => {
    const nodes = parseTaskGraph('"slow" = "echo slow"\n"board" = [{ task = "slow" }]\n');
    const { units } = collectUnits(
      nodes,
      "board",
      "/repo",
      bounds({ childTimeoutMs: 1000, tasks: { slow: 42 } }),
    );
    expect(units[0].timeoutMs).toBe(42);
  });

  test("keeps every env entry and substitutes {repo}", () => {
    const nodes = parseTaskGraph(
      '"leaf" = { run = "true", env = { FIRST = "1", SECOND = "{repo}/out" } }\n"board" = [{ task = "leaf" }]\n',
    );
    const { units } = collectUnits(nodes, "board", "/repo", bounds());
    expect(units[0].env).toEqual({ FIRST: "1", SECOND: "/repo/out" });
  });
});

describe("validation board process lifecycle", () => {
  test("a planted hang names the child and is killed with its descendants", async () => {
    const root = tempRoot();
    const pidFile = join(root, "hang.pid");
    const tasksPath = join(root, "tasks.toml");
    const boundsPath = join(root, "bounds.json");
    await Bun.write(
      tasksPath,
      [
        `"hang" = "sleep 30 & echo $! > ${pidFile}; wait"`,
        `"board" = [{ task = "hang" }]`,
        "",
      ].join("\n"),
    );
    await Bun.write(boundsPath, JSON.stringify({ boardTimeoutMs: 10000, childTimeoutMs: 1500, tasks: {} }));
    const child = Bun.spawn(
      ["bun", "scripts/validation/run-board.ts", "--root", root, "--tasks", tasksPath, "--bounds", boundsPath, "--json", "board"],
      { cwd: process.cwd(), stdout: "pipe", stderr: "pipe" },
    );
    const [stdout, stderr, exitCode] = await Promise.all([
      new Response(child.stdout).text(),
      new Response(child.stderr).text(),
      child.exited,
    ]);
    expect(exitCode).toBe(1);
    expect(stderr).toContain("hang");
    expect(stderr).toContain("killed after");
    expect(stderr).toContain("child bound");
    expect(existsSync(pidFile)).toBe(true);
    const grandchildPid = Number(readFileSync(pidFile, "utf8").trim());
    expect(Number.isSafeInteger(grandchildPid)).toBe(true);
    expect(alive(grandchildPid)).toBe(false);
    const summary = JSON.parse(stdout);
    expect(summary.schema).toBe("poodle.validation-board.v1");
    expect(summary.ok).toBe(false);
    expect(summary.failure).toContain("hang");
    expect(summary.units[0].status).toBe("timeout");
  });

  test("the board cap stops the run and reports the ceiling", async () => {
    const nodes = parseTaskGraph('"hang" = "sleep 30"\n"board" = [{ task = "hang" }]\n');
    const summary = await runBoard({
      nodes,
      selector: "board",
      root: process.cwd(),
      bounds: bounds({ boardTimeoutMs: 800, childTimeoutMs: 60_000 }),
      log: () => {},
    });
    expect(summary.ok).toBe(false);
    expect(summary.failure).toContain("hang");
    expect(summary.units[0].status).toBe("timeout");
    expect(summary.elapsedMs).toBeLessThan(10_000);
  });

  test("a passing board reports reused work and the slowest leaves", async () => {
    const nodes = parseTaskGraph(`
"shared" = "true"
"board" = [{ task = "shared" }, { run = "true" }, { task = "shared" }]
`);
    const summary = await runBoard({
      nodes,
      selector: "board",
      root: process.cwd(),
      bounds: bounds({ boardTimeoutMs: 10_000, childTimeoutMs: 5_000 }),
      log: () => {},
    });
    expect(summary.ok).toBe(true);
    expect(summary.reused).toEqual(["shared"]);
    expect(summary.slowest.length).toBeGreaterThan(0);
    expect(summary.schema).toBe("poodle.validation-board.v1");
  });
});
