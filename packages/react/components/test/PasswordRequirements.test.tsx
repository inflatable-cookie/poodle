import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { PasswordRequirements } from "../src/PasswordRequirements";
import type { PasswordRequirementsPolicy } from "../src/types";

const fullPolicy: PasswordRequirementsPolicy = {
  minLength: 8,
  requireMixedCase: true,
  requireDigit: true,
  requireSpecial: true,
};

describe("PasswordRequirements (react)", () => {
  it("shows the loading label instead of the checklist while loading", () => {
    const { container } = render(<PasswordRequirements loading requirements={fullPolicy} />);
    expect(container.querySelector(".poodle-password-requirements__loading")?.textContent).toBe(
      "Loading requirements...",
    );
    expect(container.querySelector(".poodle-password-requirements__list")).toBeNull();
  });

  it("renders checklist items according to the policy config", () => {
    const { container } = render(
      <PasswordRequirements
        requirements={{ minLength: 8, requireMixedCase: false, requireDigit: true, requireSpecial: false }}
      />,
    );
    const items = [...container.querySelectorAll<HTMLElement>(".poodle-password-requirements__list li")].map(
      (el) => el.textContent,
    );
    expect(items).toEqual(["At least 8 characters", "At least one number"]);
  });

  it("marks each item met only when the password satisfies it", () => {
    const satisfying = render(<PasswordRequirements password="Abcdef12!" requirements={fullPolicy} />);
    expect(
      satisfying.container.querySelectorAll(".poodle-password-requirements__item--met").length,
    ).toBe(4);

    const weak = render(<PasswordRequirements password="abc" requirements={fullPolicy} />);
    expect(weak.container.querySelectorAll(".poodle-password-requirements__item--met").length).toBe(0);
  });

  it("names each item met state for assistive technology", () => {
    const { container } = render(<PasswordRequirements password="abc" requirements={fullPolicy} />);
    const names = [...container.querySelectorAll<HTMLElement>(".poodle-password-requirements__list li")].map(
      (el) => el.getAttribute("aria-label"),
    );
    expect(names).toEqual([
      "At least 8 characters — not met",
      "Mix of uppercase and lowercase letters — not met",
      "At least one number — not met",
      "At least one special character — not met",
    ]);

    const satisfying = render(<PasswordRequirements password="Abcdef12!" requirements={fullPolicy} />);
    const metNames = [...satisfying.container.querySelectorAll<HTMLElement>(".poodle-password-requirements__list li")].map(
      (el) => el.getAttribute("aria-label"),
    );
    expect(metNames).toEqual([
      "At least 8 characters — met",
      "Mix of uppercase and lowercase letters — met",
      "At least one number — met",
      "At least one special character — met",
    ]);
  });

  it("shows a check or cross glyph per rule, not color alone", () => {
    const { container } = render(<PasswordRequirements password="abc" requirements={fullPolicy} />);
    const weakGlyphs = [...container.querySelectorAll<HTMLElement>(".poodle-password-requirements__list li")].map(
      (el) =>
        el.querySelector(".poodle-password-requirements__item-icon svg path")?.getAttribute("d"),
    );
    expect(weakGlyphs).toEqual([
      "M6 6l12 12M18 6L6 18",
      "M6 6l12 12M18 6L6 18",
      "M6 6l12 12M18 6L6 18",
      "M6 6l12 12M18 6L6 18",
    ]);

    const satisfying = render(<PasswordRequirements password="Abcdef12!" requirements={fullPolicy} />);
    const metGlyphs = [...satisfying.container.querySelectorAll<HTMLElement>(".poodle-password-requirements__list li")].map(
      (el) =>
        el.querySelector(".poodle-password-requirements__item-icon svg path")?.getAttribute("d"),
    );
    expect(metGlyphs).toEqual([
      "M4.5 12.5l5 5L19.5 7",
      "M4.5 12.5l5 5L19.5 7",
      "M4.5 12.5l5 5L19.5 7",
      "M4.5 12.5l5 5L19.5 7",
    ]);
  });

  it("renders the error only when requirements are absent", () => {
    const withError = render(<PasswordRequirements error="Failed to load policy." />);
    expect(withError.container.querySelector(".poodle-password-requirements__error")?.textContent).toBe(
      "Failed to load policy.",
    );
    expect(
      withError.container.querySelector(".poodle-password-requirements__error")?.getAttribute("role"),
    ).toBe("alert");
    expect(withError.container.querySelector(".poodle-password-requirements__list")).toBeNull();

    const withPolicy = render(<PasswordRequirements error="Failed to load policy." requirements={fullPolicy} />);
    expect(withPolicy.container.querySelector(".poodle-password-requirements__error")).toBeNull();
  });

  it("renders the description and hint below the checklist", () => {
    const { container } = render(
      <PasswordRequirements requirements={{ ...fullPolicy, description: "8+ characters." }} hint="Avoid common words." />,
    );
    expect(container.querySelector(".poodle-password-requirements__description")?.textContent).toBe(
      "8+ characters.",
    );
    expect(container.querySelector(".poodle-password-requirements__hint")?.textContent).toBe(
      "Avoid common words.",
    );

    const noHint = render(<PasswordRequirements requirements={fullPolicy} hint={null} />);
    expect(noHint.container.querySelector(".poodle-password-requirements__hint")).toBeNull();
  });

  it("announces updates through a polite live region and resolves the size", () => {
    const { container } = render(<PasswordRequirements requirements={fullPolicy} size="lg" />);
    const root = container.querySelector<HTMLElement>(".poodle-password-requirements");
    expect(root?.getAttribute("aria-live")).toBe("polite");
    expect(root?.getAttribute("data-size")).toBe("lg");
  });
});
