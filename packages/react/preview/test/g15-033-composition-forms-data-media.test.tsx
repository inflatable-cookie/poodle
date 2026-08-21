import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { FieldSetSpecimen } from "../src/gallery/specimens/FieldSetSpecimen";
import { ValidationSummarySpecimen } from "../src/gallery/specimens/ValidationSummarySpecimen";

describe("g15.033 composition, forms, data, and media specimens", () => {
  it("teaches FieldSet description at the group level", () => {
    const { container } = render(<FieldSetSpecimen />);

    expect(container.querySelector(".poodle-fieldset__description")?.textContent).toBe(
      "We use this to reach you about your account.",
    );
  });

  it("wires ValidationSummary links to visible invalid and pending controls", async () => {
    const { container } = render(<ValidationSummarySpecimen />);
    window.history.replaceState(null, "", "/#components/validation-summary");

    for (const link of container.querySelectorAll<HTMLAnchorElement>(".poodle-validation-summary a")) {
      const targetId = link.getAttribute("href")?.slice(1);
      expect(targetId).toBeTruthy();
      expect(container.querySelector(`#${targetId}`)).not.toBeNull();
    }
    expect(container.querySelector(".poodle-field[data-validation-state='invalid'] #project-name")).not.toBeNull();
    expect(container.querySelector(".poodle-field[data-validation-state='pending'] #repository")).not.toBeNull();

    await fireEvent.click(container.querySelector<HTMLAnchorElement>(".poodle-validation-summary a")!);
    expect(window.location.hash).toBe("#components/validation-summary");
    expect(document.activeElement?.id).toBe("project-name");
  });
});
