import { fireEvent, render, waitFor } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import EmbedInput from "../src/EmbedInput.svelte";
import { resolveEmbedParseState } from "../src/embed-input";

describe("EmbedInput (svelte)", () => {
  it("reports value changes immediately, before the debounce", async () => {
    const onValueChange = vi.fn();
    const { container } = render(EmbedInput, {
      props: { value: "", onValueChange, parseDebounce: 500 },
    });
    const input = container.querySelector("textarea") as HTMLTextAreaElement;
    await fireEvent.input(input, { target: { value: "https://vimeo.com/76979871" } });
    expect(onValueChange).toHaveBeenCalledWith("https://vimeo.com/76979871");
  });

  it("surfaces the parsed provider pill and success text after debounced parsing", async () => {
    const onParse = vi.fn();
    const { container } = render(EmbedInput, { props: { onParse, parseDebounce: 1 } });
    const input = container.querySelector("textarea") as HTMLTextAreaElement;
    await fireEvent.input(input, { target: { value: "https://youtu.be/dQw4w9WgXcQ" } });

    await waitFor(() => {
      expect(onParse).toHaveBeenCalled();
    });
    const state = onParse.mock.calls[0][0] as { provider: string; id: string };
    expect(state.provider).toBe("youtube");
    expect(state.id).toBe("dQw4w9WgXcQ");
    expect(onParse.mock.calls[0][1]).toBeNull();

    expect(container.querySelector(".poodle-embed-input__success")?.textContent).toContain(
      "Embed detected",
    );
  });

  it("shows the parse error for unrecognised text", async () => {
    const onParse = vi.fn();
    const { container } = render(EmbedInput, { props: { onParse, parseDebounce: 1 } });
    const input = container.querySelector("textarea") as HTMLTextAreaElement;
    await fireEvent.input(input, { target: { value: "not an embed source" } });

    await waitFor(() => {
      expect(onParse).toHaveBeenCalled();
    });
    expect(onParse.mock.calls[0][0]).toBeNull();
    expect(onParse.mock.calls[0][1]).toBe("Could not parse embed source");
    expect(container.querySelector(".poodle-embed-input__error")?.textContent).toContain(
      "Could not parse embed source",
    );
  });

  it("enforces the provider restriction list", async () => {
    const onParse = vi.fn();
    const { container } = render(EmbedInput, {
      props: { onParse, parseDebounce: 1, providers: ["youtube", "vimeo"] },
    });
    const input = container.querySelector("textarea") as HTMLTextAreaElement;
    await fireEvent.input(input, { target: { value: "https://example.com/asset" } });

    await waitFor(() => {
      expect(onParse).toHaveBeenCalled();
    });
    expect(onParse.mock.calls[0][0]).toBeNull();
    expect(onParse.mock.calls[0][1]).toBe('Provider "generic" is not allowed');
    expect(container.querySelector(".poodle-embed-input__error")?.textContent).toContain(
      "not allowed",
    );
  });

  it("renders an externally supplied error message", () => {
    const { container } = render(EmbedInput, { props: { error: "Embed host unreachable" } });
    expect(container.querySelector(".poodle-embed-input__error")?.textContent).toBe(
      "Embed host unreachable",
    );
  });

  it("routes parsing through a custom resolveParseState when supplied", async () => {
    const custom = vi.fn((value: string) =>
      resolveEmbedParseState(`https://youtu.be/${value}`, []),
    );
    const onParse = vi.fn();
    const { container } = render(EmbedInput, {
      props: { onParse, parseDebounce: 1, resolveParseState: custom },
    });
    const input = container.querySelector("textarea") as HTMLTextAreaElement;
    await fireEvent.input(input, { target: { value: "abc123" } });

    await waitFor(() => {
      expect(onParse).toHaveBeenCalled();
    });
    expect(custom).toHaveBeenCalled();
    expect(onParse.mock.calls[0][0]?.provider).toBe("youtube");
  });
});