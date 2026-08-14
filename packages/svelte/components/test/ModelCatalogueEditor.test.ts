import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import { MODEL_CATALOGUE_FIXTURES } from "@inflatable-cookie/poodle-core";

import ModelCatalogueEditor from "../src/ModelCatalogueEditor.svelte";

describe("ModelCatalogueEditor (svelte)", () => {
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
});
