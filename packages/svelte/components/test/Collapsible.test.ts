import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Collapsible from "../src/Collapsible.svelte";
import { asSnippet } from "./snippet";

// happy-dom lacks the Web Animations API, which the content's
// `transition:slide` calls through `element.animate`. Same polyfill precedent
// as DrawerDismissOutside.svelte.test.ts.
if (!("animate" in Element.prototype)) {
  (Element.prototype as unknown as { animate: () => unknown }).animate = () => {
    const animation = {
      onfinish: null as (() => void) | null,
      cancel: () => {},
      playState: "finished",
      currentTime: 0,
      effect: null,
      finished: Promise.resolve(),
    };
    queueMicrotask(() => animation.onfinish?.());
    return animation;
  };
}

describe("Collapsible (svelte)", () => {
  it("renders content open from defaultOpen and toggles via the trigger", async () => {
    const { container, getByRole } = render(Collapsible, {
      props: {
        title: "Details",
        defaultOpen: true,
        children: asSnippet(() => "Content"),
      },
    });
    const trigger = getByRole("button");
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(container.querySelector(".poodle-collapsible__content")).not.toBeNull();

    await fireEvent.click(trigger);
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(container.querySelector(".poodle-collapsible__content")).toBeNull();
  });

  it("keeps content closed until the trigger is pressed", async () => {
    const { container, getByRole } = render(Collapsible, {
      props: { title: "Details", children: asSnippet(() => "Content") },
    });
    const trigger = getByRole("button");
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(container.querySelector(".poodle-collapsible__content")).toBeNull();

    await fireEvent.click(trigger);
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
  });

  it("reports open changes and refuses to toggle when disabled", async () => {
    const onOpenChange = vi.fn();
    const { getByRole } = render(Collapsible, {
      props: { title: "Details", onOpenChange },
    });
    const trigger = getByRole("button");

    await fireEvent.click(trigger);
    expect(onOpenChange).toHaveBeenCalledWith(true);

    const disabled = render(Collapsible, { props: { title: "Details", disabled: true } });
    const disabledTrigger = disabled.container.querySelector("button") as HTMLButtonElement;
    expect(disabledTrigger.disabled).toBe(true);
    await fireEvent.click(disabledTrigger);
    expect(disabledTrigger.getAttribute("aria-expanded")).toBe("false");
  });

  it("projects the open state from a controlled prop", async () => {
    const { container, getByRole } = render(Collapsible, {
      props: { open: false, children: asSnippet(() => "Content") },
    });
    expect(getByRole("button").getAttribute("aria-expanded")).toBe("false");
    expect(container.querySelector(".poodle-collapsible__content")).toBeNull();
  });
});
