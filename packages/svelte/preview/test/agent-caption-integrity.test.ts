import { render, screen } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import PilotSpecimenHarness from "./PilotSpecimenHarness.svelte";
import AgentMessageSpecimen from "../src/specimens/AgentMessageSpecimen.svelte";

/** Captions a reader can actually see, in order. */
function captions(): string[] {
  return [...document.querySelectorAll(".poodle-specimen-group")].map((group) =>
    (group.querySelector("[class*=eyebrow]")?.textContent ?? "").trim(),
  );
}

describe("g15.015 agent-surface captions", () => {
  it("renders every AgentMessage example caption", () => {
    render(PilotSpecimenHarness, { props: { specimen: AgentMessageSpecimen } });
    const rendered = captions();
    expect(rendered.length).toBe(6);
    expect(rendered.filter((caption) => caption === "")).toEqual([]);
    expect(rendered[0]).toBe("Assistant and user messages");
  });

  it("renders authored descriptions for captioned groups", () => {
    render(PilotSpecimenHarness, { props: { specimen: AgentMessageSpecimen } });
    expect(
      screen.getByText(
        "Structure, not text: code spans, emphasis, links and strikethrough all survive the block model. Headings are real heading elements, so the message is navigable by heading.",
      ),
    ).not.toBeNull();
  });
});
