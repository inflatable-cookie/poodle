import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import ModelConnectionCard from "../src/ModelConnectionCard.svelte";

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

  it("shows readiness text and does not collapse on disable", async () => {
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

    expect(screen.getByText("Ready")).toBeTruthy();
    expect(screen.getByText("API key on file")).toBeTruthy();

    await rerender({
      id: "conn-openai-work",
      title: "OpenAI · Work",
      providerLabel: "OpenAI",
      readiness: "ready",
      readinessLabel: "Ready",
      accessSummary: "API key on file",
      isEnabled: false,
    });

    expect(screen.getByText("Ready")).toBeTruthy();
    expect(screen.getByText("API key on file")).toBeTruthy();
  });
});
