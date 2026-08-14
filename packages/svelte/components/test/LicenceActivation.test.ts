import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { tick } from "svelte";
import { afterEach, describe, expect, it, vi } from "vitest";

import LicenceActivation from "../src/LicenceActivation.svelte";
import LicenceActivationAccountHarness from "./LicenceActivationAccountHarness.svelte";
import type { LicenceKeyFormat, LicenceKeyProblem } from "@inflatable-cookie/poodle-core";

const VALID_KEY = "abcde-fghij-klmno-pqrst";

function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((done) => {
    resolve = done;
  });
  return { promise, resolve };
}

const deferredReaders: DeferredFileReader[] = [];

class DeferredFileReader {
  static readonly EMPTY = 0;
  static readonly LOADING = 1;
  static readonly DONE = 2;
  readonly LOADING = DeferredFileReader.LOADING;
  readyState = DeferredFileReader.EMPTY;
  result: string | ArrayBuffer | null = null;
  onload: ((event: ProgressEvent<FileReader>) => void) | null = null;
  onerror: ((event: ProgressEvent<FileReader>) => void) | null = null;

  constructor() {
    deferredReaders.push(this);
  }

  readAsDataURL(): void {
    this.readyState = DeferredFileReader.LOADING;
  }

  abort(): void {
    this.readyState = DeferredFileReader.DONE;
  }

  resolve(result: string): void {
    this.result = result;
    this.readyState = DeferredFileReader.DONE;
    this.onload?.({} as ProgressEvent<FileReader>);
  }
}

afterEach(() => {
  deferredReaders.length = 0;
  vi.unstubAllGlobals();
});

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

function mountKey(props: Record<string, unknown> = {}) {
  const onActivate = vi.fn();
  const view = render(LicenceActivation, {
    props: {
      mode: "key",
      keyFormat: keyFormat(),
      onActivate,
      ...props,
    } as never,
  });
  return { ...view, onActivate };
}

function mountAccount(props: Record<string, unknown> = {}) {
  const onActivate = vi.fn();
  const view = render(LicenceActivation, {
    props: {
      mode: "account",
      accountTokenProvider: { acquire: async () => null },
      onActivate,
      ...props,
    } as never,
  });
  return { ...view, onActivate };
}

function form(container: HTMLElement): HTMLFormElement {
  return container.querySelector("form.poodle-licence-activation") as HTMLFormElement;
}

async function submit(container: HTMLElement): Promise<void> {
  await fireEvent.submit(form(container));
  await tick();
  await tick();
}

async function chooseAccountRoute(name: "Activate offline" | "Use account activation"): Promise<void> {
  await fireEvent.click(screen.getByRole("button", { name }));
  await tick();
}

async function typeKey(container: HTMLElement, value: string): Promise<void> {
  const input = container.querySelector('input[id$="-key"]') as HTMLInputElement;
  await fireEvent.input(input, { target: { value } });
  await tick();
}

