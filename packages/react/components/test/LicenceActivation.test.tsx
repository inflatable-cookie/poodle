import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { LicenceActivation, type LicenceActivationProps } from "../src/LicenceActivation";
import type { LicenceKeyFormat, LicenceKeyProblem } from "@inflatable-cookie/poodle-core";

const VALID_KEY = "abcde-fghij-klmno-pqrst";

/** The host's real pairing: a check failure or a stray symbol is a typing
 *  mistake; a truncation is not. */
function keyFormat(overrides: Partial<LicenceKeyFormat> = {}): LicenceKeyFormat {
  return {
    parse(input) {
      const stripped = input.replace(/[-\s]/g, "");
      if (/[^A-Za-z0-9]/.test(stripped)) {
        return { ok: false, problem: { kind: "unexpectedSymbol", symbol: "!" } };
      }
      if (stripped.length < 20) {
        return { ok: false, problem: { kind: "tooShort", minimum: 20, actual: stripped.length } };
      }
      if (stripped.endsWith("X")) return { ok: false, problem: { kind: "checkFailed" } };
      return { ok: true, key: stripped.toUpperCase(), grouped: stripped.toUpperCase() };
    },
    isProbablyATypo(problem: LicenceKeyProblem) {
      return problem.kind === "checkFailed" || problem.kind === "unexpectedSymbol";
    },
    ...overrides,
  };
}

function mount(props: Partial<LicenceActivationProps> = {}) {
  const onActivate = vi.fn();
  const view = render(
    <LicenceActivation
      keyFormat={keyFormat()}
      accountTokenProvider={{ acquire: async () => null }}
      onActivate={onActivate}
      {...props}
    />,
  );
  return { ...view, onActivate };
}

function form(container: HTMLElement): HTMLFormElement {
  return container.querySelector("form.poodle-licence-activation") as HTMLFormElement;
}

async function submit(container: HTMLElement): Promise<void> {
  await act(async () => {
    fireEvent.submit(form(container));
  });
}

async function chooseRoute(name: RegExp | string): Promise<void> {
  await act(async () => {
    fireEvent.click(screen.getByRole("tab", { name }));
  });
}

async function typeKey(container: HTMLElement, value: string): Promise<void> {
  const input = container.querySelector('input[id$="-key"]') as HTMLInputElement;
  await act(async () => {
    fireEvent.change(input, { target: { value } });
  });
}

