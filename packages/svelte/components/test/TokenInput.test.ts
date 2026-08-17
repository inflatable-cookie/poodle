import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import TokenInput from "../src/TokenInput.svelte";

describe("TokenInput (svelte)", () => {
  const controlOf = (container: HTMLElement) =>
    container.querySelector(".poodle-token-input__control") as HTMLInputElement;

  it("commits the draft token on Enter and clears the draft", async () => {
    const onValuesChange = vi.fn();
    const { container } = render(TokenInput, { props: { onValuesChange } });
    const input = controlOf(container);

    await fireEvent.input(input, { target: { value: "alpha" } });
    await fireEvent.keyDown(input, { key: "Enter" });

    expect(onValuesChange).toHaveBeenCalledWith(["alpha"]);
    expect(input.value).toBe("");
    expect(container.querySelector(".poodle-token-input__token-label")?.textContent).toBe("alpha");
  });

  it("splits tokens on the separator and keeps the trailing draft", async () => {
    const onValuesChange = vi.fn();
    const { container } = render(TokenInput, { props: { onValuesChange } });
    const input = controlOf(container);

    await fireEvent.input(input, { target: { value: "alpha,beta, gamma" } });

    // Tokens before the last separator commit; the trailing fragment stays
    // as the live draft until the next separator, Enter, Tab, or blur.
    expect(onValuesChange).toHaveBeenCalledWith(["alpha", "beta"]);
    expect(input.value).toBe(" gamma");
  });

  it("dedupes committed tokens by default and allows repeats when disabled", async () => {
    const deduped = vi.fn();
    const { container } = render(TokenInput, { props: { values: ["alpha"], onValuesChange: deduped } });
    const input = controlOf(container);

    await fireEvent.input(input, { target: { value: "alpha," } });
    // The duplicate is ignored: the committed list keeps the first occurrence.
    expect(deduped).toHaveBeenCalledWith(["alpha"]);

    const repeated = vi.fn();
    const loose = render(TokenInput, {
      props: { values: ["alpha"], dedupe: false, onValuesChange: repeated },
    });
    const looseInput = controlOf(loose.container);
    await fireEvent.input(looseInput, { target: { value: "alpha," } });
    expect(repeated).toHaveBeenCalledWith(["alpha", "alpha"]);
  });

  it("removes a token via its labelled remove button", async () => {
    const onValuesChange = vi.fn();
    const { container } = render(TokenInput, {
      props: { values: ["alpha", "beta"], onValuesChange },
    });
    const removeBeta = container.querySelector('[aria-label="Remove beta"]') as HTMLButtonElement;
    expect(removeBeta).not.toBeNull();

    await fireEvent.click(removeBeta);
    expect(onValuesChange).toHaveBeenCalledWith(["alpha"]);
  });

  it("removes the last token on Backspace with an empty draft", async () => {
    const onValuesChange = vi.fn();
    const { container } = render(TokenInput, {
      props: { values: ["alpha", "beta"], onValuesChange },
    });
    const input = controlOf(container);

    await fireEvent.keyDown(input, { key: "Backspace" });
    expect(onValuesChange).toHaveBeenCalledWith(["alpha"]);
  });

  it("emits one hidden input per token when name is set", () => {
    const { container } = render(TokenInput, {
      props: { name: "tags", values: ["alpha", "beta"] },
    });
    const hidden = Array.from(container.querySelectorAll('input[type="hidden"]'));
    expect(hidden.map((input) => input.getAttribute("value"))).toEqual(["alpha", "beta"]);
    expect(hidden.every((input) => input.getAttribute("name") === "tags")).toBe(true);
  });

  it("hides remove buttons and disables entry when readOnly or disabled", () => {
    const readOnly = render(TokenInput, { props: { values: ["alpha"], readOnly: true } });
    expect(readOnly.container.querySelector(".poodle-token-input__remove")).toBeNull();
    expect((readOnly.container.querySelector(".poodle-token-input__control") as HTMLInputElement).readOnly).toBe(true);

    const disabled = render(TokenInput, { props: { values: ["alpha"], disabled: true } });
    expect(disabled.container.querySelector(".poodle-token-input__remove")).toBeNull();
    expect((disabled.container.querySelector(".poodle-token-input__control") as HTMLInputElement).disabled).toBe(true);
  });

  it("rejects drafts through resolveToken and reports the rejection", async () => {
    const onValuesChange = vi.fn();
    const onTokenReject = vi.fn();
    const { container } = render(TokenInput, {
      props: {
        resolveToken: (value: string) => (value.startsWith("bad-") ? null : value),
        onValuesChange,
        onTokenReject,
      },
    });
    const input = controlOf(container);

    await fireEvent.input(input, { target: { value: "bad-word" } });
    await fireEvent.keyDown(input, { key: "Enter" });

    expect(onValuesChange).not.toHaveBeenCalled();
    expect(onTokenReject).toHaveBeenCalledWith("bad-word");
  });

  it("forwards aria-label and describedBy to the live input", () => {
    const { container } = render(TokenInput, {
      props: { ariaLabel: "Tags", describedBy: "hint-1" },
    });
    const input = controlOf(container);
    expect(input.getAttribute("aria-label")).toBe("Tags");
    expect(input.getAttribute("aria-describedby")).toBe("hint-1");
  });
});