import { fireEvent, render, screen } from "@testing-library/svelte";
import { tick } from "svelte";
import { describe, expect, it, vi } from "vitest";

import LicenceSeats from "../src/LicenceSeats.svelte";

const SEATS = [
  { machineId: "cmd-9f3a2b7c", label: "Studio Mac", thisMachine: true },
  { machineId: "cmd-41ee80d2", label: "Tour laptop", thisMachine: false },
  { machineId: "cmd-77c1a5be", label: null, thisMachine: false },
  { machineId: "cmd-2b90fe14", label: null, thisMachine: false },
];

function mount(props: Record<string, unknown> = {}) {
  const onRelease = vi.fn();
  const view = render(LicenceSeats, { props: { seats: SEATS, onRelease, ...props } as never });
  return { ...view, onRelease };
}

describe("LicenceSeats (svelte)", () => {
  it("renders nothing at all when the authority reports no seats", () => {
    const { container } = render(LicenceSeats, { props: { seats: [] } as never });
    expect(container.querySelector(".poodle-licence-seats")).toBeNull();
    expect(container.textContent?.trim()).toBe("");
    expect(container.textContent).not.toMatch(/seat/i);
  });

  it("names a labelled machine verbatim and an unnamed one honestly", () => {
    mount();
    expect(screen.getByText("Studio Mac")).toBeTruthy();
    expect(screen.getByText("Tour laptop")).toBeTruthy();
    // Two unnamed rows look alike. Inventing a hostname to tell them apart
    // would be Poodle claiming identity it was never given.
    expect(screen.getAllByText("Unnamed machine")).toHaveLength(2);
  });

  it("marks this machine and gives it no release action", () => {
    const { container } = mount();
    const rows = [...container.querySelectorAll(".poodle-licence-seats__row")];
    const thisMachine = rows.find((row) => row.getAttribute("data-this-machine") === "true");
    expect(thisMachine?.textContent).toContain("This machine");
    expect(thisMachine?.querySelector("button")).toBeNull();
    expect(rows.filter((row) => row.getAttribute("data-this-machine") === "true")).toHaveLength(1);
  });

  it("offers every other seat a release control with an honest accessible name", () => {
    mount();
    expect(screen.getByRole("button", { name: "Release Tour laptop" })).toBeTruthy();
    expect(screen.getAllByRole("button", { name: "Release unnamed machine" })).toHaveLength(2);
    expect(screen.getAllByRole("button")).toHaveLength(3);
  });

  it("emits the exact machine ID once the release is confirmed", async () => {
    const { onRelease } = mount();
    await fireEvent.click(screen.getByRole("button", { name: "Release Tour laptop" }));
    await tick();
    expect(screen.getByText("Release this seat?")).toBeTruthy();
    // The row's own name and the confirmation body — the same honest label,
    // never an ID.
    expect(screen.getAllByText("Tour laptop")).toHaveLength(2);
    expect(onRelease).not.toHaveBeenCalled();

    const dialog = screen.getByRole("alertdialog");
    await fireEvent.click(
      [...dialog.querySelectorAll("button")].find((button) => button.textContent?.trim() === "Release")!,
    );
    await tick();
    expect(onRelease).toHaveBeenCalledWith({ machineId: "cmd-41ee80d2" });
  });

  it("emits directly when the host turns confirmation off", async () => {
    const { onRelease } = mount({ confirmRelease: false });
    await fireEvent.click(screen.getByRole("button", { name: "Release Tour laptop" }));
    await tick();
    expect(onRelease).toHaveBeenCalledWith({ machineId: "cmd-41ee80d2" });
  });

  it("never exposes a machine ID in rendered or accessible text", () => {
    const { container } = mount();
    const html = container.innerHTML;
    for (const seat of SEATS) {
      expect(container.textContent ?? "").not.toContain(seat.machineId);
      // Not shortened, not in a title, not in an accessible name.
      expect(html).not.toContain(seat.machineId);
      expect(html).not.toContain(seat.machineId.slice(4, 12));
    }
    expect(container.querySelectorAll("[title]")).toHaveLength(0);
  });

  it("says nothing about hostnames, platforms, last-seen, or seat limits", () => {
    const { container } = mount();
    expect(container.textContent).not.toMatch(
      /hostname|platform|macos|windows|linux|last seen|of \d+ seats|seat limit/i,
    );
  });

  it("disables only the pending row's action", () => {
    mount({ pendingMachineId: "cmd-41ee80d2" });
    expect(
      screen.getByRole("button", { name: "Release Tour laptop" }).hasAttribute("disabled"),
    ).toBe(true);
    for (const button of screen.getAllByRole("button", { name: "Release unnamed machine" })) {
      expect(button.hasAttribute("disabled")).toBe(false);
    }
  });

  it("labels the section and lists the seats semantically", () => {
    const { container } = mount({ title: "Activated machines" });
    const section = container.querySelector(".poodle-licence-seats") as HTMLElement;
    expect(section.tagName).toBe("SECTION");
    expect(section.getAttribute("aria-label")).toBe("Activated machines");
    expect(container.querySelectorAll("ul > li")).toHaveLength(4);
  });
});
