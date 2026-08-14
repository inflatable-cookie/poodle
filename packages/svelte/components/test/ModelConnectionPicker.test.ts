import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import {
  MODEL_CONNECTION_PICKER_FIXTURES,
  type ModelConnectionOption,
} from "@inflatable-cookie/poodle-core";

import ModelConnectionPicker from "../src/ModelConnectionPicker.svelte";

const options = MODEL_CONNECTION_PICKER_FIXTURES;

describe("ModelConnectionPicker (svelte)", () => {
  it("filters case-folded and keeps source order", async () => {
    const onQueryChange = vi.fn();
    render(ModelConnectionPicker, {
      props: { options, onQueryChange },
    });

    const search = screen.getByRole("searchbox");
    await fireEvent.input(search, { target: { value: "LOCAL" } });
    expect(onQueryChange).toHaveBeenCalledWith("LOCAL");

    const radios = screen.getAllByRole("radio");
    expect(radios.map((node) => node.getAttribute("data-model-connection-option"))).toEqual([
      "codex-app",
      "ollama-local",
      "lmstudio-local",
    ]);
  });

  it("selects only exact available enabled ids", async () => {
    const onValueChange = vi.fn();
    const { container } = render(ModelConnectionPicker, {
      props: { options, onValueChange },
    });

    const legacy = container.querySelector(
      '[data-model-connection-option="vendor-legacy"]',
    ) as HTMLButtonElement;
    await fireEvent.click(legacy);
    expect(onValueChange).not.toHaveBeenCalled();

    const responses = container.querySelector(
      '[data-model-connection-option="openai-responses"]',
    ) as HTMLButtonElement;
    await fireEvent.click(responses);
    expect(onValueChange).toHaveBeenCalledWith("openai-responses");
  });

  it("shows distinct loading and empty postures", async () => {
    const { rerender } = render(ModelConnectionPicker, {
      props: { options, state: "loading" },
    });
    expect(screen.getByText("Loading connections")).toBeTruthy();

    await rerender({ options: [] as ModelConnectionOption[], state: "ready" });
    expect(screen.getByText("No connections available")).toBeTruthy();
  });

  it("keeps a visible option tabbable when the selected option is filtered out", () => {
    const { container } = render(ModelConnectionPicker, {
      props: {
        options,
        value: "openai-responses",
        query: "local",
      },
    });

    const ollama = container.querySelector(
      '[data-model-connection-option="ollama-local"]',
    ) as HTMLButtonElement;
    expect(ollama.tabIndex).toBe(0);
  });

  it("renders the selected-route indicator", () => {
    const { container } = render(ModelConnectionPicker, {
      props: { options, defaultValue: "openai-responses" },
    });

    expect(
      container.querySelector(
        '[data-model-connection-option="openai-responses"] .poodle-model-connection-picker__selected-icon',
      ),
    ).toBeTruthy();
  });
});
