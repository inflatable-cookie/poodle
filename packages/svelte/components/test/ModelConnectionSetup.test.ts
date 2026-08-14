import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import { MODEL_CONNECTION_PICKER_FIXTURES } from "@inflatable-cookie/poodle-core";

import ModelConnectionSetup from "../src/ModelConnectionSetup.svelte";

const options = MODEL_CONNECTION_PICKER_FIXTURES;

describe("ModelConnectionSetup (svelte)", () => {
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
});
