import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { createRawSnippet } from "svelte";
import { describe, expect, it, vi } from "vitest";

import ModelConnectionCard from "../src/ModelConnectionCard.svelte";

const detailsSnippet = createRawSnippet(() => ({
  render: () => '<button type="button">Detail action</button>',
}));

describe("ModelConnectionCard (svelte)", () => {
  it("keeps disclosure and enable callbacks independent", async () => {
    const onOpenChange = vi.fn();
    const onEnabledChange = vi.fn();
    render(ModelConnectionCard, {
      props: {
        id: "conn-openai-work",
        title: "OpenAI · Work",
        providerLabel: "OpenAI",
        routeLabel: "Responses API",
        readiness: "ready",
        readinessLabel: "Ready",
        isEnabled: true,
        onOpenChange,
        onEnabledChange,
      },
    });

    await fireEvent.click(screen.getByRole("button", { name: /Expand OpenAI/i }));
    expect(onOpenChange).toHaveBeenCalledWith(true);
    expect(onEnabledChange).not.toHaveBeenCalled();

    await fireEvent.click(screen.getByRole("switch", { name: /Enable OpenAI/i }));
    expect(onEnabledChange).toHaveBeenCalledWith(false);
    expect(onOpenChange).toHaveBeenCalledTimes(1);
  });

  it("uses access summary when ready and active readiness while checking", async () => {
    const { rerender } = render(ModelConnectionCard, {
      props: {
        id: "conn-openai-work",
        title: "OpenAI · Work",
        providerLabel: "OpenAI",
        readiness: "ready",
        readinessLabel: "Ready",
        accessSummary: "API key on file",
        isEnabled: true,
      },
    });

    expect(screen.getByText("API key on file")).toBeTruthy();
    expect(screen.queryByText("Ready")).toBeNull();
    expect(
      document
        .querySelector(".poodle-model-connection-card__title-row")
        ?.firstElementChild?.classList.contains("poodle-model-connection-card__leading"),
    ).toBe(true);

    await rerender({
      id: "conn-openai-work",
      title: "OpenAI · Work",
      providerLabel: "OpenAI",
      readiness: "checking",
      readinessLabel: "Checking install",
      accessSummary: "Signed in",
      isEnabled: true,
    });

    expect(screen.getByText("Checking install")).toBeTruthy();
    expect(screen.queryByText("Signed in")).toBeNull();
    expect(
      document.querySelector(".poodle-model-connection-card__controls .poodle-status-indicator"),
    ).toBeTruthy();
  });

  it("uses instance-scoped detail ids for repeated connection records", () => {
    render(ModelConnectionCard, {
      props: { id: "same", title: "First", providerLabel: "Provider" },
    });
    render(ModelConnectionCard, {
      props: { id: "same", title: "Second", providerLabel: "Provider" },
    });

    const controls = screen
      .getAllByRole("button", { name: /Expand/ })
      .map((button) => button.getAttribute("aria-controls"));
    expect(new Set(controls).size).toBe(2);
  });

  it("restores focus when a controlled card closes around focused details", async () => {
    const props = {
      id: "controlled",
      title: "Controlled",
      providerLabel: "Provider",
      open: true,
      details: detailsSnippet as never,
    };
    const { rerender } = render(ModelConnectionCard, { props });
    const detailAction = screen.getByRole("button", { name: "Detail action" });
    detailAction.focus();

    await rerender({ ...props, open: false });

    const disclosure = screen.getByRole("button", { name: "Expand Controlled" });
    await waitFor(() => expect(document.activeElement).toBe(disclosure));
  });
});
