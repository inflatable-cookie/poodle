import { act } from "react";
import { hydrateRoot } from "react-dom/client";
import { renderToString } from "react-dom/server";
import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Button, Popover, type PopoverTriggerState } from "../src";

/**
 * Focused evidence for the g15.041 interactive trigger contract
 * (docs/contracts/components/popover.md §3, "Trigger Modes"): the wrapper is
 * inert decoration in interactive mode and the caller's real control carries
 * the disclosure ARIA from the PopoverTriggerState payload.
 */

function StatefulTrigger({ state, label = "Open popover" }: { state: PopoverTriggerState; label?: string }) {
  return (
    <Button variant="secondary" ariaExpanded={state.expanded} controls={state.controls} disabled={state.disabled}>
      {label}
    </Button>
  );
}

function surface(): HTMLElement | null {
  // The surface is portalled to the theme root (document.body in tests).
  return document.body.querySelector<HTMLElement>('[role="dialog"]');
}

describe("Popover — interactive trigger semantics", () => {
  it("keeps the wrapper roleless and untabbable; the real control carries the disclosure ARIA", () => {
    const { container } = render(
      <Popover
        triggerIsInteractive
        trigger={(state) => <StatefulTrigger state={state} />}
      >
        <p>Body</p>
      </Popover>,
    );

    const wrapper = container.querySelector<HTMLElement>(".poodle-popover__trigger")!;
    expect(wrapper.getAttribute("role")).toBeNull();
    expect(wrapper.getAttribute("tabindex")).toBeNull();
    expect(wrapper.getAttribute("aria-expanded")).toBeNull();
    expect(wrapper.getAttribute("aria-controls")).toBeNull();
    expect(wrapper.getAttribute("aria-disabled")).toBeNull();

    // Exactly one operable control inside the wrapper.
    const controls = wrapper.querySelectorAll("button");
    expect(controls).toHaveLength(1);

    const trigger = screen.getByRole("button", { name: "Open popover" });
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(trigger.getAttribute("aria-controls")).toBeNull();

    fireEvent.click(trigger);

    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    const dialog = surface();
    expect(dialog).not.toBeNull();
    expect(trigger.getAttribute("aria-controls")).toBe(dialog!.id);
  });

  it("opens and closes from the real control, repeatedly, and restores focus to it", () => {
    render(
      <Popover
        triggerIsInteractive
        trigger={(state) => <StatefulTrigger state={state} />}
      >
        <button type="button" data-testid="surface-action">
          Surface action
        </button>
      </Popover>,
    );

    const trigger = screen.getByRole("button", { name: "Open popover" });
    // The operable control is a native button: keyboard activation comes from
    // the browser, not from a wrapper handler.
    expect(trigger.tagName).toBe("BUTTON");

    for (let round = 0; round < 3; round += 1) {
      fireEvent.click(trigger);
      expect(surface()).not.toBeNull();
      expect(trigger.getAttribute("aria-expanded")).toBe("true");

      fireEvent.click(trigger);
      expect(surface()).toBeNull();
      expect(trigger.getAttribute("aria-expanded")).toBe("false");
      expect(trigger.getAttribute("aria-controls")).toBeNull();
    }

    // Escape close restores focus to the real control, not the wrapper.
    fireEvent.click(trigger);
    screen.getByTestId("surface-action").focus();
    fireEvent.keyDown(document, { key: "Escape" });
    expect(surface()).toBeNull();
    expect(document.activeElement).toBe(trigger);

    // Outside interact close restores focus to the real control as well.
    fireEvent.click(trigger);
    fireEvent.mouseDown(document.body);
    expect(surface()).toBeNull();
    expect(document.activeElement).toBe(trigger);
  });

  it("stays inert for a controlled open request until the host confirms", () => {
    const onOpenChange = vi.fn();
    const { rerender } = render(
      <Popover
        open={false}
        triggerIsInteractive
        onOpenChange={onOpenChange}
        trigger={(state) => <StatefulTrigger state={state} />}
      >
        <p>Body</p>
      </Popover>,
    );

    const trigger = screen.getByRole("button", { name: "Open popover" });
    fireEvent.click(trigger);
    expect(onOpenChange).toHaveBeenCalledWith(true);
    expect(surface()).toBeNull();

    rerender(
      <Popover
        open
        triggerIsInteractive
        onOpenChange={onOpenChange}
        trigger={(state) => <StatefulTrigger state={state} />}
      >
        <p>Body</p>
      </Popover>,
    );
    expect(surface()).not.toBeNull();
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
  });

  it("threads disabled to the real control and blocks opening", () => {
    const onOpenChange = vi.fn();
    const { container } = render(
      <Popover
        disabled
        triggerIsInteractive
        onOpenChange={onOpenChange}
        trigger={(state) => <StatefulTrigger state={state} />}
      >
        <p>Body</p>
      </Popover>,
    );

    const trigger = screen.getByRole("button", { name: "Open popover" });
    expect((trigger as HTMLButtonElement).disabled).toBe(true);
    expect(container.querySelector(".poodle-popover__trigger")!.getAttribute("data-disabled")).toBe("true");

    fireEvent.click(trigger);
    expect(surface()).toBeNull();
    expect(onOpenChange).not.toHaveBeenCalled();
  });

  it("leaves default mode untouched: the wrapper owns the button semantics", () => {
    const { container } = render(
      <Popover trigger={<span>Open popover</span>}>
        <p>Body</p>
      </Popover>,
    );

    const wrapper = container.querySelector<HTMLElement>(".poodle-popover__trigger")!;
    expect(wrapper.getAttribute("role")).toBe("button");
    expect(wrapper.getAttribute("tabindex")).toBe("0");
    expect(wrapper.getAttribute("aria-expanded")).toBe("false");

    fireEvent.keyDown(wrapper, { key: "Enter" });
    expect(surface()).not.toBeNull();
    expect(wrapper.getAttribute("aria-expanded")).toBe("true");
    expect(wrapper.getAttribute("aria-controls")).toBe(surface()!.id);

    fireEvent.keyDown(wrapper, { key: " " });
    expect(surface()).toBeNull();
  });

  describe("SSR (renderToString + hydration)", () => {
    function ssrElement(defaultOpen: boolean) {
      return (
        <Popover
          defaultOpen={defaultOpen}
          triggerIsInteractive
          ariaLabel="SSR popover"
          trigger={(state) => <StatefulTrigger state={state} />}
        >
          <p>Body</p>
        </Popover>
      );
    }

    it("emits correct closed markup with no aria-controls", () => {
      const html = renderToString(ssrElement(false));
      expect(html).toContain('aria-expanded="false"');
      expect(html).not.toContain("aria-controls");
      expect(html).not.toContain('role="dialog"');
      // The wrapper carries no button semantics in interactive mode.
      expect(html).not.toContain('role="button"');
    });

    it("emits the open trigger's aria-controls and hydrates without repairing it", async () => {
      const element = ssrElement(true);
      const html = renderToString(element);
      expect(html).toContain('aria-expanded="true"');
      const controls = /aria-controls="([^"]+)"/.exec(html)?.[1];
      expect(controls).toBeTruthy();

      // The surface is portalled, so it cannot exist in server markup; the
      // SSR contract is that hydration reproduces the *same* id, letting the
      // surface appear under the id the server already advertised — no
      // post-mount repair of aria-controls. Any hydration mismatch logs
      // console.error, which the shared vitest setup turns into a failure.
      const container = document.createElement("div");
      document.body.appendChild(container);
      container.innerHTML = html;

      let root: ReturnType<typeof hydrateRoot> | undefined;
      await act(async () => {
        root = hydrateRoot(container, element);
      });

      const dialog = surface();
      expect(dialog).not.toBeNull();
      expect(dialog!.id).toBe(controls);

      const trigger = screen.getByRole("button", { name: "Open popover" });
      expect(trigger.getAttribute("aria-controls")).toBe(controls);

      await act(async () => root?.unmount());
      container.remove();
    });
  });
});
