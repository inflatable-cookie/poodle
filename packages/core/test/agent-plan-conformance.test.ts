/**
 * Cross-runtime conformance: runs the shared vectors in
 * packages/contracts/headless/vectors/agent-plan.json against the TS
 * implementation. The Rust mirror runs the same vectors
 * (packages/contracts/headless/tests/agent_plan_conformance.rs).
 *
 * The decision lifecycle is what the host acts on, so a divergence would have
 * one target re-decide a plan another target considers settled.
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, test } from "bun:test";

import {
  canDecidePlan,
  decidePlan,
  planRecordSummary,
  planStatusLabel,
  type AgentPlanDecision,
  type AgentPlanSettledStatus,
  type AgentPlanStatus,
} from "../src/agent-plan.ts";

const vectors = JSON.parse(
  readFileSync(
    join(import.meta.dir, "..", "..", "contracts", "headless", "vectors", "agent-plan.json"),
    "utf8",
  ),
) as {
  decide: Array<{
    name: string;
    status: AgentPlanStatus;
    next: AgentPlanSettledStatus;
    decidedAt?: string;
    decision: AgentPlanDecision | null;
    canDecide: boolean;
  }>;
  labels: Array<{ status: AgentPlanStatus; label: string }>;
  summary: Array<{ name: string; plan: string; maxLength: number; summary: string }>;
};

describe("decision vectors", () => {
  for (const testCase of vectors.decide) {
    test(testCase.name, () => {
      expect(decidePlan(testCase.status, testCase.next, testCase.decidedAt)).toEqual(
        testCase.decision,
      );
      expect(canDecidePlan(testCase.status)).toBe(testCase.canDecide);
    });
  }
});

describe("status label vectors", () => {
  for (const testCase of vectors.labels) {
    test(testCase.status, () => {
      expect(planStatusLabel(testCase.status)).toBe(testCase.label);
    });
  }
});

describe("record summary vectors", () => {
  for (const testCase of vectors.summary) {
    test(testCase.name, () => {
      expect(planRecordSummary(testCase.plan, testCase.maxLength)).toBe(testCase.summary);
    });
  }
});