describe("LicenceActivation (react)", () => {
  it("presents all three routes as equally visible, reachable tabs", () => {
    mount();
    const tabs = screen.getAllByRole("tab");
    expect(tabs.map((tab) => tab.textContent?.trim())).toEqual(["Key", "Account", "Licence file"]);
    for (const tab of tabs) {
      expect(tab.hasAttribute("disabled")).toBe(false);
      expect(tab.getAttribute("tabindex")).not.toBe(null);
    }
  });

  it("renders no account-token input on any route", async () => {
    const { container } = mount();
    await chooseRoute("Account");
    const inputs = [...container.querySelectorAll("input")].filter(
      (input) => input.type !== "file",
    );
    // The key route is no longer mounted, so only the shared machine label
    // remains. Nothing anywhere asks the customer to paste a token.
    expect(inputs).toHaveLength(1);
    expect(inputs[0].id).toMatch(/-label$/);
    expect(container.textContent).not.toMatch(/paste|token/i);
  });

  it("emits a valid key exactly once, exactly as typed", async () => {
    const { container, onActivate } = mount();
    await typeKey(container, VALID_KEY);
    await submit(container);
    expect(onActivate).toHaveBeenCalledTimes(1);
    expect(onActivate).toHaveBeenCalledWith({
      credential: { kind: "key", key: VALID_KEY },
      label: null,
    });
  });

  it("hands the injected parser lowercase, dashes, whitespace and I/L/O unchanged", async () => {
    const seen: string[] = [];
    const recording = keyFormat();
    const spy: LicenceKeyFormat = {
      parse(input) {
        seen.push(input);
        return recording.parse(input);
      },
      isProbablyATypo: recording.isProbablyATypo,
    };
    const raw = " abcde-fghij klmno-pqrsI lO ";
    const { container } = mount({ keyFormat: spy });
    await typeKey(container, raw);
    await submit(container);
    expect(seen).toEqual([raw]);
  });

  it("renders typo copy for a check failure and emits nothing", async () => {
    const { container, onActivate } = mount();
    await typeKey(container, "abcde-fghij-klmno-pqrsX");
    await submit(container);
    expect(screen.getByText("Check the key for a typing mistake.")).toBeTruthy();
    expect(container.textContent).not.toMatch(/invalid|fake|not recognised|not recognized/i);
    expect(onActivate).not.toHaveBeenCalled();
  });

  it("renders typo copy for an unexpected symbol and emits nothing", async () => {
    const { container, onActivate } = mount();
    await typeKey(container, "abcde-fghij-klmno-pqr$t");
    await submit(container);
    expect(screen.getByText("Check the key for a typing mistake.")).toBeTruthy();
    expect(onActivate).not.toHaveBeenCalled();
  });

  it("renders distinct copy for a too-short key and emits nothing", async () => {
    const { container, onActivate } = mount();
    await typeKey(container, "abc");
    await submit(container);
    expect(screen.getByText("This key is too short.")).toBeTruthy();
    expect(screen.queryByText("Check the key for a typing mistake.")).toBeNull();
    expect(onActivate).not.toHaveBeenCalled();
  });

  it("does not round-trip a rejected key back into the field", async () => {
    const { container } = mount();
    await typeKey(container, "abc");
    await submit(container);
    const input = container.querySelector('input[id$="-key"]') as HTMLInputElement;
    expect(input.value).toBe("abc");
    expect(form(container).outerHTML).not.toMatch(/data-[a-z-]*key="abc"/);
  });

  it("invokes the injected account provider and emits its token", async () => {
    const acquire = vi.fn(async () => "tok_live");
    const { container, onActivate } = mount({ accountTokenProvider: { acquire } });
    await chooseRoute("Account");
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Continue with account" }));
    });
    expect(acquire).toHaveBeenCalledTimes(1);
    expect(onActivate).toHaveBeenCalledWith({
      credential: { kind: "accountToken", token: "tok_live" },
      label: null,
    });
    // The token was never rendered back or parked in an attribute.
    expect(container.innerHTML).not.toContain("tok_live");
  });

  it("treats a cancelled account flow as quiet", async () => {
    const { container, onActivate } = mount({
      accountTokenProvider: { acquire: async () => null },
    });
    await chooseRoute("Account");
    await submit(container);
    expect(onActivate).not.toHaveBeenCalled();
    expect(container.querySelector('[role="status"]')).toBeNull();
  });

  it("reports a failed account flow politely, without credential detail", async () => {
    const { container, onActivate } = mount({
      accountTokenProvider: {
        acquire: async () => {
          throw new Error("tok_secret leaked in the message");
        },
      },
    });
    await chooseRoute("Account");
    await submit(container);
    const message = container.querySelector('[role="status"]') as HTMLElement;
    expect(message.textContent).toBe("Account activation could not be completed.");
    expect(container.innerHTML).not.toContain("tok_secret");
    expect(onActivate).not.toHaveBeenCalled();
  });

  it("emits file bytes as base64 with no data-URL prefix", async () => {
    const { container, onActivate } = mount({ fileAccept: ".licence" });
    await chooseRoute("Licence file");
    const input = container.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(["ABC"], "studio.licence", { type: "application/octet-stream" });
    Object.defineProperty(input, "files", { value: [file], configurable: true });
    await act(async () => {
      fireEvent.change(input);
    });
    // The browser reads the bytes asynchronously. Submitting before the read
    // lands is a rejection, not an emit, so retry until the read is in.
    await waitFor(async () => {
      await submit(container);
      expect(onActivate).toHaveBeenCalledTimes(1);
    });

    const detail = onActivate.mock.calls[0][0] as {
      credential: { kind: string; contentsBase64: string };
    };
    expect(detail.credential.kind).toBe("licenceFile");
    expect(detail.credential.contentsBase64).toBe(btoa("ABC"));
    expect(detail.credential.contentsBase64).not.toContain("data:");
    expect(detail.credential.contentsBase64).not.toContain("base64,");
    // The name may show; the contents never do.
    expect(container.textContent).toContain("studio.licence");
    expect(container.textContent).not.toContain(btoa("ABC"));
  });

  it("asks for a file rather than emitting an empty licence-file route", async () => {
    const { container, onActivate } = mount();
    await chooseRoute("Licence file");
    await submit(container);
    expect(screen.getByRole("status").textContent).toBe("Choose a licence file to continue.");
    expect(onActivate).not.toHaveBeenCalled();
  });

  it("carries the trimmed machine label, and null when it is blank", async () => {
    const { container, onActivate } = mount();
    const label = container.querySelector('input[id$="-label"]') as HTMLInputElement;
    await act(async () => {
      fireEvent.change(label, { target: { value: "   " } });
    });
    await typeKey(container, VALID_KEY);
    await submit(container);
    expect(onActivate).toHaveBeenLastCalledWith({
      credential: { kind: "key", key: VALID_KEY },
      label: null,
    });

    await act(async () => {
      fireEvent.change(label, { target: { value: "  Studio Mac  " } });
    });
    await submit(container);
    expect(onActivate).toHaveBeenLastCalledWith({
      credential: { kind: "key", key: VALID_KEY },
      label: "Studio Mac",
    });
  });

  it("blocks a duplicate submit while pending without hiding a route", async () => {
    const { container, onActivate } = mount({ pending: true });
    expect(form(container).getAttribute("aria-busy")).toBe("true");
    expect(screen.getByRole("button", { name: "Activate" }).hasAttribute("disabled")).toBe(true);
    expect(screen.getAllByRole("tab")).toHaveLength(3);
    await typeKey(container, VALID_KEY);
    await submit(container);
    expect(onActivate).not.toHaveBeenCalled();
  });

  it("disables every route and field when disabled, and still shows all three", () => {
    const { container } = mount({ disabled: true });
    const tabs = screen.getAllByRole("tab");
    expect(tabs).toHaveLength(3);
    expect(tabs.every((tab) => tab.hasAttribute("disabled"))).toBe(true);
    for (const input of container.querySelectorAll("input")) {
      expect(input.hasAttribute("disabled")).toBe(true);
    }
    expect(screen.getByRole("button", { name: "Activate" }).hasAttribute("disabled")).toBe(true);
  });

  it("moves focus to the route's own first field when a route activates", async () => {
    const { container } = mount();
    await chooseRoute("Licence file");
    await waitFor(() => {
      const panel = container.querySelector(".poodle-licence-activation__route") as HTMLElement;
      expect(panel.contains(document.activeElement)).toBe(true);
    });
  });

  it("focuses the key field when its submit is rejected", async () => {
    const { container } = mount();
    await typeKey(container, "abc");
    await submit(container);
    await waitFor(() => {
      expect((document.activeElement as HTMLElement)?.id).toMatch(/-key$/);
    });
  });
});
