import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import {
  MODEL_CATALOGUE_FIXTURES,
  MODEL_CONNECTION_PICKER_FIXTURES,
  type ModelConnectionOption,
} from "@inflatable-cookie/poodle-core";

import { ModelCatalogueEditor } from "../src/ModelCatalogueEditor";
import { ModelConnectionCard } from "../src/ModelConnectionCard";
import { ModelConnectionPicker } from "../src/ModelConnectionPicker";
import { ModelConnectionSetup } from "../src/ModelConnectionSetup";

const options = MODEL_CONNECTION_PICKER_FIXTURES;

describe("ModelConnectionPicker (react)", () => {
  it("filters case-folded and keeps source order when query is controlled", () => {
    const { container } = render(
      <ModelConnectionPicker options={options} query="LOCAL" />,
    );
    const ids = [...container.querySelectorAll("[data-model-connection-option]")].map((node) =>
      node.getAttribute("data-model-connection-option"),
    );
    expect(ids).toEqual(["codex-app", "ollama-local", "lmstudio-local"]);
  });

  it("emits query changes from search", () => {
    const onQueryChange = vi.fn();
    render(<ModelConnectionPicker options={options} onQueryChange={onQueryChange} />);
    fireEvent.change(screen.getByRole("searchbox"), { target: { value: "LOCAL" } });
    expect(onQueryChange).toHaveBeenCalledWith("LOCAL");
  });

  it("selects only exact available enabled ids", () => {
    const onValueChange = vi.fn();
    const { container } = render(
      <ModelConnectionPicker options={options} onValueChange={onValueChange} />,
    );

    fireEvent.click(
      container.querySelector('[data-model-connection-option="vendor-legacy"]') as HTMLElement,
    );
    expect(onValueChange).not.toHaveBeenCalled();

    fireEvent.click(
      container.querySelector('[data-model-connection-option="openai-responses"]') as HTMLElement,
    );
    expect(onValueChange).toHaveBeenCalledWith("openai-responses");
  });

  it("shows distinct loading and empty postures", () => {
    const { rerender } = render(
      <ModelConnectionPicker options={options} state="loading" />,
    );
    expect(document.querySelector('[data-state="loading"]')).toBeTruthy();

    rerender(<ModelConnectionPicker options={[] as ModelConnectionOption[]} state="ready" />);
    expect(document.querySelector('[data-state="empty"]')).toBeTruthy();
  });
});

describe("ModelConnectionSetup (react)", () => {
  it("continues then submits only with host canSubmit", () => {
    const onStageChange = vi.fn();
    const onSubmit = vi.fn();
    const { rerender } = render(
      <ModelConnectionSetup
        options={options}
        defaultValue="openai-responses"
        onStageChange={onStageChange}
        onSubmit={onSubmit}
      />,
    );

    fireEvent.click(screen.getByRole("button", { name: "Continue" }));
    expect(onStageChange).toHaveBeenCalledWith("configure");

    rerender(
      <ModelConnectionSetup
        options={options}
        stage="configure"
        value="openai-responses"
        canSubmit={false}
        onStageChange={onStageChange}
        onSubmit={onSubmit}
      />,
    );
    expect(screen.getByRole("button", { name: "Add connection" }).hasAttribute("disabled")).toBe(
      true,
    );

    rerender(
      <ModelConnectionSetup
        options={options}
        stage="configure"
        value="openai-responses"
        canSubmit
        onStageChange={onStageChange}
        onSubmit={onSubmit}
      />,
    );
    fireEvent.click(screen.getByRole("button", { name: "Add connection" }));
    expect(onSubmit).toHaveBeenCalledWith("openai-responses");
  });

  it("locks workflow actions while pending", () => {
    const onCancel = vi.fn();
    render(
      <ModelConnectionSetup
        options={options}
        stage="configure"
        value="openai-responses"
        canSubmit
        isPending
        onCancel={onCancel}
      />,
    );

    expect(screen.getByRole("button", { name: "Cancel" }).hasAttribute("disabled")).toBe(true);
    fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
    expect(onCancel).not.toHaveBeenCalled();
  });
});

describe("ModelConnectionCard (react)", () => {
  it("keeps disclosure and enable callbacks independent", () => {
    const onOpenChange = vi.fn();
    const onEnabledChange = vi.fn();
    render(
      <ModelConnectionCard
        id="conn-openai-work"
        title="OpenAI · Work"
        providerLabel="OpenAI"
        readiness="ready"
        readinessLabel="Ready"
        isEnabled
        onOpenChange={onOpenChange}
        onEnabledChange={onEnabledChange}
      />,
    );

    fireEvent.click(screen.getByRole("button", { name: /Expand OpenAI/i }));
    expect(onOpenChange).toHaveBeenCalledWith(true);
    expect(onEnabledChange).not.toHaveBeenCalled();

    fireEvent.click(screen.getByRole("switch", { name: /Enable OpenAI/i }));
    expect(onEnabledChange).toHaveBeenCalledWith(false);
  });
});

describe("ModelCatalogueEditor (react)", () => {
  it("emits complete shown-id order from move actions", () => {
    const onOrderChange = vi.fn();
    render(
      <ModelCatalogueEditor items={MODEL_CATALOGUE_FIXTURES} onOrderChange={onOrderChange} />,
    );

    fireEvent.click(screen.getByRole("button", { name: /Move Frontier Alpha down/i }));
    expect(onOrderChange).toHaveBeenCalledWith([
      "model-beta",
      "model-alpha",
      "model-gamma",
      "model-dup-a",
    ]);
  });

  it("hide emits only visibility intent", () => {
    const onVisibilityChange = vi.fn();
    render(
      <ModelCatalogueEditor
        items={MODEL_CATALOGUE_FIXTURES}
        onVisibilityChange={onVisibilityChange}
      />,
    );

    fireEvent.click(screen.getByRole("button", { name: /Hide Frontier Alpha/i }));
    expect(onVisibilityChange).toHaveBeenCalledWith({ id: "model-alpha", visible: false });
  });

  it("keeps catalogue postures distinct", () => {
    const { rerender } = render(<ModelCatalogueEditor items={[]} state="loading" />);
    expect(screen.getByText("Loading models")).toBeTruthy();

    rerender(<ModelCatalogueEditor items={[]} state="sessionNegotiated" />);
    expect(screen.getByText("Models after session")).toBeTruthy();
  });
});
