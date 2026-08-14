import { render, screen } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import LicenceStatus from "../src/LicenceStatus.svelte";
import { formatDisplayTimeDate, type LicenceUsability } from "@inflatable-cookie/poodle-core";

const USE_UNTIL = 1_800_000_000;
const UPDATE_UNTIL = 1_900_000_000;
const CHECKED = 1_750_000_000;

function mount(props: Record<string, unknown> = {}) {
  return render(LicenceStatus, {
    props: {
      usability: { state: "active" },
      trustBasis: { kind: "offlineSignature" },
      useUntil: null,
      updateUntil: null,
      usable: true,
      attention: "none",
      ...props,
    } as never,
  });
}

function root(container: HTMLElement): HTMLElement {
  return container.querySelector(".poodle-licence-status") as HTMLElement;
}

describe("LicenceStatus (svelte)", () => {
  const states: Array<[string, LicenceUsability, string]> = [
    ["active", { state: "active" }, "Licence active"],
    ["inGrace", { state: "inGrace", until: USE_UNTIL }, "Licence active"],
    ["useWindowExpired", { state: "useWindowExpired", at: CHECKED }, "Use coverage ended"],
    ["leaseLapsed", { state: "leaseLapsed", at: CHECKED }, "Licence confirmation required"],
    ["clockRefused", { state: "clockRefused" }, "Check this machine's clock"],
  ];

  for (const [name, usability, title] of states) {
    it(`renders the ${name} state distinctly`, () => {
      const { container } = mount({ usability });
      expect(root(container).dataset.state).toBe(name);
      expect(screen.getByText(title)).toBeTruthy();
    });
  }

  // The state the whole component exists to get right. A renewal that has not
  // landed is not the customer's failure, so nothing on screen may say it is.
  it("gives inGrace no warning or danger role, and never announces", () => {
    const { container } = mount({
      usability: { state: "inGrace", until: USE_UNTIL },
      attention: "actionable",
    });
    const element = root(container);
    expect(element.dataset.tone).not.toBe("warning");
    expect(element.dataset.tone).not.toBe("danger");
    expect(element.querySelector('[data-status="warning"]')).toBeNull();
    expect(element.querySelector('[data-status="danger"]')).toBeNull();
    expect(element.querySelector("[aria-live]")).toBeNull();
    expect(element.querySelector('[role="alert"]')).toBeNull();
    expect(element.querySelector('[role="status"]')).toBeNull();
    expect(element.className).not.toMatch(/warning|danger|error/);
  });

  it("carries the inGrace continuation deadline in the quiet detail row", () => {
    const { container } = mount({ usability: { state: "inGrace", until: USE_UNTIL } });
    const detail = container.querySelector(".poodle-licence-status__detail") as HTMLElement;
    expect(detail.textContent).toContain("Use continues until");
    expect(detail.querySelector("time")?.getAttribute("datetime")).toBe(
      new Date(USE_UNTIL * 1_000).toISOString(),
    );
    expect(detail.querySelector("time")?.textContent).toBe(
      formatDisplayTimeDate(USE_UNTIL * 1_000),
    );
    expect(detail.textContent).not.toMatch(/ago|from now|\bin\s+\d/i);
  });

  it("gives clockRefused the clock remedy and no expiry or purchase copy", () => {
    const { container } = mount({ usability: { state: "clockRefused" } });
    const text = root(container).textContent ?? "";
    expect(text).toMatch(/clock/i);
    expect(text).toMatch(/set the clock/i);
    expect(text).not.toMatch(/expir|invalid|revok|buy|purchase|renew|subscri/i);
  });

  it("keeps use and update coverage as two visible labelled values when both are open", () => {
    const { container } = mount({ useUntil: null, updateUntil: null });
    const terms = [...container.querySelectorAll(".poodle-licence-status__term")].map(
      (node) => node.textContent,
    );
    expect(terms).toContain("Use coverage");
    expect(terms).toContain("Update coverage");
    expect(container.querySelectorAll('[data-row="use"]')).toHaveLength(1);
    expect(container.querySelectorAll('[data-row="update"]')).toHaveLength(1);
    expect(screen.getAllByText("No end date")).toHaveLength(2);
  });

  it("keeps use and update coverage as two visible labelled values when both are dated", () => {
    const { container } = mount({ useUntil: USE_UNTIL, updateUntil: UPDATE_UNTIL });
    const terms = [...container.querySelectorAll(".poodle-licence-status__term")].map(
      (node) => node.textContent,
    );
    expect(terms).toContain("Use coverage");
    expect(terms).toContain("Updates");
    expect(container.querySelector('[data-row="use"] time')).toBeTruthy();
    expect(container.querySelector('[data-row="update"] time')).toBeTruthy();
  });

  it("keeps the two windows separate when only one has an end date", () => {
    const { container } = mount({ useUntil: USE_UNTIL, updateUntil: null });
    expect(container.querySelector('[data-row="use"] time')).toBeTruthy();
    expect(container.querySelector('[data-row="update"]')?.textContent).toContain("No end date");
  });

  it("uses grammatical deadline phrases for future coverage", () => {
    const future = Math.floor(Date.now() / 1_000) + 240 * 86_400;
    const { container } = mount({ useUntil: future, updateUntil: future });

    expect(container.querySelector('[data-row="use"]')?.textContent?.trim()).toMatch(/^ends in /);
    expect(container.querySelector('[data-row="update"]')?.textContent?.trim()).toMatch(/^end in /);
  });

  it("uses grammatical deadline phrases for elapsed coverage", () => {
    const past = Math.floor(Date.now() / 1_000) - 240 * 86_400;
    const { container } = mount({ useUntil: past, updateUntil: past });

    expect(container.querySelector('[data-row="use"]')?.textContent?.trim()).toMatch(/^ended .* ago$/);
    expect(container.querySelector('[data-row="update"]')?.textContent?.trim()).toMatch(/^ended .* ago$/);
  });

  it("renders both trust bases distinctly", () => {
    const offline = mount({ trustBasis: { kind: "offlineSignature" } });
    expect(offline.container.querySelector('[data-row="trust"]')?.textContent).toContain(
      "verified on this machine",
    );
    offline.unmount();

    const remote = mount({ trustBasis: { kind: "remoteAssertion", checked: CHECKED } });
    const cell = remote.container.querySelector('[data-row="trust"]') as HTMLElement;
    expect(cell.textContent).toContain("confirmed");
    expect(cell.querySelector("time")).toBeTruthy();
  });

  // `usable` is a report, not a permission. Nothing may key a control off it.
  it("changes only reported state when usable flips", () => {
    const on = mount({ usable: true });
    const off = mount({ usable: false });

    for (const view of [on, off]) {
      expect(view.container.querySelectorAll("button")).toHaveLength(0);
      expect(view.container.querySelectorAll("[disabled]")).toHaveLength(0);
      expect(view.container.querySelectorAll("[aria-disabled]")).toHaveLength(0);
      expect(view.container.querySelectorAll(".poodle-licence-status__term")).toHaveLength(3);
    }
    expect(root(on.container).dataset.usable).toBe("true");
    expect(root(off.container).dataset.usable).toBe("false");
  });

  it("labels the section and reports attention without interrupting", () => {
    const { container } = mount({ attention: "informational", title: "Licence" });
    const element = root(container);
    expect(element.tagName).toBe("SECTION");
    expect(element.getAttribute("aria-label")).toBe("Licence");
    expect(element.dataset.attention).toBe("informational");
    expect(element.dataset.tone).toBe("info");
    expect(element.querySelector("[aria-live]")).toBeNull();
  });
});
