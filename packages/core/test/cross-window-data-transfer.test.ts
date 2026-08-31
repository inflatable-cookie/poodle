/**
 * The bounded cross-window envelope — the adversarial half of the wire.
 *
 * `DataTransfer` is untrusted input written by whatever the OS decided to hand
 * the window, so the cases that matter here are the refusals: a body that is
 * not JSON, a body that is JSON but not a receipt, a version this build does
 * not speak, a token outside its bounds, and an envelope large enough that
 * parsing it at all is the attack. The one accepted case has to be exact, and
 * has to carry nothing but the two fields.
 */

import { describe, expect, test } from "bun:test";

import {
  createCrossWindowDataTransferAdapter,
  CROSS_WINDOW_DRAG_MAX_ENVELOPE_LENGTH,
  type CrossWindowDataTransferLike,
} from "../src/dom/cross-window-data-transfer";
import {
  CROSS_WINDOW_DRAG_MAX_TOKEN_LENGTH,
  CROSS_WINDOW_DRAG_MIME_TYPE,
  CROSS_WINDOW_DRAG_PROTOCOL_VERSION,
} from "../src/cross-window-drag";

function transfer(entries: Record<string, string> = {}): CrossWindowDataTransferLike {
  const data = new Map(Object.entries(entries));
  return {
    get types() {
      return [...data.keys()];
    },
    getData: (format: string) => data.get(format) ?? "",
    setData: (format: string, value: string) => {
      data.set(format, value);
    },
  };
}

const receipt = { protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token: "opaque-host-id" };

describe("cross-window DataTransfer codec", () => {
  test("writes exactly the protocol version and token under the contracted MIME type", () => {
    const adapter = createCrossWindowDataTransferAdapter();
    const dataTransfer = transfer();

    adapter.write(dataTransfer, receipt);

    expect(dataTransfer.types).toEqual([CROSS_WINDOW_DRAG_MIME_TYPE]);
    expect(JSON.parse(dataTransfer.getData(CROSS_WINDOW_DRAG_MIME_TYPE))).toEqual({
      protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION,
      token: "opaque-host-id",
    });
    expect(adapter.read(dataTransfer)).toEqual(receipt);
  });

  test("a host protocol may rename the MIME type without changing the receipt shape", () => {
    const adapter = createCrossWindowDataTransferAdapter({ mimeType: "application/x-host-drag" });
    const dataTransfer = transfer();

    adapter.write(dataTransfer, receipt);

    expect(dataTransfer.types).toEqual(["application/x-host-drag"]);
    expect(adapter.accepts(dataTransfer)).toBe(true);
    expect(adapter.read(dataTransfer)).toEqual(receipt);
    expect(createCrossWindowDataTransferAdapter().read(dataTransfer)).toBeNull();
  });

  test("accepts answers only the dragover question and never claims to have read a body", () => {
    const adapter = createCrossWindowDataTransferAdapter();

    // The declared type is all a browser exposes during dragover. An envelope
    // whose body is unreadable then must still be accepted for hover, and
    // still be refused at drop.
    const hover = transfer({ [CROSS_WINDOW_DRAG_MIME_TYPE]: "" });
    expect(adapter.accepts(hover)).toBe(true);
    expect(adapter.read(hover)).toBeNull();

    expect(adapter.accepts(transfer({ "text/plain": "hello" }))).toBe(false);
  });

  test("refuses a body that is not JSON, not an object, or not a receipt", () => {
    const adapter = createCrossWindowDataTransferAdapter();

    for (const body of [
      "not json",
      "null",
      "42",
      '"a string"',
      "[]",
      "{}",
      JSON.stringify({ token: "no-version" }),
      JSON.stringify({ protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION }),
      JSON.stringify({ protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token: 7 }),
      JSON.stringify({ protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token: "" }),
    ]) {
      expect(adapter.read(transfer({ [CROSS_WINDOW_DRAG_MIME_TYPE]: body }))).toBeNull();
    }
  });

  test("refuses a future protocol version rather than parsing it best-effort", () => {
    const adapter = createCrossWindowDataTransferAdapter();
    const body = JSON.stringify({
      protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION + 1,
      token: "opaque-host-id",
    });

    expect(adapter.read(transfer({ [CROSS_WINDOW_DRAG_MIME_TYPE]: body }))).toBeNull();
  });

  test("refuses an oversized token and an oversized envelope", () => {
    const adapter = createCrossWindowDataTransferAdapter();

    const longToken = JSON.stringify({
      protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION,
      token: "t".repeat(CROSS_WINDOW_DRAG_MAX_TOKEN_LENGTH + 1),
    });
    expect(adapter.read(transfer({ [CROSS_WINDOW_DRAG_MIME_TYPE]: longToken }))).toBeNull();

    // Padded past the envelope bound with an otherwise valid receipt: the
    // length check has to come before JSON.parse, or the bound buys nothing.
    const padded = `${JSON.stringify(receipt)}${" ".repeat(CROSS_WINDOW_DRAG_MAX_ENVELOPE_LENGTH)}`;
    expect(adapter.read(transfer({ [CROSS_WINDOW_DRAG_MIME_TYPE]: padded }))).toBeNull();
  });

  test("normalizes away extra keys instead of passing a host record onward", () => {
    const adapter = createCrossWindowDataTransferAdapter();
    const body = JSON.stringify({
      protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION,
      token: "opaque-host-id",
      panel: { id: "mixer", path: "/tmp/session" },
      geometry: { x: 10, y: 20 },
    });

    const decoded = adapter.read(transfer({ [CROSS_WINDOW_DRAG_MIME_TYPE]: body }));

    expect(decoded).toEqual(receipt);
    expect(Object.keys(decoded ?? {})).toEqual(["protocolVersion", "token"]);
  });

  test("refuses to write a receipt this build cannot carry", () => {
    const adapter = createCrossWindowDataTransferAdapter();
    const dataTransfer = transfer();

    expect(() =>
      adapter.write(dataTransfer, {
        protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION + 1,
        token: "opaque-host-id",
      }),
    ).toThrow();
    expect(dataTransfer.types).toEqual([]);
  });
});
