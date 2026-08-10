/**
 * Cross-runtime conformance: runs the shared vectors in
 * packages/contracts/headless/vectors/agent-subagent.json against the TS
 * implementation. The Rust mirror runs the same vectors
 * (packages/contracts/headless/tests/agent_subagent_conformance.rs).
 *
 * The badge wording and the terminal mapping are what the transcript renders,
 * so a divergence would have one target show a child as finished while another
 * still shows it working.
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, test } from "bun:test";

import {
  isTerminalSubagentStatus,
  subagentStatusLabel,
  subagentStatusSpins,
  type AgentSubagentStatus,
} from "../src/agent-subagent.ts";

const vectors = JSON.parse(
  readFileSync(
    join(import.meta.dir, "..", "..", "contracts", "headless", "vectors", "agent-subagent.json"),
    "utf8",
  ),
) as {
  labels: Array<{ status: AgentSubagentStatus; label: string }>;
  terminal: Array<{ status: AgentSubagentStatus; terminal: boolean }>;
  spinning: Array<{ status: AgentSubagentStatus; spinning: boolean }>;
};

describe("status label vectors", () => {
  for (const testCase of vectors.labels) {
    test(`${testCase.status} → ${testCase.label}`, () => {
      expect(subagentStatusLabel(testCase.status)).toBe(testCase.label);
    });
  }
});

describe("terminal mapping vectors", () => {
  for (const testCase of vectors.terminal) {
    test(`${testCase.status} terminal=${testCase.terminal}`, () => {
      expect(isTerminalSubagentStatus(testCase.status)).toBe(testCase.terminal);
    });
  }
});

describe("spinner mapping vectors", () => {
  for (const testCase of vectors.spinning) {
    test(`${testCase.status} spinning=${testCase.spinning}`, () => {
      expect(subagentStatusSpins(testCase.status)).toBe(testCase.spinning);
    });
  }
});