describe("LicenceActivation (svelte)", () => {
  it("keeps key activation separate from account activation with its offline fallback", () => {
    const key = mountKey();
    expect(key.getByText("Licence key")).toBeTruthy();
    expect(key.queryByRole("button", { name: "Activate offline" })).toBeNull();
    key.unmount();

    const account = mountAccount();
    expect(account.getByRole("button", { name: "Continue with account" })).toBeTruthy();
    const offline = account.getByRole("button", { name: "Activate offline" });
    expect(offline.closest(".poodle-licence-activation__header")).toBeTruthy();
    expect(offline.classList.contains("poodle-button")).toBe(true);
    expect(offline.classList.contains("poodle-text-link")).toBe(false);
    expect(offline.getAttribute("data-variant")).toBe("ghost");
    expect(offline.getAttribute("data-size")).toBe("xs");
    expect(offline.querySelector("svg.poodle-icon")).toBeTruthy();
    expect(account.queryByText("Licence key")).toBeNull();
    expect(account.queryAllByRole("tab")).toHaveLength(0);
  });

  it("opts into grouped CodeInput key entry without changing the default field", async () => {
    const plain = mountKey();
    expect(plain.container.querySelector(".poodle-code-input")).toBeNull();
    plain.unmount();

    const { container, onActivate } = mountKey({
      keyCodeInput: { length: 20, groups: [5, 5, 5, 5], separator: "-" },
    });
    const slots = [...container.querySelectorAll(".poodle-code-input__slot")];
    expect(slots).toHaveLength(20);
    expect(
      slots
        .map((slot, index) =>
          slot.classList.contains("poodle-code-input__slot--group-end") ? index : null,
        )
        .filter((index) => index !== null),
    ).toEqual([4, 9, 14]);
    expect(
      [...container.querySelectorAll(".poodle-code-input__separator")].map(
        (separator) => separator.textContent,
      ),
    ).toEqual(["-", "-", "-"]);

    await typeKey(container, "abcdefghijklmnopqrst");
    await waitFor(() =>
      expect(container.querySelector('[aria-label="Code check passed"]')).not.toBeNull(),
    );
    await submit(container);
    expect(onActivate).toHaveBeenCalledWith({
      credential: { kind: "key", key: "abcdefghijklmnopqrst" },
      label: null,
    });
  });

  it("renders neither an account-token input nor machine naming by default", () => {
    const { container } = mountAccount();
    const inputs = [...container.querySelectorAll("input")].filter(
      (input) => input.type !== "file",
    );
    expect(inputs).toHaveLength(0);
    expect(screen.queryByRole("button", { name: "Edit machine name" })).toBeNull();
    expect(container.textContent).not.toMatch(/paste|token/i);
  });

  it("emits a valid key exactly once, exactly as typed", async () => {
    const { container, onActivate } = mountKey();
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
    const { container } = mountKey({ keyFormat: spy });
    await typeKey(container, raw);
    await submit(container);
    expect(seen).toEqual([raw]);
  });

  it("renders typo copy for a check failure and emits nothing", async () => {
    const { container, onActivate } = mountKey();
    await typeKey(container, "abcde-fghij-klmno-pqrsX");
    await submit(container);
    expect(screen.getByText("Check the key for a typing mistake.")).toBeTruthy();
    expect(container.textContent).not.toMatch(/invalid|fake|not recognised|not recognized/i);
    expect(onActivate).not.toHaveBeenCalled();
  });

  it("renders typo copy for an unexpected symbol and emits nothing", async () => {
    const { container, onActivate } = mountKey();
    await typeKey(container, "abcde-fghij-klmno-pqr$t");
    await submit(container);
    expect(screen.getByText("Check the key for a typing mistake.")).toBeTruthy();
    expect(onActivate).not.toHaveBeenCalled();
  });

  it("renders distinct copy for a too-short key and emits nothing", async () => {
    const { container, onActivate } = mountKey();
    await typeKey(container, "abc");
    await submit(container);
    expect(screen.getByText("This key is too short.")).toBeTruthy();
    expect(screen.queryByText("Check the key for a typing mistake.")).toBeNull();
    expect(onActivate).not.toHaveBeenCalled();
  });

  it("does not round-trip a rejected key back into the field", async () => {
    const { container } = mountKey();
    await typeKey(container, "abc");
    await submit(container);
    const input = container.querySelector('input[id$="-key"]') as HTMLInputElement;
    expect(input.value).toBe("abc");
    expect(form(container).outerHTML).not.toMatch(/data-[a-z-]*key="abc"/);
  });

  it("invokes the injected account provider and emits its token", async () => {
    const acquire = vi.fn(async () => "tok_live");
    const { container, onActivate } = mountAccount({ accountTokenProvider: { acquire } });
    await fireEvent.click(screen.getByRole("button", { name: "Continue with account" }));
    await tick();
    await tick();
    expect(acquire).toHaveBeenCalledTimes(1);
    expect(onActivate).toHaveBeenCalledWith({
      credential: { kind: "accountToken", token: "tok_live" },
      label: null,
    });
    // The token was never rendered back or parked in an attribute.
    expect(container.innerHTML).not.toContain("tok_live");
  });

  it("submits host-owned account fields through the injected provider", async () => {
    const acquire = vi.fn(async (email: string) => `token-for:${email}`);
    const onActivate = vi.fn();
    const { container } = render(LicenceActivationAccountHarness, {
      props: { acquire, onActivate },
    });

    await fireEvent.input(screen.getByLabelText("Account email"), {
      target: { value: "studio@example.com" },
    });
    await fireEvent.click(screen.getByRole("button", { name: "Activate" }));
    await tick();
    await tick();

    expect(acquire).toHaveBeenCalledWith("studio@example.com");
    expect(onActivate).toHaveBeenCalledWith({
      credential: { kind: "accountToken", token: "token-for:studio@example.com" },
      label: null,
    });
    expect(container.querySelectorAll("form")).toHaveLength(1);
  });

  it("freezes an account acquisition and emits the label captured at submit", async () => {
    const acquisition = deferred<string | null>();
    const { container, onActivate } = mountAccount({
      accountTokenProvider: { acquire: () => acquisition.promise },
      machineLabel: "Studio Mac",
    });
    const label = screen.getByRole("button", { name: "Edit machine name" });
    await fireEvent.submit(form(container));
    await tick();

    expect(form(container).getAttribute("aria-busy")).toBe("true");
    expect(screen.getByRole("button", { name: "Activate offline" }).hasAttribute("disabled")).toBe(true);
    expect(label.hasAttribute("disabled")).toBe(true);

    acquisition.resolve("tok_deferred");
    await tick();
    await tick();
    expect(onActivate).toHaveBeenCalledWith({
      credential: { kind: "accountToken", token: "tok_deferred" },
      label: "Studio Mac",
    });
  });

  it("treats a cancelled account flow as quiet", async () => {
    const { container, onActivate } = mountAccount({
      accountTokenProvider: { acquire: async () => null },
    });
    await submit(container);
    expect(onActivate).not.toHaveBeenCalled();
    expect(container.querySelector('[role="status"]')).toBeNull();
  });

  it("reports a failed account flow politely, without credential detail", async () => {
    const { container, onActivate } = mountAccount({
      accountTokenProvider: {
        acquire: async () => {
          throw new Error("tok_secret leaked in the message");
        },
      },
    });
    await submit(container);
    const message = container.querySelector('[role="status"]') as HTMLElement;
    expect(message.textContent).toBe("Account activation could not be completed.");
    expect(container.innerHTML).not.toContain("tok_secret");
    expect(onActivate).not.toHaveBeenCalled();
  });

  it("emits file bytes as base64 with no data-URL prefix", async () => {
    const { container, onActivate } = mountAccount({ fileAccept: ".licence" });
    await chooseAccountRoute("Activate offline");
    const input = container.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(["ABC"], "studio.licence", { type: "application/octet-stream" });
    Object.defineProperty(input, "files", { value: [file], configurable: true });
    await fireEvent.change(input);
    // The browser reads the bytes asynchronously. Submitting before the read
    // lands is a rejection, not an emit, so retry until the read is in.
    await vi.waitFor(async () => {
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
    const { container, onActivate } = mountAccount();
    await chooseAccountRoute("Activate offline");
    await submit(container);
    expect(screen.getByRole("status").textContent).toBe("Choose a licence file to continue.");
    const input = container.querySelector('input[type="file"]') as HTMLInputElement;
    const messageId = screen.getByRole("status").id;
    expect(input.getAttribute("aria-describedby")).toBe(messageId);
    await vi.waitFor(() => {
      expect((document.activeElement as HTMLElement).getAttribute("aria-describedby")).toBe(
        messageId,
      );
    });
    expect(onActivate).not.toHaveBeenCalled();
  });

  it("clears completed file bytes when the file route is left", async () => {
    const { container, onActivate } = mountAccount();
    await chooseAccountRoute("Activate offline");
    const input = container.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(["ABC"], "studio.licence", { type: "application/octet-stream" });
    Object.defineProperty(input, "files", { value: [file], configurable: true });
    await fireEvent.change(input);
    await vi.waitFor(async () => {
      await submit(container);
      expect(onActivate).toHaveBeenCalledTimes(1);
    });
    onActivate.mockClear();

    await chooseAccountRoute("Use account activation");
    await chooseAccountRoute("Activate offline");
    await submit(container);
    expect(screen.getByRole("status").textContent).toBe("Choose a licence file to continue.");
    expect(onActivate).not.toHaveBeenCalled();
  });

  it("ignores a file read that completes after the file is removed", async () => {
    vi.stubGlobal("FileReader", DeferredFileReader as unknown as typeof FileReader);
    const { container, onActivate } = mountAccount();
    await chooseAccountRoute("Activate offline");
    const input = container.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(["ABC"], "studio.licence", { type: "application/octet-stream" });
    Object.defineProperty(input, "files", { value: [file], configurable: true });
    await fireEvent.change(input);
    await tick();
    const reader = deferredReaders[0];
    await fireEvent.click(screen.getByRole("button", { name: "Remove studio.licence" }));
    reader.resolve("data:application/octet-stream;base64,QUJD");
    await tick();

    await submit(container);
    expect(screen.getByRole("status").textContent).toBe("Choose a licence file to continue.");
    expect(onActivate).not.toHaveBeenCalled();
  });

  it("clears a premature file-required message when the selected read completes", async () => {
    vi.stubGlobal("FileReader", DeferredFileReader as unknown as typeof FileReader);
    const { container, onActivate } = mountAccount();
    await chooseAccountRoute("Activate offline");
    const input = container.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(["ABC"], "studio.licence", { type: "application/octet-stream" });
    Object.defineProperty(input, "files", { value: [file], configurable: true });
    await fireEvent.change(input);
    await tick();

    await submit(container);
    expect(screen.getByRole("status").textContent).toBe("Choose a licence file to continue.");
    deferredReaders[0].resolve("data:application/octet-stream;base64,QUJD");
    await tick();
    expect(screen.queryByRole("status")).toBeNull();

    await submit(container);
    expect(onActivate).toHaveBeenCalledWith({
      credential: { kind: "licenceFile", contentsBase64: "QUJD" },
      label: null,
    });
  });

  it("shows an empty opted-in machine label as placeholder, never as a value", async () => {
    const { container, onActivate } = mountKey({ machineLabel: null });
    const display = screen.getByRole("button", { name: "Edit machine name" });
    const actions = display.closest(".poodle-licence-activation__actions");
    expect(actions?.querySelector('button[type="submit"]')).toBeTruthy();
    expect(display.textContent).toContain("unnamed machine");
    expect(display.classList.contains("poodle-editable-label__display--empty")).toBe(true);

    await fireEvent.click(display);
    await tick();
    const label = container.querySelector(".poodle-editable-label__input") as HTMLInputElement;
    expect(label.value).toBe("");
    expect(label.placeholder).toBe("unnamed machine");

    await typeKey(container, VALID_KEY);
    await submit(container);
    expect(onActivate).toHaveBeenLastCalledWith({
      credential: { kind: "key", key: VALID_KEY },
      label: null,
    });
  });

  it("carries the trimmed committed machine label", async () => {
    const { container, onActivate } = mountKey({ machineLabel: "Studio Mac" });
    await fireEvent.click(screen.getByRole("button", { name: "Edit machine name" }));
    await tick();
    const label = container.querySelector(".poodle-editable-label__input") as HTMLInputElement;
    await fireEvent.input(label, { target: { value: "  Tour laptop  " } });
    expect(await fireEvent.keyDown(label, { key: "Enter" })).toBe(false);
    await tick();
    expect(onActivate).not.toHaveBeenCalled();
    await typeKey(container, VALID_KEY);
    await submit(container);
    expect(onActivate).toHaveBeenLastCalledWith({
      credential: { kind: "key", key: VALID_KEY },
      label: "Tour laptop",
    });
  });

  it("blocks a duplicate submit while pending", async () => {
    const { container, onActivate } = mountKey({ pending: true });
    expect(form(container).getAttribute("aria-busy")).toBe("true");
    expect(screen.getByRole("button", { name: "Activate" }).hasAttribute("disabled")).toBe(true);
    await typeKey(container, VALID_KEY);
    await submit(container);
    expect(onActivate).not.toHaveBeenCalled();
  });

  it("disables the account route switch, submit, account fields, and machine edit", () => {
    const { container } = mountAccount({ disabled: true, machineLabel: "Studio Mac" });
    for (const input of container.querySelectorAll("input")) {
      expect(input.hasAttribute("disabled")).toBe(true);
    }
    expect(screen.getByRole("button", { name: "Edit machine name" }).hasAttribute("disabled")).toBe(true);
    expect(screen.getByRole("button", { name: "Activate offline" }).hasAttribute("disabled")).toBe(true);
    expect(screen.getByRole("button", { name: "Continue with account" }).hasAttribute("disabled")).toBe(true);
  });

  it("moves focus to the route's own first field when a route activates", async () => {
    const { container } = mountAccount();
    await chooseAccountRoute("Activate offline");
    await vi.waitFor(() => {
      const panel = container.querySelector(".poodle-licence-activation__route") as HTMLElement;
      expect(panel.contains(document.activeElement)).toBe(true);
    });
  });

  it("focuses the key field when its submit is rejected", async () => {
    const { container } = mountKey();
    await typeKey(container, "abc");
    await submit(container);
    await vi.waitFor(() => {
      expect((document.activeElement as HTMLElement)?.id).toMatch(/-key$/);
    });
  });
});
