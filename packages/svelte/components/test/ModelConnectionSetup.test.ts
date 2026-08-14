import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import {
  MODEL_CONNECTION_PICKER_FIXTURES,
  type ModelConnectionOption,
} from "@inflatable-cookie/poodle-core";

import ModelConnectionSetup from "../src/ModelConnectionSetup.svelte";

const options = MODEL_CONNECTION_PICKER_FIXTURES;
const directOptions: ModelConnectionOption[] = options.map((option) =>
  option.id === "codex-app"
    ? { ...option, availability: "available", availabilityLabel: "Available", isDisabled: false }
    : option,
);

describe("ModelConnectionSetup (svelte)", () => {
  it("submits a direct route without entering configure", async () => {
    const onStageChange = vi.fn();
    const onSubmit = vi.fn();
    render(ModelConnectionSetup, {
      props: {
        options: directOptions,
        defaultValue: "codex-app",
        canSubmit: true,
        onStageChange,
        onSubmit,
      },
    });

    expect(screen.queryByRole("button", { name: "Continue" })).toBeNull();
    await fireEvent.click(screen.getByRole("button", { name: "Add connection" }));
    expect(onSubmit).toHaveBeenCalledWith("codex-app");
    expect(onStageChange).not.toHaveBeenCalled();
  });

  it("blocks continue without a selectable connection", async () => {
    const onStageChange = vi.fn();
    render(ModelConnectionSetup, {
      props: { options, onStageChange },
    });

    const continueButton = screen.getByRole("button", { name: "Continue" });
    expect(continueButton.hasAttribute("disabled")).toBe(true);
    await fireEvent.click(continueButton);
    expect(onStageChange).not.toHaveBeenCalled();
  });

  it("continues then submits only with host canSubmit", async () => {
    const onStageChange = vi.fn();
    const onSubmit = vi.fn();
    const { rerender } = render(ModelConnectionSetup, {
      props: {
        options,
        defaultValue: "openai-responses",
        onStageChange,
        onSubmit,
      },
    });

    await fireEvent.click(screen.getByRole("button", { name: "Continue" }));
    expect(onStageChange).toHaveBeenCalledWith("configure");

    await rerender({
      options,
      stage: "configure",
      value: "openai-responses",
      canSubmit: false,
      onStageChange,
      onSubmit,
    });
    expect(screen.getByRole("button", { name: "Add connection" }).hasAttribute("disabled")).toBe(
      true,
    );

    await rerender({
      options,
      stage: "configure",
      value: "openai-responses",
      canSubmit: true,
      onStageChange,
      onSubmit,
    });
    await fireEvent.click(screen.getByRole("button", { name: "Add connection" }));
    expect(onSubmit).toHaveBeenCalledWith("openai-responses");
  });

  it("locks workflow actions while pending", async () => {
    const onCancel = vi.fn();
    render(ModelConnectionSetup, {
      props: {
        options,
        stage: "configure",
        value: "openai-responses",
        canSubmit: true,
        isPending: true,
        onCancel,
      },
    });

    expect(screen.getByRole("button", { name: "Back" }).hasAttribute("disabled")).toBe(true);
    expect(screen.getByRole("button", { name: "Cancel" }).hasAttribute("disabled")).toBe(true);
    expect(screen.getByRole("button", { name: "Add connection" }).hasAttribute("disabled")).toBe(
      true,
    );
    await fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
    expect(onCancel).not.toHaveBeenCalled();
  });

  it("does not render an empty configuration surface", () => {
    const { container } = render(ModelConnectionSetup, {
      props: {
        options,
        stage: "configure",
        value: "openai-responses",
      },
    });

    expect(container.querySelector(".poodle-model-connection-setup__configuration")).toBeNull();
  });

  it("restores focus to the selected route after a controlled Back transition", async () => {
    const { container, rerender } = render(ModelConnectionSetup, {
      props: {
        options,
        stage: "configure",
        value: "openai-responses",
      },
    });

    await rerender({
      options,
      stage: "choose",
      value: "openai-responses",
    });

    const selected = container.querySelector(
      '[data-model-connection-option="openai-responses"]',
    );
    await waitFor(() => expect(document.activeElement).toBe(selected));
  });
});
