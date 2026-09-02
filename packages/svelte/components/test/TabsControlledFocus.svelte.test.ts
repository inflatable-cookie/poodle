import { fireEvent, render, screen, within } from "@testing-library/svelte";
import { tick } from "svelte";
import type { ComponentProps } from "svelte";
import { describe, expect, it, vi } from "vitest";

import Tabs from "../src/Tabs.svelte";
import TabsControlledFocusHarness from "./TabsControlledFocusHarness.svelte";

type FocusPolicy = NonNullable<ComponentProps<typeof Tabs>["focusOnValueChange"]>;

const preserve: FocusPolicy = "preserve";
const selectedTab: FocusPolicy = "selected-tab";

async function flush(): Promise<void> {
  await tick();
  await new Promise((resolve) => setTimeout(resolve, 0));
}

function inspector(): HTMLElement {
  return screen.getByRole("tablist", { name: "Inspector" });
}

function inspectorTab(name: string): HTMLElement {
  return within(inspector()).getByRole("tab", { name });
}

describe("Tabs controlled-panel focus (svelte)", () => {
  it("exports preserve and selected-tab on the public prop", () => {
    expect([preserve, selectedTab]).toEqual(["preserve", "selected-tab"]);
  });

  it("transfers from an outgoing interactive descendant to the Tree tab exactly once", async () => {
    render(TabsControlledFocusHarness, { props: { focusOnValueChange: "selected-tab" } });
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    await fireEvent.click(screen.getByTestId("select-tree"));
    await flush();

    expect(document.activeElement).toBe(treeTab);
    expect(focus).toHaveBeenCalledTimes(1);
    expect(screen.queryByTestId("list-card")).toBeNull();
  });

  it("default preserve does not invoke tab focus when the outgoing panel unmounts", async () => {
    render(TabsControlledFocusHarness, {});
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    await fireEvent.click(screen.getByTestId("select-tree"));
    await flush();

    expect(focus).not.toHaveBeenCalled();
    expect(document.activeElement).not.toBe(treeTab);
  });

  it("keeps focus outside the outgoing panel, including another Tabs instance", async () => {
    render(TabsControlledFocusHarness, { props: { focusOnValueChange: "selected-tab" } });
    const outside = screen.getByTestId("outside");
    outside.focus();
    await fireEvent.click(screen.getByTestId("select-tree"));
    await flush();
    expect(document.activeElement).toBe(outside);

    const other = screen.getByTestId("other-panel");
    other.focus();
    await fireEvent.click(screen.getByTestId("select-tree"));
    await flush();
    expect(document.activeElement).toBe(other);
  });

  it("does not steal focus that is already on a tab", async () => {
    render(TabsControlledFocusHarness, { props: { focusOnValueChange: "selected-tab" } });
    const componentsTab = inspectorTab("Components");
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    componentsTab.focus();

    await fireEvent.click(screen.getByTestId("select-tree"));
    await flush();

    expect(focus).not.toHaveBeenCalled();
    expect(document.activeElement).toBe(componentsTab);
  });

  it("does not focus a missing destination", async () => {
    render(TabsControlledFocusHarness, { props: { focusOnValueChange: "selected-tab" } });
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    await fireEvent.click(screen.getByTestId("select-missing"));
    await flush();

    expect(focus).not.toHaveBeenCalled();
    expect(document.activeElement).not.toBe(treeTab);
  });

  it("does not focus a disabled destination", async () => {
    render(TabsControlledFocusHarness, {
      props: {
        focusOnValueChange: "selected-tab",
        items: [
          { value: "components", label: "Components" },
          { value: "tree", label: "Tree", disabled: true },
        ],
      },
    });
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    await fireEvent.click(screen.getByTestId("select-tree"));
    await flush();

    expect(focus).not.toHaveBeenCalled();
    expect(document.activeElement).not.toBe(treeTab);
  });

  it("revalidates a destination disabled before the pending timer fires", async () => {
    render(TabsControlledFocusHarness, { props: { focusOnValueChange: "selected-tab" } });
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    await fireEvent.click(screen.getByTestId("stale-disable"));
    await flush();

    expect(focus).not.toHaveBeenCalled();
    expect((treeTab as HTMLButtonElement).disabled).toBe(true);
  });

  it("invalidates a pending destination when policy changes to preserve", async () => {
    render(TabsControlledFocusHarness, { props: { focusOnValueChange: "selected-tab" } });
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    await fireEvent.click(screen.getByTestId("stale-policy"));
    await flush();

    expect(focus).not.toHaveBeenCalled();
    expect(document.activeElement).not.toBe(treeTab);
  });

  it("superseded Components → Preview → Tree focuses only the Tree tab once", async () => {
    render(TabsControlledFocusHarness, { props: { focusOnValueChange: "selected-tab" } });
    const previewTab = inspectorTab("Preview");
    const treeTab = inspectorTab("Tree");
    const previewFocus = vi.spyOn(previewTab, "focus");
    const treeFocus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    await fireEvent.click(screen.getByTestId("supersede"));
    await flush();

    expect(previewFocus).not.toHaveBeenCalled();
    expect(treeFocus).toHaveBeenCalledTimes(1);
    expect(document.activeElement).toBe(treeTab);
  });
  it("separate-commit supersession retargets the latched transfer to the final tab", async () => {
    render(TabsControlledFocusHarness, { props: { focusOnValueChange: "selected-tab" } });
    const previewTab = inspectorTab("Preview");
    const treeTab = inspectorTab("Tree");
    const previewFocus = vi.spyOn(previewTab, "focus");
    const treeFocus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    await fireEvent.click(screen.getByTestId("supersede-commits"));
    await flush();

    expect(previewFocus).not.toHaveBeenCalled();
    expect(treeFocus).toHaveBeenCalledTimes(1);
    expect(document.activeElement).toBe(treeTab);
  });

  it("teardown makes a pending transfer inert", async () => {
    render(TabsControlledFocusHarness, { props: { focusOnValueChange: "selected-tab" } });
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    await fireEvent.click(screen.getByTestId("teardown"));
    await flush();

    expect(screen.queryByRole("tablist", { name: "Inspector" })).toBeNull();
    expect(focus).not.toHaveBeenCalled();
  });

  it("async accepted Components → Tree with a ListCard descendant focuses only the Tree tab", async () => {
    render(TabsControlledFocusHarness, { props: { focusOnValueChange: "selected-tab" } });
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();
    expect(document.activeElement).toBe(screen.getByTestId("list-card"));

    await fireEvent.click(screen.getByTestId("accept-open"));
    await Promise.resolve();
    await flush();

    expect(screen.queryByTestId("list-card")).toBeNull();
    expect(document.activeElement).toBe(treeTab);
    expect(focus).toHaveBeenCalledTimes(1);
  });
});
