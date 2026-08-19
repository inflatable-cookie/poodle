import { readFileSync } from "node:fs";
import { join } from "node:path";
import {
  cleanup as cleanupReact,
  fireEvent as fireEventReact,
  render as renderReact,
} from "@testing-library/react";
import {
  cleanup as cleanupSvelte,
  fireEvent as fireEventSvelte,
  render as renderSvelte,
} from "@testing-library/svelte";
import iconNodes from "lucide-static/icon-nodes.json";
import { createElement, type ComponentType } from "react";
import { describe, expect, it } from "vitest";

import { IconProvider, type IconSet } from "../../packages/react/components/src";
import { specimenMap as reactMap } from "../../packages/react/preview/src/gallery/specimen-map";
import { specimenMap as svelteMap } from "../../packages/svelte/preview/src/specimens/registry";
import PilotSpecimenHarness from "../../packages/svelte/preview/test/PilotSpecimenHarness.svelte";

/**
 * `g15.024` curates the agent and tools family back to the outline's section
 * budget. Caption order and Svelte/React equality are pinned exactly. GPUI
 * teaches the same ordered intent from deterministic source evidence until
 * `g15.026` owns mounted native page probing.
 */

function captions(): string[] {
  return [...document.querySelectorAll(".poodle-specimen-group")].map((group) =>
    (group.querySelector("[class*=eyebrow]")?.textContent ?? "").trim(),
  );
}

interface Page {
  slug: string;
  gpui: string;
  expected: string[];
}

const PAGES: Page[] = [
  {
    slug: "agent-chat-input",
    gpui: "agent_chat_input_specimen",
    expected: [
      "Default composer",
      "Questions and plans",
      "Busy and unavailable",
      "Attachments and footer",
      "Submission rules",
      "Editor growth",
    ],
  },
  {
    slug: "agent-message",
    gpui: "agent_message",
    expected: [
      "Assistant and user messages",
      "Inline formatting and headings",
      "Code blocks",
      "List structures",
      "Quotes, rules and fallback",
      "Streaming",
    ],
  },
  {
    slug: "agent-question",
    gpui: "agent_question",
    expected: [
      "Hosted by the composer",
      "Choice modes",
      "Batch progress",
      "Dismissal",
      "Shortcut limits",
    ],
  },
  {
    slug: "agent-question-record",
    gpui: "agent_question_record",
    expected: ["Selected answers", "Free-text override", "Declined", "Presentation options"],
  },
  {
    slug: "agent-transcript",
    gpui: "agent_transcript",
    expected: [
      "A worked turn",
      "Tool run states",
      "Streaming and detached scroll",
      "Long transcript rendering",
      "Empty",
    ],
  },
  {
    slug: "changed-files",
    gpui: "changed_files",
    expected: ["Worked change set", "Paths and scopes", "Count variations", "Overflow and actions"],
  },
];

const GPUI_SPECIMENS = join(import.meta.dirname, "../../packages/gpui/preview/src/specimens");

function collect(source: string, pattern: RegExp): string[] {
  return [...source.matchAll(pattern)].map((match) => match[1]!);
}

function gpuiCaptions(page: Page): string[] {
  const source = readFileSync(join(GPUI_SPECIMENS, `${page.gpui}.rs`), "utf8");
  const examplesMatch = source.match(/\n\s*specimen_layout\(/);
  const examples = examplesMatch ? source.slice(0, examplesMatch.index) : source;

  const grouped = collect(
    examples,
    /\bgroup(?:_block)?\(\s*(?:(?:&?theme|state)[^,]*,\s*)?"([^"]+)"/g,
  );
  if (grouped.length > 0) return grouped;

  const sections = collect(examples, /\bsection\(\s*"([^"]+)"/g);
  if (sections.length > 0) return sections;

  return collect(examples, /EyebrowSpec::new\(\)\s*\.with_content\("([^"]+)"\)/g);
}

function svelteCaptions(page: Page): string[] {
  const Specimen = svelteMap[page.slug];
  expect(Specimen, `${page.slug} missing from the Svelte registry`).toBeTruthy();
  renderSvelte(PilotSpecimenHarness, { props: { specimen: Specimen as never } });
  const rendered = captions();
  cleanupSvelte();
  return rendered;
}

