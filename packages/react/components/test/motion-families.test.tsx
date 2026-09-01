import { act, cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { liveWebMotionCount } from "@inflatable-cookie/poodle-core";

import { Checkbox } from "../src/Checkbox";
import { Accordion } from "../src/Accordion";
import { Collapsible } from "../src/Collapsible";
import { IconButton } from "../src/IconButton";
import { MotionPolicyProvider } from "../src/motion-policy";
import { Skeleton } from "../src/Skeleton";
import { Spinner } from "../src/Spinner";
import { Tabs } from "../src/Tabs";
import { ToastStack } from "../src/ToastStack";

type PresenceHold = {
  finish: () => void;
  keyframes: Keyframe[];
  options?: KeyframeAnimationOptions;
};

const presenceHolds: PresenceHold[] = [];
const animationCalls: PresenceHold[] = [];

(Element.prototype as unknown as { animate: (keyframes: Keyframe[], options?: KeyframeAnimationOptions) => Animation }).animate =
  function animate(keyframes, options) {
    let settled = false;
    let resolve!: (value: unknown) => void;
    let reject!: (reason?: unknown) => void;
    const finished = new Promise((res, fail) => {
      resolve = res;
      reject = fail;
    });
    const hold: PresenceHold = {
      keyframes,
      options,
      finish() {
        if (settled) {
          return;
        }
        settled = true;
        resolve(undefined);
      },
    };
    presenceHolds.push(hold);
    animationCalls.push(hold);
    return {
      cancel() {
        if (settled) {
          return;
        }
        settled = true;
        reject(new DOMException("The user aborted a request.", "AbortError"));
      },
      finished,
    } as unknown as Animation;
  };

async function frame(): Promise<void> {
  await new Promise((resolve) => setTimeout(resolve, 20));
}

async function finishPresence(): Promise<void> {
  const current = presenceHolds.splice(0);
  for (const hold of current) {
    hold.finish();
  }
  await Promise.resolve();
  await Promise.resolve();
  await frame();
}

describe("g16.034 mounted family receipts (react)", () => {
  afterEach(() => {
    presenceHolds.length = 0;
    animationCalls.length = 0;
    cleanup();
  });

  it("preloaded toasts are settled, inert on exit, and drop from WAAPI finished", async () => {
    const { container, rerender } = render(<ToastStack items={[{ id: "save", title: "Saved" }]} />);
    const toast = container.querySelector(".poodle-toast") as HTMLElement;
    expect(toast.dataset.motion).toBe("settled");

    rerender(<ToastStack items={[]} />);
    const exiting = container.querySelector(".poodle-toast") as HTMLElement;
    expect(exiting.dataset.motion).toBe("exit");
    expect(exiting.getAttribute("aria-hidden")).toBe("true");
    expect(exiting.hasAttribute("inert")).toBe(true);
    expect(exiting.querySelector("button")?.getAttribute("tabindex")).toBe("-1");
    await act(async () => {
      await finishPresence();
    });
    expect(container.querySelector(".poodle-toast")).toBeNull();
  });

  it("two stacks with the same toast id keep independent clocks", () => {
    const first = render(<ToastStack items={[]} />);
    const second = render(<ToastStack items={[]} />);
    first.rerender(<ToastStack items={[{ id: "save", title: "One" }]} />);
    second.rerender(<ToastStack items={[{ id: "save", title: "Two" }]} />);
    expect(first.container.querySelector(".poodle-toast")?.getAttribute("data-motion")).toBe("enter");
    expect(second.container.querySelector(".poodle-toast")?.getAttribute("data-motion")).toBe("enter");
    expect(liveWebMotionCount()).toBeGreaterThanOrEqual(2);
    first.unmount();
    expect(second.container.querySelector(".poodle-toast")).not.toBeNull();
    expect(liveWebMotionCount()).toBeGreaterThanOrEqual(1);
    second.unmount();
  });

  it("animated=false never becomes motion-ready", async () => {
    const { container } = render(<Skeleton animated={false} />);
    await act(async () => {
      await frame();
    });
    const root = container.querySelector("[data-animated]") as HTMLElement;
    expect(root.dataset.animated).toBe("false");
    expect(root.getAttribute("data-motion-ready")).not.toBe("true");
  });

  it("keeps closed disclosure content in layout until the clip finishes", async () => {
    const { container, getByRole } = render(
      <Collapsible title="Details" defaultOpen>
        Content
      </Collapsible>,
    );
    await act(async () => {
      await frame();
    });
    expect(container.querySelector("[data-motion-ready='true']")).not.toBeNull();
    fireEvent.click(getByRole("button"));
    const content = container.querySelector(".poodle-collapsible__content") as HTMLElement;
    expect(content.hasAttribute("inert")).toBe(true);
    expect(content.hasAttribute("hidden")).toBe(false);
  });

  it("rapid controlled Collapsible and Accordion reversal uses the live height", async () => {
    const collapsible = render(
      <Collapsible open title="Details">
        Content
      </Collapsible>,
    );
    await act(async () => {
      await frame();
    });
    const collapsibleClip = collapsible.container.querySelector(
      ".poodle-collapsible__content-clip",
    ) as HTMLElement;
    Object.defineProperty(collapsibleClip, "scrollHeight", { configurable: true, value: 80 });

    const accordion = render(
      <Accordion
        items={[{ value: "one", label: "One" }]}
        selectionMode="multiple"
        value={["one"]}
      >
        {() => "Content"}
      </Accordion>,
    );
    await act(async () => {
      await frame();
    });
    const accordionClip = accordion.container.querySelector(
      ".poodle-accordion__panel-clip",
    ) as HTMLElement;
    Object.defineProperty(accordionClip, "scrollHeight", { configurable: true, value: 80 });
    animationCalls.length = 0;

    await act(async () => {
      collapsible.rerender(
        <Collapsible open={false} title="Details">
          Content
        </Collapsible>,
      );
      accordion.rerender(
        <Accordion
          items={[{ value: "one", label: "One" }]}
          selectionMode="multiple"
          value={[]}
        >
          {() => "Content"}
        </Accordion>,
      );
    });
    expect(collapsible.container.querySelector(".poodle-collapsible__content")?.hasAttribute("hidden")).toBe(false);
    expect(accordion.container.querySelector(".poodle-accordion__panel")?.hasAttribute("hidden")).toBe(false);

    collapsibleClip.style.height = "60px";
    accordionClip.style.height = "60px";
    await act(async () => {
      collapsible.rerender(
        <Collapsible open title="Details">
          Content
        </Collapsible>,
      );
      accordion.rerender(
        <Accordion
          items={[{ value: "one", label: "One" }]}
          selectionMode="multiple"
          value={["one"]}
        >
          {() => "Content"}
        </Accordion>,
      );
    });

    const reversalCalls = animationCalls.slice(-2);
    expect(reversalCalls).toHaveLength(2);
    for (const call of reversalCalls) {
      expect(call.options?.duration).toBe(45);
      expect(call.keyframes[0]).toEqual({ height: "60px" });
    }
    expect(collapsible.container.querySelector(".poodle-collapsible__content")?.hasAttribute("hidden")).toBe(false);
    expect(accordion.container.querySelector(".poodle-accordion__panel")?.hasAttribute("hidden")).toBe(false);
    expect(liveWebMotionCount()).toBe(2);
  });

  it("reduced IconButton lives under the reduced policy hook", () => {
    const { container } = render(
      <MotionPolicyProvider policy="reduced">
        <IconButton ariaLabel="Star" icon="star" />
      </MotionPolicyProvider>,
    );
    const provider = container.querySelector(".poodle-motion-policy-provider") as HTMLElement;
    const button = container.querySelector(".poodle-icon-button") as HTMLElement;
    expect(provider.getAttribute("data-poodle-motion-policy")).toBe("reduced");
    expect(provider.contains(button)).toBe(true);
  });

  it("frozen Spinner never becomes motion-ready", async () => {
    const { container } = render(
      <MotionPolicyProvider policy="frozen">
        <Spinner />
      </MotionPolicyProvider>,
    );
    await act(async () => {
      await frame();
    });
    expect(container.querySelector("[data-poodle-motion-policy='frozen']")).not.toBeNull();
    expect(container.querySelector("[data-motion-ready='true']")).toBeNull();
  });

  it("tabs indicator observes the selected item and cancels rAF on teardown", () => {
    const spy = vi.spyOn(globalThis, "cancelAnimationFrame");
    const { container, unmount } = render(
      <Tabs
        items={[
          { value: "a", label: "A" },
          { value: "b", label: "B" },
        ]}
        value="a"
        activeEdge="underline"
      />,
    );
    expect(container.querySelector(".poodle-tabs__indicator")).not.toBeNull();
    unmount();
    expect(spy).toHaveBeenCalled();
    spy.mockRestore();
  });

  it("checkbox checked state commits immediately", () => {
    const { getByRole } = render(<Checkbox label="Accept" />);
    const box = getByRole("checkbox") as HTMLInputElement;
    expect(box.checked).toBe(false);
    fireEvent.click(box);
    expect(box.checked).toBe(true);
  });
});
