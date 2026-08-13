import { fireEvent, render, screen } from "@testing-library/svelte";
import { tick } from "svelte";
import { describe, expect, it } from "vitest";

import UpdateCenter from "../src/UpdateCenter.svelte";

const offer = {
  presence: "attention",
  status: { kind: "ready" },
  availability: { state: "offer", version: "1.4.0", reason: "staged", notes: null },
} as const;

describe("UpdateCenter (svelte)", () => {
  // A host is invited to pass plain reads off a non-reactive controller plus
  // `observe`. Svelte 5 props are lazy getters, so anything re-read inside a
  // notify-tracked derived picks up fresh values. Anything read straight in
  // the template does not — and `presence` gates the whole component, so the
  // failure mode is "the icon never appears", indistinguishable from working.
  it("renders the trigger once the authority notifies, having started hidden", async () => {
    const observers: Array<() => void> = [];
    const controller = { presence: "hidden" as string };

    const { container } = render(UpdateCenter, {
      props: {
        ...offer,
        get presence() {
          return controller.presence;
        },
        observe: (fn: () => void) => {
          observers.push(fn);
          return () => observers.splice(observers.indexOf(fn), 1);
        },
      } as never,
    });

    expect(container.querySelector(".poodle-update-center")).toBeNull();

    // The authority finds an update and notifies. Nothing else changes.
    controller.presence = "attention";
    observers.forEach((fn) => fn());
    await tick();

    expect(container.querySelector(".poodle-update-center")).not.toBeNull();
    expect(screen.getByRole("button", { name: "Updates" })).toBeTruthy();
  });

  it("renders nothing at all when presence is hidden", () => {
    const { container } = render(UpdateCenter, { props: { presence: "hidden" } });

    expect(container.querySelector(".poodle-update-center")).toBeNull();
    expect(screen.queryByRole("button")).toBeNull();
  });

  it("draws the eye when presence is attention", () => {
    const { container } = render(UpdateCenter, { props: offer });

    expect(screen.getByRole("button", { name: "Updates" })).toBeTruthy();
    expect(container.querySelector(".poodle-update-center__indicator")).toBeTruthy();
  });

  it("shows an unremarkable trigger when presence is quiet", () => {
    const { container } = render(UpdateCenter, { props: { ...offer, presence: "quiet" } });

    expect(screen.getByRole("button", { name: "Updates" })).toBeTruthy();
    expect(container.querySelector(".poodle-update-center__indicator")).toBeNull();
  });

  it("shows the offer inside the popover", async () => {
    render(UpdateCenter, { props: offer });

    await fireEvent.click(screen.getByRole("button", { name: "Updates" }));

    expect(screen.getByRole("dialog", { name: "Updates" })).toBeTruthy();
    expect(screen.getByText("Version 1.4.0 is available")).toBeTruthy();
    expect(screen.getByRole("button", { name: "Install and restart" })).toBeTruthy();
  });

  it("swaps the icon for a determinate progress ring while downloading", () => {
    const { container } = render(UpdateCenter, {
      props: { presence: "quiet", progress: { state: "downloading", fraction: 0.42 } },
    });

    const ring = container.querySelector(".poodle-update-center__ring");
    expect(ring).toBeTruthy();
    expect(ring?.getAttribute("data-indeterminate")).toBe("false");
    expect(container.querySelector(".poodle-icon")).toBeNull();
    expect(container.querySelector(".poodle-update-center__ring-fill")?.getAttribute("stroke-dasharray")).toBeTruthy();
    expect(screen.getByRole("button", { name: "Downloading update, 42%" })).toBeTruthy();
  });

  it("swaps the icon for an indeterminate ring when the fraction is null", () => {
    const { container } = render(UpdateCenter, {
      props: { presence: "quiet", progress: { state: "downloading", fraction: null } },
    });

    const ring = container.querySelector(".poodle-update-center__ring");
    expect(ring?.getAttribute("data-indeterminate")).toBe("true");
    expect(container.querySelector(".poodle-update-center__ring-fill")?.getAttribute("stroke-dasharray")).toBeNull();
    expect(screen.getByRole("button", { name: "Downloading update" })).toBeTruthy();
  });
});
