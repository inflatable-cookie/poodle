import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import PasswordRequirements from "../src/PasswordRequirements.svelte";
import type { PasswordRequirementsPolicy } from "../src/types";

const fullPolicy: PasswordRequirementsPolicy = {
  minLength: 8,
  requireMixedCase: true,
  requireDigit: true,
  requireSpecial: true,
};

describe("PasswordRequirements (svelte)", () => {
  it("shows the loading label instead of the checklist while loading", () => {
    const { container } = render(PasswordRequirements, {
      props: { loading: true, requirements: fullPolicy },
    });
    expect(container.querySelector(".poodle-password-requirements__loading")?.textContent).toBe(
      "Loading requirements...",
    );
    expect(container.querySelector(".poodle-password-requirements__list")).toBeNull();
  });

  it("renders checklist items according to the policy config", () => {
    const { container } = render(PasswordRequirements, {
      props: {
        requirements: { minLength: 8, requireMixedCase: false, requireDigit: true, requireSpecial: false },
      },
    });
    const items = [...container.querySelectorAll<HTMLElement>(".poodle-password-requirements__list li")].map(
      (el) => el.textContent,
    );
    expect(items).toEqual(["At least 8 characters", "At least one number"]);
  });

  it("marks each item met only when the password satisfies it", () => {
    const satisfying = render(PasswordRequirements, {
      props: { password: "Abcdef12!", requirements: fullPolicy },
    });
    expect(
      satisfying.container.querySelectorAll(".poodle-password-requirements__item--met").length,
    ).toBe(4);

    const weak = render(PasswordRequirements, {
      props: { password: "abc", requirements: fullPolicy },
    });
    expect(weak.container.querySelectorAll(".poodle-password-requirements__item--met").length).toBe(0);
  });

  it("renders the error only when requirements are absent", () => {
    const withError = render(PasswordRequirements, { props: { error: "Failed to load policy." } });
    expect(withError.container.querySelector(".poodle-password-requirements__error")?.textContent).toBe(
      "Failed to load policy.",
    );
    expect(withError.container.querySelector(".poodle-password-requirements__list")).toBeNull();

    const withPolicy = render(PasswordRequirements, {
      props: { error: "Failed to load policy.", requirements: fullPolicy },
    });
    expect(withPolicy.container.querySelector(".poodle-password-requirements__error")).toBeNull();
  });

  it("renders the description and hint below the checklist", () => {
    const { container } = render(PasswordRequirements, {
      props: {
        requirements: { ...fullPolicy, description: "8+ characters." },
        hint: "Avoid common words.",
      },
    });
    expect(container.querySelector(".poodle-password-requirements__description")?.textContent).toBe(
      "8+ characters.",
    );
    expect(container.querySelector(".poodle-password-requirements__hint")?.textContent).toBe(
      "Avoid common words.",
    );

    const noHint = render(PasswordRequirements, { props: { requirements: fullPolicy, hint: null } });
    expect(noHint.container.querySelector(".poodle-password-requirements__hint")).toBeNull();
  });

  it("announces updates through a polite live region and resolves the size", () => {
    const { container } = render(PasswordRequirements, {
      props: { requirements: fullPolicy, size: "lg" },
    });
    const root = container.querySelector<HTMLElement>(".poodle-password-requirements");
    expect(root?.getAttribute("aria-live")).toBe("polite");
    expect(root?.getAttribute("data-size")).toBe("lg");
  });
});
