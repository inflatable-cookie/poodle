import { act, fireEvent, render, screen, within } from "@testing-library/react";
import { flushSync } from "react-dom";
import { useState, type ReactNode } from "react";
import { describe, expect, it, vi } from "vitest";

import { Tabs, type TabsProps } from "../src/Tabs";
import type { TabItem } from "../src/types";

type FocusPolicy = NonNullable<TabsProps["focusOnValueChange"]>;

const preserve: FocusPolicy = "preserve";
const selectedTab: FocusPolicy = "selected-tab";

const defaultItems: TabItem[] = [
  { value: "components", label: "Components" },
  { value: "preview", label: "Preview" },
  { value: "tree", label: "Tree" },
];

const otherItems: TabItem[] = [
  { value: "other-a", label: "Other A" },
  { value: "other-b", label: "Other B" },
];

async function flush(): Promise<void> {
  await act(async () => {
    await new Promise((resolve) => setTimeout(resolve, 0));
  });
}

function inspector(): HTMLElement {
  return screen.getByRole("tablist", { name: "Inspector" });
}

function inspectorTab(name: string): HTMLElement {
  return within(inspector()).getByRole("tab", { name });
}

function panel(activeValue: string): ReactNode {
  if (activeValue === "components") {
    return <button type="button" data-testid="list-card">ListCard row</button>;
  }
  if (activeValue === "tree") {
    return <button type="button" data-testid="tree-return">Return to screen</button>;
  }
  return <button type="button" data-testid="preview-panel">Preview body</button>;
}

function Harness({
  items = defaultItems,
  initialValue = "components",
  focusOnValueChange = "preserve",
}: {
  items?: TabItem[];
  initialValue?: string;
  focusOnValueChange?: FocusPolicy;
}) {
  const [value, setValue] = useState(initialValue);
  const [alive, setAlive] = useState(true);
  const [liveItems, setLiveItems] = useState(items);
  const [livePolicy, setLivePolicy] = useState(focusOnValueChange);

  return (
    <>
      <button type="button" data-testid="outside">
        Outside
      </button>
      <button type="button" data-testid="select-tree" onClick={() => setValue("tree")}>
        Select Tree
      </button>
      <span data-testid="accept-open" onClick={() => {
          void Promise.resolve().then(() => setValue("tree"));
        }}>
        Accept Open
      </span>
      <button
        type="button"
        data-testid="supersede"
        onClick={() => {
          setValue("preview");
          setValue("tree");
        }}
      >
        Supersede
      </button>
      <button
        type="button"
        data-testid="supersede-commits"
        onClick={() => {
          flushSync(() => setValue("preview"));
          queueMicrotask(() => flushSync(() => setValue("tree")));
        }}
      >
        Supersede in separate commits
      </button>
      <button type="button" data-testid="select-missing" onClick={() => setValue("ghost")}>
        Select missing
      </button>
      <button
        type="button"
        data-testid="stale-disable"
        onClick={() => {
          flushSync(() => setValue("tree"));
          queueMicrotask(() =>
            flushSync(() =>
              setLiveItems((currentItems) =>
                currentItems.map((item) =>
                  item.value === "tree" ? { ...item, disabled: true } : item,
                ),
              ),
            ),
          );
        }}
      >
        Disable Tree
      </button>
      <button
        type="button"
        data-testid="stale-policy"
        onClick={() => {
          flushSync(() => setValue("tree"));
          queueMicrotask(() => flushSync(() => setLivePolicy("preserve")));
        }}
      >
        Preserve focus
      </button>
      <button
        type="button"
        data-testid="teardown"
        onClick={() => {
          setValue("tree");
          queueMicrotask(() => setAlive(false));
        }}
      >
        Teardown
      </button>
      {alive ? (
        <Tabs
          items={liveItems}
          value={value}
          focusOnValueChange={livePolicy}
          ariaLabel="Inspector"
          onValueChange={setValue}
        >
          {panel}
        </Tabs>
      ) : null}
      <Tabs items={otherItems} value="other-a" focusOnValueChange="selected-tab" ariaLabel="Other">
        {() => (
          <button type="button" data-testid="other-panel">
            Other panel
          </button>
        )}
      </Tabs>
    </>
  );
}

