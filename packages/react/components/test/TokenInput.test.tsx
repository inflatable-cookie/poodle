import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { TokenInput } from "../src/TokenInput";

describe("TokenInput (react)", () => {
  const controlOf = (container: HTMLElement) =>
    container.querySelector(".poodle-token-input__control") as HTMLInputElement;

  it("commits the draft token on Enter and clears the draft", () => {
    const onValuesChange = vi.fn();
    const { container } = render(<TokenInput onValuesChange={onValuesChange} />);
    const input = controlOf(container);

    fireEvent.change(input, { target: { value: "alpha" } });
    fireEvent.keyDown(input, { key: "Enter" });

    expect(onValuesChange).toHaveBeenCalledWith(["alpha"]);
    expect(input.value).toBe("");
    expect(container.querySelector(".poodle-token-input__token-label")?.textContent).toBe("alpha");
  });

  it("splits tokens on the separator and keeps the trailing draft", () => {
    const onValuesChange = vi.fn();
    const { container } = render(<TokenInput onValuesChange={onValuesChange} />);
    const input = controlOf(container);

    fireEvent.change(input, { target: { value: "alpha,beta, gamma" } });

    // Tokens before the last separator commit; the trailing fragment stays
    // as the live draft until the next separator, Enter, Tab, or blur.
    expect(onValuesChange).toHaveBeenCalledWith(["alpha", "beta"]);
    expect(input.value).toBe(" gamma");
  });

  it("dedupes committed tokens by default and allows repeats when disabled", () => {
    const deduped = vi.fn();
    const { container } = render(<TokenInput values={["alpha"]} onValuesChange={deduped} />);
    const input = controlOf(container);

    fireEvent.change(input, { target: { value: "alpha," } });
    // The duplicate is ignored: the committed list keeps the first occurrence.
    expect(deduped).toHaveBeenCalledWith(["alpha"]);

    const repeated = vi.fn();
    const loose = render(
      <TokenInput values={["alpha"]} dedupe={false} onValuesChange={repeated} />,
    );
    const looseInput = controlOf(loose.container);
    fireEvent.change(looseInput, { target: { value: "alpha," } });
    expect(repeated).toHaveBeenCalledWith(["alpha", "alpha"]);
  });

  it("removes a token via its labelled remove button", () => {
    const onValuesChange = vi.fn();
    const { container } = render(
      <TokenInput values={["alpha", "beta"]} onValuesChange={onValuesChange} />,
    );
    const removeBeta = container.querySelector('[aria-label="Remove beta"]') as HTMLButtonElement;
    expect(removeBeta).not.toBeNull();

    fireEvent.click(removeBeta);
    expect(onValuesChange).toHaveBeenCalledWith(["alpha"]);
  });

  it("removes the last token on Backspace with an empty draft", () => {
    const onValuesChange = vi.fn();
    const { container } = render(
      <TokenInput values={["alpha", "beta"]} onValuesChange={onValuesChange} />,
    );
    const input = controlOf(container);

    fireEvent.keyDown(input, { key: "Backspace" });
    expect(onValuesChange).toHaveBeenCalledWith(["alpha"]);
  });

  it("emits one hidden input per token when name is set", () => {
    const { container } = render(<TokenInput name="tags" values={["alpha", "beta"]} />);
    const hidden = Array.from(container.querySelectorAll('input[type="hidden"]'));
    expect(hidden.map((input) => input.getAttribute("value"))).toEqual(["alpha", "beta"]);
    expect(hidden.every((input) => input.getAttribute("name") === "tags")).toBe(true);
  });

  it("hides remove buttons and disables entry when readOnly or disabled", () => {
    const readOnly = render(<TokenInput values={["alpha"]} readOnly />);
    expect(readOnly.container.querySelector(".poodle-token-input__remove")).toBeNull();
    expect(
      (readOnly.container.querySelector(".poodle-token-input__control") as HTMLInputElement).readOnly,
    ).toBe(true);

    const disabled = render(<TokenInput values={["alpha"]} disabled />);
    expect(disabled.container.querySelector(".poodle-token-input__remove")).toBeNull();
    expect(
      (disabled.container.querySelector(".poodle-token-input__control") as HTMLInputElement).disabled,
    ).toBe(true);
  });

  it("rejects drafts through resolveToken and reports the rejection", () => {
    const onValuesChange = vi.fn();
    const onTokenReject = vi.fn();
    const { container } = render(
      <TokenInput
        resolveToken={(value: string) => (value.startsWith("bad-") ? null : value)}
        onValuesChange={onValuesChange}
        onTokenReject={onTokenReject}
      />,
    );
    const input = controlOf(container);

    fireEvent.change(input, { target: { value: "bad-word" } });
    fireEvent.keyDown(input, { key: "Enter" });

    expect(onValuesChange).not.toHaveBeenCalled();
    expect(onTokenReject).toHaveBeenCalledWith("bad-word");
  });

  it("forwards aria-label and describedBy to the live input", () => {
    const { container } = render(<TokenInput ariaLabel="Tags" describedBy="hint-1" />);
    const input = controlOf(container);
    expect(input.getAttribute("aria-label")).toBe("Tags");
    expect(input.getAttribute("aria-describedby")).toBe("hint-1");
  });
});