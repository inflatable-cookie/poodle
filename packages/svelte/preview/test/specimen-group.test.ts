import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import SpecimenGroup from "../src/components/SpecimenGroup.svelte";

describe("SpecimenGroup", () => {
  it("renders the caption label and optional description", () => {
    render(SpecimenGroup, {
      props: {
        label: "Inline markup",
        description: "Structure, not text: code spans survive the block model.",
      },
    });

    const group = document.querySelector(".poodle-specimen-group");
    expect(group).not.toBeNull();
    expect((group?.querySelector("[class*=eyebrow]")?.textContent ?? "").trim()).toBe(
      "Inline markup",
    );
    expect(document.querySelector(".poodle-text")?.textContent).toBe(
      "Structure, not text: code spans survive the block model.",
    );
  });

  it("omits description copy when none is supplied", () => {
    render(SpecimenGroup, { props: { label: "Quotes and rules" } });
    expect(document.querySelector(".poodle-text")).toBeNull();
  });
});