function renderReactSpecimen(Specimen: ComponentType) {
  return renderReact(
    createElement(
      IconProvider,
      { icons: iconNodes as unknown as IconSet },
      createElement(Specimen),
    ),
  );
}

function reactCaptions(page: Page): string[] {
  const Specimen = reactMap[page.slug] as ComponentType | undefined;
  expect(Specimen, `${page.slug} missing from the React registry`).toBeTruthy();
  renderReactSpecimen(Specimen!);
  const rendered = captions();
  cleanupReact();
  return rendered;
}

describe("g15.024 agent and tools specimens", () => {
  it("owns exactly the six pages the card partitions", () => {
    expect(PAGES.map((page) => page.slug)).toEqual([
      "agent-chat-input",
      "agent-message",
      "agent-question",
      "agent-question-record",
      "agent-transcript",
      "changed-files",
    ]);
  });

  for (const page of PAGES) {
    describe(page.slug, () => {
      it("renders its curated captions in order", () => {
        expect(svelteCaptions(page)).toEqual(page.expected);
      });

      it("keeps Svelte and React captions identical", () => {
        expect(reactCaptions(page)).toEqual(svelteCaptions(page));
      });

      it("stays inside the outline's section budget", () => {
        const count = page.expected.length;
        expect(count).toBeGreaterThanOrEqual(3);
        expect(count).toBeLessThanOrEqual(6);
      });

      it("captions every example group", () => {
        expect(svelteCaptions(page).filter((caption) => caption === "")).toEqual([]);
      });

      it("teaches the same ordered sections in GPUI", () => {
        expect(gpuiCaptions(page)).toEqual(page.expected);
      });

      it("keeps the axis matrices out of Examples", () => {
        for (const caption of page.expected) {
          expect(caption).not.toMatch(/\bsizes?\b|\bdensit/i);
        }
      });
    });
  }

  function groupByCaption(caption: string): HTMLElement {
    const group = [...document.querySelectorAll(".poodle-specimen-group")].find(
      (candidate) =>
        (candidate.querySelector("[class*=eyebrow]")?.textContent ?? "").trim() === caption,
    );
    expect(group, `no specimen group captioned "${caption}"`).toBeTruthy();
    return group as HTMLElement;
  }

  it("teaches AgentMessage roles, heading levels, lists, fallback, and the streaming caret", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["agent-message"] as never },
    });

    const roles = groupByCaption("Assistant and user messages");
    expect(roles.querySelector('[data-role="user"]')).toBeTruthy();
    expect(roles.querySelectorAll(".poodle-agent-message").length).toBe(3);

    const formatting = groupByCaption("Inline formatting and headings");
    expect(formatting.querySelector("h1")).toBeTruthy();
    expect(formatting.querySelector("h2")).toBeTruthy();
    expect(formatting.querySelector("h3")).toBeTruthy();
    expect(formatting.querySelector("h4")).toBeTruthy();
    expect(formatting.querySelector("h5")).toBeTruthy();
    expect(formatting.querySelector("h6")).toBeTruthy();
    expect(formatting.querySelector("code, a, em, strong, del, s")).toBeTruthy();

    expect(groupByCaption("Code blocks").querySelectorAll("pre").length).toBeGreaterThanOrEqual(2);
    expect(groupByCaption("List structures").querySelectorAll("ul, ol").length).toBeGreaterThanOrEqual(4);

    const fallback = groupByCaption("Quotes, rules and fallback");
    expect(fallback.querySelector("blockquote")).toBeTruthy();
    expect(fallback.querySelector(".poodle-separator")).toBeTruthy();
    expect(fallback.textContent).toMatch(/<div>/);

    const caret = groupByCaption("Streaming").querySelector(".poodle-agent-message__caret");
    expect(caret, "streaming caret").toBeTruthy();
    expect(caret!.getAttribute("aria-hidden")).toBe("true");
    cleanupSvelte();
  });

  it("keeps AgentQuestion hosted override, choice modes, batch, dismissal, and shortcut limits live", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["agent-question"] as never },
    });

    const hosted = groupByCaption("Hosted by the composer");
    expect(hosted.querySelector(".poodle-agent-chat-input")).toBeTruthy();
    const hostedOption = hosted.querySelector(
      ".poodle-agent-question__option",
    ) as HTMLButtonElement;
    expect(hostedOption, "hosted option").toBeTruthy();
    fireEventSvelte.click(hostedOption);
    const hostedEditor = hosted.querySelector("textarea, input") as HTMLTextAreaElement;
    expect(hostedEditor, "hosted composer editor").toBeTruthy();
    fireEventSvelte.input(hostedEditor, { target: { value: "Neither — put it in the sidebar." } });
    expect(hosted.querySelector('.poodle-agent-question__option[data-selected="true"]')).toBeNull();

    const choices = groupByCaption("Choice modes");
    expect(choices.querySelectorAll(".poodle-agent-question").length).toBe(3);
    expect(choices.querySelectorAll(".poodle-checkbox").length).toBeGreaterThan(0);
    expect(choices.querySelector('.poodle-agent-question__option[data-selected="true"]')).toBeTruthy();
    expect(choices.textContent).toMatch(/A block in the conversation/);

    const batch = groupByCaption("Batch progress");
    const batchQuestions = batch.querySelectorAll(".poodle-agent-question");
    expect(batchQuestions.length).toBe(2);
    expect(batch.querySelectorAll(".poodle-agent-question__progress-dot").length).toBe(8);

    const dismissal = groupByCaption("Dismissal");
    expect(dismissal.querySelectorAll(".poodle-agent-question").length).toBe(2);
    expect(dismissal.querySelectorAll(".poodle-agent-question__dismiss").length).toBe(1);

    const shortcuts = groupByCaption("Shortcut limits");
    const questions = [...shortcuts.querySelectorAll(".poodle-agent-question")];
    expect(questions.length).toBe(2);
    expect(questions[0]!.querySelectorAll(".poodle-agent-question__option-shortcut").length).toBe(9);
    expect(questions[0]!.querySelectorAll(".poodle-agent-question__option").length).toBe(12);
    expect(questions[1]!.querySelectorAll(".poodle-agent-question__option-shortcut").length).toBe(0);
    cleanupSvelte();
  });

  it("keeps AgentQuestionRecord selected, override, declined, and presentation variants", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["agent-question-record"] as never },
    });
    const selected = groupByCaption("Selected answers");
    expect(selected.querySelectorAll(".poodle-agent-question-record").length).toBe(2);
    expect(selected.textContent).toMatch(/Anchored above the composer/);
    expect(selected.textContent).toMatch(/Svelte/);

    const override = groupByCaption("Free-text override");
    expect(override.textContent).toMatch(/Neither — put it in the sidebar/);
    expect(override.querySelectorAll(".poodle-agent-question-record__option").length).toBe(0);

    expect(groupByCaption("Declined").textContent).toMatch(/Declined/);

    const presentation = groupByCaption("Presentation options");
    expect(presentation.querySelectorAll(".poodle-agent-question-record").length).toBe(3);
    expect(presentation.textContent).toMatch(/Placement/);
    cleanupSvelte();
  });

  it("updates AgentChatInput submit, stop, and attachment removal in both web runtimes", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["agent-chat-input"] as never },
    });
    const composer = groupByCaption("Default composer");
    const send = [...composer.querySelectorAll("button")].find(
      (button) => button.getAttribute("aria-label") === "Send",
    ) as HTMLButtonElement;
    expect(send, "default send").toBeTruthy();
    expect(send.disabled).toBe(false);
    fireEventSvelte.click(send);
    expect(composer.textContent).toContain("Last submitted: Summarise the release notes and open a PR");

    const busy = groupByCaption("Busy and unavailable");
    const stop = [...busy.querySelectorAll("button")].find(
      (button) => /stop/i.test(button.getAttribute("aria-label") ?? ""),
    ) as HTMLButtonElement;
    expect(stop, "busy stop").toBeTruthy();
    fireEventSvelte.click(stop);
    expect(busy.textContent).toMatch(/Stop pressed 1 time/);

    const attachments = groupByCaption("Attachments and footer");
    expect(attachments.querySelectorAll(".poodle-agent-chat-input__attachment").length).toBe(2);
    const remove = [...attachments.querySelectorAll("button")].find((button) =>
      /Remove architecture/.test(button.getAttribute("aria-label") ?? ""),
    ) as HTMLButtonElement;
    expect(remove, "remove architecture.png").toBeTruthy();
    fireEventSvelte.click(remove);
    expect(attachments.querySelectorAll(".poodle-agent-chat-input__attachment").length).toBe(1);

    const rules = groupByCaption("Submission rules");
    const sends = [...rules.querySelectorAll("button")].filter(
      (button) => button.getAttribute("aria-label") === "Send",
    ) as HTMLButtonElement[];
    expect(sends.length).toBeGreaterThanOrEqual(2);
    expect(sends[0]!.disabled).toBe(true);
    expect(sends[1]!.disabled).toBe(false);
    cleanupSvelte();

    renderReactSpecimen(reactMap["agent-chat-input"] as ComponentType);
    const reactComposer = groupByCaption("Default composer");
    const reactSend = [...reactComposer.querySelectorAll("button")].find(
      (button) => button.getAttribute("aria-label") === "Send",
    ) as HTMLButtonElement;
    fireEventReact.click(reactSend);
    expect(reactComposer.textContent).toContain(
      "Last submitted: Summarise the release notes and open a PR",
    );
    cleanupReact();
  });

  it("keeps AgentTranscript worked-turn, thirty-call, streaming, windowed, and empty evidence", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["agent-transcript"] as never },
    });
    const turn = groupByCaption("A worked turn");
    expect(turn.querySelector(".poodle-changed-files")).toBeTruthy();
    expect(turn.textContent).toMatch(/Working for 1h 1m/);

    const runs = groupByCaption("Tool run states");
    expect(runs.textContent).toMatch(/\+29 previous tool calls/);
    expect(runs.querySelectorAll(".poodle-tool-call-group").length).toBeGreaterThanOrEqual(3);

    const streaming = groupByCaption("Streaming and detached scroll");
    const caret = streaming.querySelector(".poodle-agent-message__caret");
    expect(caret, "transcript streaming caret").toBeTruthy();
    expect(caret!.getAttribute("aria-hidden")).toBe("true");

    const long = groupByCaption("Long transcript rendering");
    const transcripts = long.querySelectorAll(".poodle-agent-transcript");
    expect(transcripts.length).toBe(2);
    expect(transcripts[0]!.getAttribute("data-virtualized")).toBe("true");
    expect(transcripts[1]!.getAttribute("data-virtualized")).toBe("false");

    const empty = groupByCaption("Empty");
    expect(empty.querySelector('[data-empty="true"]')).toBeTruthy();
    cleanupSvelte();
  });

  it("keeps ChangedFiles disclosure, scopes, overflow, and withheld diff action live", () => {
    renderSvelte(PilotSpecimenHarness, {
      props: { specimen: svelteMap["changed-files"] as never },
    });
    const worked = groupByCaption("Worked change set");
    expect(worked.querySelectorAll(".poodle-changed-files").length).toBe(2);
    const toggle = worked.querySelector(".poodle-changed-files__toggle") as HTMLButtonElement;
    expect(toggle, "worked toggle").toBeTruthy();
    expect(worked.querySelector(".poodle-changed-files__tree")).toBeTruthy();
    fireEventSvelte.click(toggle);
    expect(worked.querySelectorAll(".poodle-changed-files__tree").length).toBe(2);

    const paths = groupByCaption("Paths and scopes");
    expect(paths.textContent).toMatch(/cp-api/);
    expect(paths.textContent).toMatch(/packages/);

    const overflow = groupByCaption("Overflow and actions");
    const overflowing = overflow.querySelectorAll(".poodle-changed-files")[0] as HTMLElement;
    expect(overflowing.querySelectorAll(".poodle-changed-files__chip").length).toBe(2);
    expect(overflow.textContent).toMatch(/Show all/);
    const noDiff = overflow.querySelectorAll(".poodle-changed-files")[1] as HTMLElement;
    expect(noDiff.textContent).not.toMatch(/Open diff/);
    expect(captions()).not.toContain("Empty");
    cleanupSvelte();
  });
});
