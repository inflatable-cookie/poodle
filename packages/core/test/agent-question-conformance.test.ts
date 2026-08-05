/**
 * Cross-runtime conformance: runs the shared vectors in
 * packages/contracts/headless/vectors/agent-question.json against the TS
 * implementation. The Rust mirror runs the same vectors
 * (packages/contracts/headless/tests/agent_question_conformance.rs).
 *
 * Answer resolution is what the agent receives, so a divergence would have the
 * desktop build send a different answer from the web one for the same
 * interaction.
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, test } from "bun:test";

import {
  answeredQuestionSummary,
  canSubmitQuestion,
  declineQuestion,
  nextQuestionIndex,
  questionBatchComplete,
  questionProgress,
  resolveQuestionAnswer,
  showsQuestionProgress,
  submitsOnSelect,
  toggleQuestionSelection,
  type AgentQuestionAnswer,
  type AgentQuestionItem,
} from "../src/agent-question.ts";

const vectors = JSON.parse(
  readFileSync(
    join(import.meta.dir, "..", "..", "contracts", "headless", "vectors", "agent-question.json"),
    "utf8",
  ),
) as {
  resolve: Array<{
    name: string;
    q: AgentQuestionItem;
    sel: string[];
    text: string;
    answer: AgentQuestionAnswer | null;
    canSubmit: boolean;
    submitsOnSelect: boolean;
  }>;
  toggle: Array<{ name: string; q: AgentQuestionItem; sel: string[]; value: string; result: string[] }>;
  progress: Array<{
    name: string;
    questions: AgentQuestionItem[];
    activeIndex: number;
    progress: { states: string[]; current: number; total: number };
    shows: boolean;
    next: number;
    complete: boolean;
  }>;
  summary: Array<{ name: string; answer: AgentQuestionAnswer; summary: string }>;
};

describe("answer resolution vectors", () => {
  for (const testCase of vectors.resolve) {
    test(testCase.name, () => {
      expect(resolveQuestionAnswer(testCase.q, testCase.sel, testCase.text)).toEqual(testCase.answer);
      expect(canSubmitQuestion(testCase.q, testCase.sel, testCase.text)).toBe(testCase.canSubmit);
      expect(submitsOnSelect(testCase.q)).toBe(testCase.submitsOnSelect);
    });
  }
});

describe("selection toggling vectors", () => {
  for (const testCase of vectors.toggle) {
    test(testCase.name, () => {
      expect(toggleQuestionSelection(testCase.q, testCase.sel, testCase.value)).toEqual(testCase.result);
    });
  }
});

describe("batch progress vectors", () => {
  for (const testCase of vectors.progress) {
    test(testCase.name, () => {
      expect(questionProgress(testCase.questions, testCase.activeIndex)).toEqual(testCase.progress as any);
      expect(showsQuestionProgress(testCase.questions)).toBe(testCase.shows);
      expect(nextQuestionIndex(testCase.questions, testCase.activeIndex)).toBe(testCase.next);
      expect(questionBatchComplete(testCase.questions, testCase.activeIndex)).toBe(testCase.complete);
    });
  }
});

describe("answered summary vectors", () => {
  const options = [
    { value: "inline", label: "Inline in the transcript" },
    { value: "composer", label: "Anchored above the composer" },
    { value: "modal", label: "Modal dialog" },
  ];

  for (const testCase of vectors.summary) {
    test(testCase.name, () => {
      const question: AgentQuestionItem = {
        id: testCase.answer.questionId,
        prompt: "",
        options,
      };
      expect(answeredQuestionSummary({ question, answer: testCase.answer })).toBe(testCase.summary);
    });
  }
});

describe("declineQuestion", () => {
  test("carries neither values nor text", () => {
    // A turn cannot finish with an open question, so dismissal has to send
    // something — but it must not look like an answer.
    const question: AgentQuestionItem = { id: "q", prompt: "?", options: [] };
    expect(declineQuestion(question)).toEqual({
      questionId: "q",
      outcome: "declined",
      values: [],
      text: "",
    });
  });
});
