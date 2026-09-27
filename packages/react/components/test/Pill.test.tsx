import { fireEvent, render } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { describe, expect, it, vi } from "vitest";

import { Pill } from "../src/Pill";

const tokensCss = readFileSync(
  new URL("../../../core/src/tokens/generated/css/poodle-tokens.css", `file://${import.meta.dirname}/`),
  "utf8",
);
const iconCss = readFileSync(
  new URL("../../../core/src/styles/icon.css", `file://${import.meta.dirname}/`),
  "utf8",
);
const pillCss = readFileSync(
  new URL("../../../core/src/styles/pill.css", `file://${import.meta.dirname}/`),
  "utf8",
);

/** Injects the real cascade. icon.css comes after pill.css to mirror the bundle
 *  order that let `.poodle-icon[data-size]` win the 0,2,0 tie before the
 *  dismiss rule was raised to `svg.poodle-icon` (0,2,1). */
function injectPillStyles(): void {
  const style = document.createElement("style");
  style.textContent = `${tokensCss}\n${pillCss}\n${iconCss}`;
  document.head.appendChild(style);
}

describe("Pill (react)", () => {
  it("projects tone, appearance, size, and density data attributes", () => {
    const { container } = render(<Pill tone="info" appearance="subtle" size="lg">Beta</Pill>);
    const root = container.querySelector(".poodle-pill") as HTMLElement;
    expect(root.dataset.tone).toBe("info");
    expect(root.dataset.appearance).toBe("subtle");
    expect(root.dataset.size).toBe("lg");
  });

  it("defaults to the tint appearance and emits no fill axis", () => {
    const { container } = render(<Pill>Neutral</Pill>);
    const root = container.querySelector(".poodle-pill") as HTMLElement;
    expect(root.dataset.appearance).toBe("tint");
    expect(root.hasAttribute("data-fill")).toBe(false);

    const explicitTint = render(<Pill appearance="tint">Neutral</Pill>).container.querySelector(
      ".poodle-pill",
    ) as HTMLElement;
    expect(explicitTint.dataset.appearance).toBe(root.dataset.appearance);
    expect(explicitTint.outerHTML).toBe(root.outerHTML);
  });

  it("projects all four appearances as one mutually exclusive axis", () => {
    for (const appearance of ["tint", "solid", "subtle", "badge"] as const) {
      const { container } = render(
        <Pill tone="warning" appearance={appearance} dot title="Warning">
          Warning
        </Pill>,
      );
      const root = container.querySelector(".poodle-pill") as HTMLElement;
      expect(root.dataset.appearance).toBe(appearance);
      expect(root.hasAttribute("data-fill")).toBe(false);
      expect(root.title).toBe("Warning");
      expect(container.querySelector(".poodle-pill__dot")).not.toBeNull();
    }
  });

  it("carries the accent token and marks it custom", () => {
    const { container } = render(<Pill accent="#ff9900">Beta</Pill>);
    const root = container.querySelector(".poodle-pill") as HTMLElement;
    expect(root.dataset.accent).toBe("custom");
    expect(root.style.getPropertyValue("--poodle-pill-accent")).toBe("#ff9900");
  });

  it("projects muted, adaptive-width, and dot anatomy", () => {
    const { container } = render(
      <Pill muted adaptiveWidth dot>
        Beta
      </Pill>,
    );
    const root = container.querySelector(".poodle-pill") as HTMLElement;
    expect(root.dataset.muted).toBe("true");
    expect(root.dataset.adaptiveWidth).toBe("true");
    expect(container.querySelector(".poodle-pill__dot")?.getAttribute("aria-hidden")).toBe(
      "true",
    );
  });

  it("renders no dismiss control unless dismissible", () => {
    const { container } = render(<Pill>Videos</Pill>);
    expect(container.querySelector(".poodle-pill__dismiss")).toBeNull();
  });

  it("renders a real dismiss button with a default accessible name and fires onDismiss", () => {
    const onDismiss = vi.fn();
    const { container } = render(
      <Pill dismissible onDismiss={onDismiss}>
        Audio
      </Pill>,
    );
    const dismiss = container.querySelector(".poodle-pill__dismiss") as HTMLButtonElement;
    expect(dismiss.tagName).toBe("BUTTON");
    expect(dismiss.getAttribute("type")).toBe("button");
    expect(dismiss.disabled).toBe(false);
    expect(dismiss.tabIndex).toBeGreaterThanOrEqual(0);
    expect(dismiss.getAttribute("aria-label")).toBe("Dismiss");

    fireEvent.click(dismiss);
    expect(onDismiss).toHaveBeenCalledTimes(1);
  });

  it("takes the dismiss accessible name from dismissLabel", () => {
    const onDismiss = vi.fn();
    const { container } = render(
      <Pill dismissible dismissLabel="Remove filter: Videos" onDismiss={onDismiss}>
        Videos
      </Pill>,
    );
    const dismiss = container.querySelector(".poodle-pill__dismiss") as HTMLButtonElement;
    expect(dismiss.getAttribute("aria-label")).toBe("Remove filter: Videos");

    fireEvent.click(dismiss);
    expect(onDismiss).toHaveBeenCalledTimes(1);
  });

  it("sizes the dismiss icon in em, beating the icon data-size cascade", () => {
    injectPillStyles();
    const { container } = render(<Pill dismissible>Videos</Pill>);
    const dismiss = container.querySelector(".poodle-pill__dismiss") as HTMLElement;
    const icon = container.querySelector(".poodle-pill__dismiss .poodle-icon") as HTMLElement;
    expect(icon.getAttribute("data-size")).toBe("sm");

    // `font: inherit` is not resolved by happy-dom, so drive the em base
    // explicitly: the contracted 0.75em must track the dismiss font size, not
    // the icon's data-size rem value (0.75rem).
    dismiss.style.fontSize = "20px";
    expect(getComputedStyle(icon).width).toBe("15px");
    expect(getComputedStyle(icon).height).toBe("15px");

    dismiss.style.fontSize = "8px";
    expect(getComputedStyle(icon).width).toBe("6px");
    expect(getComputedStyle(icon).height).toBe("6px");
  });
});
