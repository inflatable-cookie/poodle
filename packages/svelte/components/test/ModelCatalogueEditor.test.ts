import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import { MODEL_CATALOGUE_FIXTURES } from "@inflatable-cookie/poodle-core";

import ModelCatalogueEditor from "../src/ModelCatalogueEditor.svelte";

describe("ModelCatalogueEditor (svelte)", () => {
  it("renders a compact model-provider title with only optional description below", () => {
    const { container } = render(ModelCatalogueEditor, {
      props: { items: MODEL_CATALOGUE_FIXTURES },
    });

    const labels = container.querySelectorAll(".poodle-model-catalogue-editor__label");
    expect(labels[0]?.textContent?.trim()).toBe("Frontier Alpha OpenAI");
    expect(labels[0]?.querySelector(".poodle-model-catalogue-editor__provider")?.textContent?.trim()).toBe(
      "OpenAI",
    );
    const firstRow = container.querySelector('[data-model-catalogue-id="model-alpha"]');
    expect(firstRow?.querySelector(".poodle-model-catalogue-editor__label-row")?.textContent).not.toContain(
      "Default",
    );
    expect(firstRow?.querySelector(".poodle-model-catalogue-editor__utilities")?.textContent).toContain(
      "Default",
    );
    expect(container.querySelectorAll(".poodle-model-catalogue-editor__description")).toHaveLength(1);
  });

  it("emits complete shown-id order from move actions", async () => {
    const onOrderChange = vi.fn();
    render(ModelCatalogueEditor, {
      props: {
        items: MODEL_CATALOGUE_FIXTURES,
        onOrderChange,
      },
    });

    await fireEvent.click(screen.getByRole("button", { name: /Move Frontier Alpha down/i }));
    expect(onOrderChange).toHaveBeenCalledWith([
      "model-beta",
      "model-alpha",
      "model-gamma",
      "model-dup-a",
    ]);
  });

  it("hide emits only visibility intent", async () => {
    const onVisibilityChange = vi.fn();
    render(ModelCatalogueEditor, {
      props: {
        items: MODEL_CATALOGUE_FIXTURES,
        onVisibilityChange,
      },
    });

    await fireEvent.click(screen.getByRole("button", { name: /Hide Frontier Alpha/i }));
    expect(onVisibilityChange).toHaveBeenCalledWith({ id: "model-alpha", visible: false });
  });

  it("keeps catalogue postures distinct", () => {
    const { rerender } = render(ModelCatalogueEditor, {
      props: { items: [], state: "loading" },
    });
    expect(screen.getByText("Loading models")).toBeTruthy();

    rerender({ items: [], state: "sessionNegotiated" });
    expect(screen.getByText("Models after session")).toBeTruthy();

    rerender({ items: [], state: "empty" });
    expect(screen.getByText("No models")).toBeTruthy();
  });

  it("tracks a keyboard grab by stable id and limits pointer drag to the handle", async () => {
    const { container } = render(ModelCatalogueEditor, {
      props: { items: MODEL_CATALOGUE_FIXTURES, onOrderChange: vi.fn() },
    });
    const handle = screen.getByRole("button", {
      name: /Frontier Alpha, position 1 of 4/i,
    }) as HTMLButtonElement;
    const row = container.querySelector(
      '[data-model-catalogue-id="model-alpha"]',
    ) as HTMLElement;

    // g16.028: pointer drag moved onto the common substrate. The row owns no
    // drag attribute at all, and the handle carries only the substrate's own
    // disarmed `draggable="false"` — the controller arms a native drag on a
    // bridged source and on nothing else. The keyboard grab stays the
    // component's own, because each arrow press is a committed move.
    expect(row.getAttribute("draggable")).toBeNull();
    expect(handle.getAttribute("draggable")).toBe("false");
    await fireEvent.keyDown(handle, { key: " " });
    await fireEvent.keyDown(handle, { key: "ArrowDown" });
    expect(row.getAttribute("data-grabbed")).toBe("true");
    expect(handle.getAttribute("aria-pressed")).toBe("true");
  });
});
