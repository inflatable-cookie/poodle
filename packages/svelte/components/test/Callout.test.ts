import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Callout from "../src/Callout.svelte";

describe("Callout (svelte)", () => {
  it("renders title and message with the tone data attribute", () => {
    const { container } = render(Callout, {
      props: { tone: "danger", title: "Failed", message: "Try again" },
    });
    const root = container.querySelector(".poodle-callout") as HTMLElement;
    expect(root.dataset.tone).toBe("danger");
    expect(container.querySelector(".poodle-callout__content strong")?.textContent).toBe("Failed");
    expect(container.querySelector(".poodle-callout__content p")?.textContent).toBe("Try again");
  });

  it("defaults to tint and projects every tone/fill combination", () => {
    const tones = ["neutral", "info", "success", "warning", "danger", "pending"] as const;
    for (const tone of tones) {
      for (const fill of ["tint", "solid"] as const) {
        const { container } = render(Callout, { props: { tone, fill, message: "m" } });
        const root = container.querySelector(".poodle-callout") as HTMLElement;
        expect(root.dataset.tone).toBe(tone);
        expect(root.dataset.fill).toBe(fill);
        if (tone === "pending") {
          expect(container.querySelector(".poodle-spinner")?.getAttribute("data-tone")).toBe(
            fill === "solid" ? "current" : "accent",
          );
        }
      }
    }

    const defaultRoot = render(Callout, { props: { message: "m" } }).container.querySelector(
      ".poodle-callout",
    ) as HTMLElement;
    expect(defaultRoot.dataset.fill).toBe("tint");
  });

  it("projects an alert or status live region from announceMode", () => {
    const assertive = render(Callout, { props: { announceMode: "assertive", message: "m" } });
    const assertiveRoot = assertive.container.querySelector(".poodle-callout") as HTMLElement;
    expect(assertiveRoot.getAttribute("role")).toBe("alert");
    expect(assertiveRoot.getAttribute("aria-live")).toBe("assertive");

    const polite = render(Callout, { props: { announceMode: "polite", message: "m" } });
    const politeRoot = polite.container.querySelector(".poodle-callout") as HTMLElement;
    expect(politeRoot.getAttribute("role")).toBe("status");
    expect(politeRoot.getAttribute("aria-live")).toBe("polite");

    const silent = render(Callout, { props: { message: "m" } });
    expect(silent.container.querySelector(".poodle-callout")?.getAttribute("role")).toBeNull();
  });

  it("renders the dismiss button only when dismissible and emits onDismiss", async () => {
    const onDismiss = vi.fn();
    const plain = render(Callout, { props: { message: "m" } });
    expect(plain.container.querySelector(".poodle-callout__dismiss")).toBeNull();

    const { container } = render(Callout, {
      props: { message: "m", dismissible: true, onDismiss },
    });
    const dismiss = container.querySelector(".poodle-callout__dismiss") as HTMLButtonElement;
    expect(dismiss.getAttribute("aria-label")).toBe("Dismiss message");

    await fireEvent.click(dismiss);
    expect(onDismiss).toHaveBeenCalledTimes(1);
  });
});
