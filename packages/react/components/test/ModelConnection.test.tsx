import { fireEvent, render, screen, waitFor } from "@testing-library/react";
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
const directOptions: ModelConnectionOption[] = options.map((option) =>
  option.id === "codex-app"
    ? { ...option, availability: "available", availabilityLabel: "Available", isDisabled: false }
    : option,
);

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
    expect(screen.getByText("Loading connections")).toBeTruthy();

    rerender(<ModelConnectionPicker options={[] as ModelConnectionOption[]} state="ready" />);
    expect(screen.getByText("No connections available")).toBeTruthy();
  });

  it("keeps a visible option tabbable when the selected option is filtered out", () => {
    const { container } = render(
      <ModelConnectionPicker
        options={options}
        value="openai-responses"
        query="local"
      />,
    );

    const ollama = container.querySelector(
      '[data-model-connection-option="ollama-local"]',
    ) as HTMLButtonElement;
    expect(ollama.tabIndex).toBe(0);
  });

  it("renders the selected-route indicator", () => {
    const { container } = render(
      <ModelConnectionPicker options={options} defaultValue="openai-responses" />,
    );
    const selected = container.querySelector(
      '[data-model-connection-option="openai-responses"]',
    ) as HTMLElement;

    expect(
      selected.querySelector(
        ".poodle-model-connection-picker__leading > .poodle-model-connection-picker__selected-icon",
      ),
    ).toBeTruthy();
    expect(
      selected.querySelector(
        ".poodle-model-connection-picker__availability .poodle-model-connection-picker__selected-icon",
      ),
    ).toBeNull();
  });

  it("renders compact option copy without group badges or visible descriptions", () => {
    const { container } = render(<ModelConnectionPicker options={options} />);

    expect(container.querySelector(".poodle-model-connection-picker__badges")).toBeNull();
    expect(container.querySelector(".poodle-model-connection-picker__description")).toBeNull();
    expect(
      container.querySelector(
        '[data-model-connection-option="codex-app"] .poodle-status-indicator__label',
      )?.textContent,
    ).toBe("Checking");
    expect(
      container.querySelector(
        '[data-model-connection-option="vendor-legacy"] .poodle-status-indicator__label',
      )?.textContent,
    ).toBe("Unsupported");
  });
});

describe("ModelConnectionSetup (react)", () => {
  it("submits a direct route without entering configure", () => {
    const onStageChange = vi.fn();
    const onSubmit = vi.fn();
    render(
      <ModelConnectionSetup
        options={directOptions}
        defaultValue="codex-app"
        canSubmit
        onStageChange={onStageChange}
        onSubmit={onSubmit}
      />,
    );

    expect(screen.queryByRole("button", { name: "Continue" })).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "Add connection" }));
    expect(onSubmit).toHaveBeenCalledWith("codex-app");
    expect(onStageChange).not.toHaveBeenCalled();
  });

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

  it("does not render an empty configuration surface", () => {
    const { container } = render(
      <ModelConnectionSetup
        options={options}
        stage="configure"
        value="openai-responses"
      />,
    );

    expect(container.querySelector(".poodle-model-connection-setup__configuration")).toBeNull();
  });

  it("restores focus to the selected route after a controlled Back transition", async () => {
    const { container, rerender } = render(
      <ModelConnectionSetup
        options={options}
        stage="configure"
        value="openai-responses"
      />,
    );

    rerender(
      <ModelConnectionSetup
        options={options}
        stage="choose"
        value="openai-responses"
      />,
    );

    const selected = container.querySelector(
      '[data-model-connection-option="openai-responses"]',
    );
    await waitFor(() => expect(document.activeElement).toBe(selected));
  });
});

describe("ModelConnectionCard (react)", () => {
  it("uses access summary when ready and active readiness while checking", () => {
    const { container, rerender } = render(
      <ModelConnectionCard
        id="conn-openai-work"
        title="OpenAI · Work"
        providerLabel="OpenAI"
        accessSummary="API key on file"
        readiness="ready"
        readinessLabel="Ready"
      />,
    );

    const controls = container.querySelector(".poodle-model-connection-card__controls");
    expect(
      container
        .querySelector(".poodle-model-connection-card__title-row")
        ?.firstElementChild?.classList.contains("poodle-model-connection-card__leading"),
    ).toBe(true);
    expect(controls?.querySelector(".poodle-status-indicator")?.textContent).toContain(
      "API key on file",
    );
    expect(screen.queryByText("Ready")).toBeNull();

    rerender(
      <ModelConnectionCard
        id="conn-openai-work"
        title="OpenAI · Work"
        providerLabel="OpenAI"
        accessSummary="Signed in"
        readiness="checking"
        readinessLabel="Checking install"
      />,
    );

    expect(controls?.querySelector(".poodle-status-indicator")?.textContent).toContain(
      "Checking install",
    );
    expect(screen.queryByText("Signed in")).toBeNull();
  });

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

  it("uses instance-scoped detail ids for repeated connection records", () => {
    render(
      <>
        <ModelConnectionCard id="same" title="First" providerLabel="Provider" />
        <ModelConnectionCard id="same" title="Second" providerLabel="Provider" />
      </>,
    );

    const controls = screen
      .getAllByRole("button", { name: /Expand/ })
      .map((button) => button.getAttribute("aria-controls"));
    expect(new Set(controls).size).toBe(2);
  });

  it("restores focus when a controlled card closes around focused details", async () => {
    const details = () => <button type="button">Detail action</button>;
    const { rerender } = render(
      <ModelConnectionCard
        id="controlled"
        title="Controlled"
        providerLabel="Provider"
        open
        details={details}
      />,
    );
    screen.getByRole("button", { name: "Detail action" }).focus();

    rerender(
      <ModelConnectionCard
        id="controlled"
        title="Controlled"
        providerLabel="Provider"
        open={false}
        details={details}
      />,
    );

    const disclosure = screen.getByRole("button", { name: "Expand Controlled" });
    await waitFor(() => expect(document.activeElement).toBe(disclosure));
  });
});

describe("ModelCatalogueEditor (react)", () => {
  it("renders a compact model-provider title with only optional description below", () => {
    const { container } = render(
      <ModelCatalogueEditor items={MODEL_CATALOGUE_FIXTURES} />,
    );

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

  it("tracks a keyboard grab by stable id and limits pointer drag to the handle", () => {
    const { container } = render(
      <ModelCatalogueEditor
        items={MODEL_CATALOGUE_FIXTURES}
        onOrderChange={vi.fn()}
      />,
    );
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
    fireEvent.keyDown(handle, { key: " " });
    fireEvent.keyDown(handle, { key: "ArrowDown" });
    expect(row.getAttribute("data-grabbed")).toBe("true");
    expect(handle.getAttribute("aria-pressed")).toBe("true");
  });
});