describe("Tabs controlled-panel focus (react)", () => {
  it("exports preserve and selected-tab on the public prop", () => {
    expect([preserve, selectedTab]).toEqual(["preserve", "selected-tab"]);
  });

  it("transfers from an outgoing interactive descendant to the Tree tab exactly once", async () => {
    render(<Harness focusOnValueChange="selected-tab" />);
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    fireEvent.click(screen.getByTestId("select-tree"));
    await flush();

    expect(document.activeElement).toBe(treeTab);
    expect(focus).toHaveBeenCalledTimes(1);
    expect(screen.queryByTestId("list-card")).toBeNull();
  });

  it("default preserve does not invoke tab focus when the outgoing panel unmounts", async () => {
    render(<Harness />);
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    fireEvent.click(screen.getByTestId("select-tree"));
    await flush();

    expect(focus).not.toHaveBeenCalled();
    expect(document.activeElement).not.toBe(treeTab);
  });

  it("keeps focus outside the outgoing panel, including another Tabs instance", async () => {
    render(<Harness focusOnValueChange="selected-tab" />);
    const outside = screen.getByTestId("outside");
    outside.focus();
    fireEvent.click(screen.getByTestId("select-tree"));
    await flush();
    expect(document.activeElement).toBe(outside);

    const other = screen.getByTestId("other-panel");
    other.focus();
    fireEvent.click(screen.getByTestId("select-tree"));
    await flush();
    expect(document.activeElement).toBe(other);
  });

  it("does not steal focus that is already on a tab", async () => {
    render(<Harness focusOnValueChange="selected-tab" />);
    const componentsTab = inspectorTab("Components");
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    componentsTab.focus();

    fireEvent.click(screen.getByTestId("select-tree"));
    await flush();

    expect(focus).not.toHaveBeenCalled();
    expect(document.activeElement).toBe(componentsTab);
  });

  it("does not focus a missing destination", async () => {
    render(<Harness focusOnValueChange="selected-tab" />);
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    fireEvent.click(screen.getByTestId("select-missing"));
    await flush();

    expect(focus).not.toHaveBeenCalled();
    expect(document.activeElement).not.toBe(treeTab);
  });

  it("does not focus a disabled destination", async () => {
    render(
      <Harness
        focusOnValueChange="selected-tab"
        items={[
          { value: "components", label: "Components" },
          { value: "tree", label: "Tree", disabled: true },
        ]}
      />,
    );
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    fireEvent.click(screen.getByTestId("select-tree"));
    await flush();

    expect(focus).not.toHaveBeenCalled();
    expect(document.activeElement).not.toBe(treeTab);
  });

  it("revalidates a destination disabled before the pending timer fires", async () => {
    render(<Harness focusOnValueChange="selected-tab" />);
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    fireEvent.click(screen.getByTestId("stale-disable"));
    await flush();

    expect(focus).not.toHaveBeenCalled();
    expect((treeTab as HTMLButtonElement).disabled).toBe(true);
  });

  it("invalidates a pending destination when policy changes to preserve", async () => {
    render(<Harness focusOnValueChange="selected-tab" />);
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    fireEvent.click(screen.getByTestId("stale-policy"));
    await flush();

    expect(focus).not.toHaveBeenCalled();
    expect(document.activeElement).not.toBe(treeTab);
  });

  it("superseded Components → Preview → Tree focuses only the Tree tab once", async () => {
    render(<Harness focusOnValueChange="selected-tab" />);
    const previewTab = inspectorTab("Preview");
    const treeTab = inspectorTab("Tree");
    const previewFocus = vi.spyOn(previewTab, "focus");
    const treeFocus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    fireEvent.click(screen.getByTestId("supersede"));
    await flush();

    expect(previewFocus).not.toHaveBeenCalled();
    expect(treeFocus).toHaveBeenCalledTimes(1);
    expect(document.activeElement).toBe(treeTab);
  });
  it("separate-commit supersession retargets the latched transfer to the final tab", async () => {
    render(<Harness focusOnValueChange="selected-tab" />);
    const previewTab = inspectorTab("Preview");
    const treeTab = inspectorTab("Tree");
    const previewFocus = vi.spyOn(previewTab, "focus");
    const treeFocus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    fireEvent.click(screen.getByTestId("supersede-commits"));
    await flush();

    expect(previewFocus).not.toHaveBeenCalled();
    expect(treeFocus).toHaveBeenCalledTimes(1);
    expect(document.activeElement).toBe(treeTab);
  });

  it("teardown makes a pending transfer inert", async () => {
    const view = render(<Harness focusOnValueChange="selected-tab" />);
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();

    fireEvent.click(screen.getByTestId("select-tree"));
    view.unmount();
    await flush();

    expect(screen.queryByRole("tablist", { name: "Inspector" })).toBeNull();
    expect(focus).not.toHaveBeenCalled();
  });

  it("async accepted Components → Tree with a ListCard descendant focuses only the Tree tab", async () => {
    render(<Harness focusOnValueChange="selected-tab" />);
    const treeTab = inspectorTab("Tree");
    const focus = vi.spyOn(treeTab, "focus");
    screen.getByTestId("list-card").focus();
    expect(document.activeElement).toBe(screen.getByTestId("list-card"));

    fireEvent.click(screen.getByTestId("accept-open"));
    await act(async () => {
      await Promise.resolve();
    });
    await flush();

    expect(screen.queryByTestId("list-card")).toBeNull();
    expect(document.activeElement).toBe(treeTab);
    expect(focus).toHaveBeenCalledTimes(1);
  });
});
