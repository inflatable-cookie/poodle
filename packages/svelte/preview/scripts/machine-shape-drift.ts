// Machine-shape drift gate (g13-047 R5).
//
// Two rules, both halves of the card:
//
//  1. Pinning: a machine present in BOTH runtimes
//     (`packages/core/src/*.ts` and `packages/contracts/headless/src/*.rs`)
//     must be covered by a shared conformance vector that BOTH harnesses
//     execute. A vector one side runs is not a pin.
//
//  2. Convention: a machine module that declares a transition must follow the
//     documented shape (g11.002, "Machine Shape Convention" section): the
//     stateful form (State + Context + Event + Effect + TransitionResult /
//     `(State, Vec<Effect>)`) or the trivial-case form (single implicit
//     state, value in context; no State type). Modules that are correctly
//     different are baselined below, each with a reason.
//
// The convention statement is in
// `docs/roadmaps/g11/002-headless-machine-spec-format-and-pilot-contracts.md`.
// Closing a drift means deleting its baseline entry.

import { readFileSync, existsSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../..");
const vectorsDir = path.join(repoRoot, "packages/contracts/headless/vectors");
const tsSrcDir = path.join(repoRoot, "packages/core/src");
const rsSrcDir = path.join(repoRoot, "packages/contracts/headless/src");

/**
 * Machines present in both runtimes. Key: canonical name (TS spelling,
 * snake_case-free). Value: the vector file that pins it and the runner
 * evidence each harness must contain.
 */
const PINNED: Record<
  string,
  { file: string; tsRunner: string; tsEvidence: string; rsRunner: string; rsEvidence: string }
> = {
  checkbox: {
    file: "machines.json",
    tsRunner: "packages/core/test/conformance.test.ts",
    tsEvidence: 'case "checkbox"',
    rsRunner: "packages/contracts/headless/tests/conformance.rs",
    rsEvidence: "fn checkbox_conformance",
  },
  disclosure: {
    file: "machines.json",
    tsRunner: "packages/core/test/conformance.test.ts",
    tsEvidence: 'case "disclosure"',
    rsRunner: "packages/contracts/headless/tests/conformance.rs",
    rsEvidence: "fn disclosure_conformance",
  },
  hover: {
    file: "machines.json",
    tsRunner: "packages/core/test/conformance.test.ts",
    tsEvidence: 'case "hover"',
    rsRunner: "packages/contracts/headless/tests/conformance.rs",
    rsEvidence: "fn hover_conformance",
  },
  menu: {
    file: "machines.json",
    tsRunner: "packages/core/test/conformance.test.ts",
    tsEvidence: 'case "menu"',
    rsRunner: "packages/contracts/headless/tests/conformance.rs",
    rsEvidence: "fn menu_conformance",
  },
  modal: {
    file: "machines.json",
    tsRunner: "packages/core/test/conformance.test.ts",
    tsEvidence: 'case "modal"',
    rsRunner: "packages/contracts/headless/tests/conformance.rs",
    rsEvidence: "fn modal_conformance",
  },
  singleSelect: {
    file: "machines.json",
    tsRunner: "packages/core/test/conformance.test.ts",
    tsEvidence: 'case "singleSelect"',
    rsRunner: "packages/contracts/headless/tests/conformance.rs",
    rsEvidence: "fn single_select_conformance",
  },
  select: {
    file: "machines.json",
    tsRunner: "packages/core/test/conformance.test.ts",
    tsEvidence: 'case "select"',
    rsRunner: "packages/contracts/headless/tests/conformance.rs",
    rsEvidence: "fn select_conformance",
  },
  slider: {
    file: "machines.json",
    tsRunner: "packages/core/test/conformance.test.ts",
    tsEvidence: 'case "slider"',
    rsRunner: "packages/contracts/headless/tests/conformance.rs",
    rsEvidence: "fn slider_conformance",
  },
  tabs: {
    file: "machines.json",
    tsRunner: "packages/core/test/conformance.test.ts",
    tsEvidence: 'case "tabs"',
    rsRunner: "packages/contracts/headless/tests/conformance.rs",
    rsEvidence: "fn tabs_conformance",
  },
  toggleGroup: {
    file: "machines.json",
    tsRunner: "packages/core/test/conformance.test.ts",
    tsEvidence: 'case "toggleGroup"',
    rsRunner: "packages/contracts/headless/tests/conformance.rs",
    rsEvidence: "fn toggle_group_conformance",
  },
  dragDrop: {
    file: "machines.json",
    tsRunner: "packages/core/test/conformance.test.ts",
    tsEvidence: "dragDrop",
    rsRunner: "packages/contracts/headless/tests/conformance.rs",
    rsEvidence: "fn drag_drop_conformance",
  },
  switch: {
    file: "machines.json",
    tsRunner: "packages/core/test/conformance.test.ts",
    tsEvidence: 'case "switch"',
    rsRunner: "packages/contracts/headless/tests/conformance.rs",
    rsEvidence: "fn switch_conformance",
  },
  color: {
    file: "domain.json",
    tsRunner: "packages/core/test/domain-conformance.test.ts",
    tsEvidence: "vectors.color",
    rsRunner: "packages/contracts/headless/tests/domain_conformance.rs",
    rsEvidence: "fn color_conformance",
  },
  date: {
    file: "domain.json",
    tsRunner: "packages/core/test/domain-conformance.test.ts",
    tsEvidence: "vectors.date",
    rsRunner: "packages/contracts/headless/tests/domain_conformance.rs",
    rsEvidence: "fn date_conformance",
  },
  duration: {
    file: "domain.json",
    tsRunner: "packages/core/test/domain-conformance.test.ts",
    tsEvidence: "vectors.duration",
    rsRunner: "packages/contracts/headless/tests/domain_conformance.rs",
    rsEvidence: "fn duration_conformance",
  },
  nav: {
    file: "domain.json",
    tsRunner: "packages/core/test/domain-conformance.test.ts",
    tsEvidence: "vectors.nav",
    rsRunner: "packages/contracts/headless/tests/domain_conformance.rs",
    rsEvidence: "fn nav_conformance",
  },
  pagination: {
    file: "domain.json",
    tsRunner: "packages/core/test/domain-conformance.test.ts",
    tsEvidence: "vectors.pagination",
    rsRunner: "packages/contracts/headless/tests/domain_conformance.rs",
    rsEvidence: "fn pagination_conformance",
  },
  tree: {
    file: "domain.json",
    tsRunner: "packages/core/test/domain-conformance.test.ts",
    tsEvidence: "vectors.tree",
    rsRunner: "packages/contracts/headless/tests/domain_conformance.rs",
    rsEvidence: "fn tree_conformance",
  },
  agentTranscript: {
    file: "agent-transcript.json",
    tsRunner: "packages/core/test/agent-transcript-conformance.test.ts",
    tsEvidence: "agent-transcript.json",
    rsRunner: "packages/contracts/headless/tests/agent_transcript_conformance.rs",
    rsEvidence: "agent-transcript.json",
  },
  agentPlan: {
    file: "agent-plan.json",
    tsRunner: "packages/core/test/agent-plan-conformance.test.ts",
    tsEvidence: "agent-plan.json",
    rsRunner: "packages/contracts/headless/tests/agent_plan_conformance.rs",
    rsEvidence: "agent-plan.json",
  },
  agentQuestion: {
    file: "agent-question.json",
    tsRunner: "packages/core/test/agent-question-conformance.test.ts",
    tsEvidence: "agent-question.json",
    rsRunner: "packages/contracts/headless/tests/agent_question_conformance.rs",
    rsEvidence: "agent-question.json",
  },
  agentSubagent: {
    file: "agent-subagent.json",
    tsRunner: "packages/core/test/agent-subagent-conformance.test.ts",
    tsEvidence: "agent-subagent.json",
    rsRunner: "packages/contracts/headless/tests/agent_subagent_conformance.rs",
    rsEvidence: "agent-subagent.json",
  },
};

/**
 * Correctly-different machine modules, exempt from the convention rule with a
 * reason. Key: `<runtime>:<module>`. The stateless trivial-case machines are
 * NOT listed here — the convention accommodates them, so they pass the check.
 */
const BASELINE: Record<string, string> = {
  "rs:text_input":
    "text editing model (caret/selection/keyboard-table contract), not a behavior machine; no Context/Event/Effect machine shape by design.",
  "popover":
    "pure-transition vectors reconciled into the mounted conformance corpus (g14.005): 22 Popover cases cover open/close/escape/outside/disabled claims across Svelte, React, and GPUI through the real machine; the machine stays, exercised by both web runtimes and the Rust mirror through the corpus.",
};

/** Canonical name for a module: strip separators, lowercase. */
function canonicalName(name: string): string {
  return name.replace(/[^a-z0-9]/gi, "").toLowerCase();
}

function tsMachineModules(): string[] {
  return readdirSync(tsSrcDir)
    .filter((name) => name.endsWith(".ts"))
    .map((name) => name.replace(/\.ts$/, ""))
    .filter((name) => name !== "index" && name !== "machine");
}

function rsMachineModules(): string[] {
  return readdirSync(rsSrcDir)
    .filter((name) => name.endsWith(".rs"))
    .map((name) => name.replace(/\.rs$/, ""))
    .filter((name) => name !== "lib");
}

function duplicatedMachines(): string[] {
  const ts = new Set(tsMachineModules().map(canonicalName));
  const rs = new Set(rsMachineModules().map(canonicalName));
  return [...ts].filter((name) => rs.has(name));
}

/** Does the module declare a transition? */
function declaresTransition(src: string): boolean {
  return /export function \w+Transition\s*\(/.test(src) || /pub fn \w+_transition\s*\(/.test(src);
}

interface ShapeFlags {
  context: boolean;
  event: boolean;
  effect: boolean;
  result: boolean;
  state: boolean;
  transitionResult: boolean;
}

function tsShape(src: string): ShapeFlags {
  return {
    context: /export interface \w+Context\b/.test(src),
    event: /export type \w+Event\b/.test(src),
    effect: /export type \w+Effect\b/.test(src),
    result: /export (?:type|interface) \w+Result\b/.test(src),
    state: /export type \w+State\b/.test(src),
    transitionResult: /import type \{[^}]*TransitionResult/.test(src),
  };
}

function rsShape(src: string): ShapeFlags {
  return {
    context: /pub struct \w+Context\b/.test(src),
    event: /pub enum \w+Event\b/.test(src),
    effect: /pub enum \w+Effect\b/.test(src),
    result: false,
    state: /pub enum \w+State\b/.test(src),
    transitionResult: false,
  };
}

interface Finding {
  module: string;
  detail: string;
}

function pinningFindings(): Finding[] {
  const findings: Finding[] = [];
  const tsSet = new Set(tsMachineModules().map(canonicalName));
  const rsSet = new Set(rsMachineModules().map(canonicalName));
  const duplicated = duplicatedMachines();
  const pinnedNames = new Set(Object.keys(PINNED).map(canonicalName));

  for (const name of duplicated) {
    if (!pinnedNames.has(name)) {
      findings.push({
        module: name,
        detail: "present in both runtimes but not in the PINNED registry (no shared vector)",
      });
    }
  }

  for (const [name, entry] of Object.entries(PINNED)) {
    const canonical = canonicalName(name);

    if (!tsSet.has(canonical) || !rsSet.has(canonical)) {
      findings.push({
        module: name,
        detail: `PINNED registry entry has no ${!tsSet.has(canonical) ? "TS" : "Rust"} module — delete the entry`,
      });
      continue;
    }

    const vectorPath = path.join(vectorsDir, entry.file);

    if (!existsSync(vectorPath)) {
      findings.push({ module: name, detail: `vector file missing: ${entry.file}` });
      continue;
    }

    const vectorText = readFileSync(vectorPath, "utf8");

    if (entry.file !== "domain.json" && entry.file !== "machines.json") {
      // Whole-file pin (agent vectors): the file exists and both runners
      // reference it — checked below.
    } else if (!vectorText.includes(`"${entry.file === "machines.json" ? name : name.toLowerCase()}"`)) {
      findings.push({ module: name, detail: `no vector key for ${name} in ${entry.file}` });
    }

    const tsRunnerPath = path.join(repoRoot, entry.tsRunner);

    if (!existsSync(tsRunnerPath)) {
      findings.push({ module: name, detail: `TS runner missing: ${entry.tsRunner}` });
    } else if (!readFileSync(tsRunnerPath, "utf8").includes(entry.tsEvidence)) {
      findings.push({
        module: name,
        detail: `TS runner ${entry.tsRunner} does not reference the vector (${entry.tsEvidence})`,
      });
    }

    const rsRunnerPath = path.join(repoRoot, entry.rsRunner);

    if (!existsSync(rsRunnerPath)) {
      findings.push({ module: name, detail: `Rust runner missing: ${entry.rsRunner}` });
    } else if (!readFileSync(rsRunnerPath, "utf8").includes(entry.rsEvidence)) {
      findings.push({
        module: name,
        detail: `Rust runner ${entry.rsRunner} does not reference the vector (${entry.rsEvidence})`,
      });
    }
  }

  return findings;
}

function shapeFindings(): Finding[] {
  const findings: Finding[] = [];

  for (const file of tsMachineModules()) {
    const src = readFileSync(path.join(tsSrcDir, `${file}.ts`), "utf8");

    if (!declaresTransition(src)) {
      continue;
    }

    const shape = tsShape(src);

    if (shape.state && !shape.transitionResult) {
      findings.push({
        module: `ts:${file}`,
        detail: "declares a State type but does not import TransitionResult (stateful machines must use the canonical Result)",
      });
    }

    if (!shape.context || !shape.event || !shape.effect || !shape.result) {
      findings.push({
        module: `ts:${file}`,
        detail: `transition without the machine shape: context=${shape.context} event=${shape.event} effect=${shape.effect} result=${shape.result}`,
      });
    }
  }

  for (const file of rsMachineModules()) {
    const src = readFileSync(path.join(rsSrcDir, `${file}.rs`), "utf8");

    if (!declaresTransition(src)) {
      continue;
    }

    const shape = rsShape(src);

    if (!shape.context || !shape.event || !shape.effect) {
      findings.push({
        module: `rs:${file}`,
        detail: `transition without the machine shape: context=${shape.context} event=${shape.event} effect=${shape.effect}`,
      });
    }
  }

  return findings;
}

export function machineShapeDrift(): { findings: Finding[]; baselined: Finding[] } {
  const findings = [...pinningFindings(), ...shapeFindings()];
  const baselined = findings.filter((finding) => finding.module in BASELINE);
  const fresh = findings.filter((finding) => !(finding.module in BASELINE));

  return { findings: fresh, baselined };
}

// Standalone gate: `bun packages/svelte/preview/scripts/machine-shape-drift.ts`
// (DRIFT_REPORT=1 lists the drift without exiting non-zero).
if (import.meta.main) {
  const { findings, baselined } = machineShapeDrift();

  console.log(
    `machine-shape-drift: checked ${Object.keys(PINNED).length} pinned machines, ` +
      `${tsMachineModules().length} TS modules, ${rsMachineModules().length} Rust modules\n`,
  );

  for (const b of baselined) {
    console.log(`baselined: [${b.module}] ${BASELINE[b.module]}`);
  }

  if (findings.length > 0) {
    console.log(`\nFAIL — ${findings.length} finding(s):`);
    for (const finding of findings) console.log(`  [${finding.module}] ${finding.detail}`);
  } else {
    console.log("OK — every duplicated machine is pinned by both harnesses; every transition follows the convention.");
  }

  if (findings.length > 0 && process.env.DRIFT_REPORT !== "1") {
    process.exit(1);
  }
}
