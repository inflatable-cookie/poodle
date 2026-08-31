/**
 * External-file boundaries — the pure validation claims.
 *
 * These are the rules that run before any target, any adapter, and any host
 * command: what a prepared export is allowed to be, and what an inbound batch
 * has to look like before eligibility is even asked. The Rust mirror
 * (`packages/contracts/headless/src/external_file_drag.rs`) states the same
 * claims against the same vocabulary; wiring, lifecycle, and cleanup live in
 * `test/headless-dom/inbound-files-and-drag-out.test.ts`.
 */

import { describe, expect, test } from "bun:test";

import {
  canExportAnything,
  INBOUND_FILE_PROTOCOL_VERSION,
  EXTERNAL_FILE_MAX_NAME_LENGTH,
  EXTERNAL_FILE_MAX_RECEIPT_LENGTH,
  isPresentableFileName,
  validateFileExport,
  validateInboundFiles,
  type DragExportCapabilities,
  type InboundFileBatch,
  type InboundFileCapabilities,
  type InboundFileReceipt,
  type PreparedFileExport,
} from "../src/external-file-drag";

const exportCapabilities: DragExportCapabilities = {
  files: true,
  multipleFiles: false,
  promisedFiles: false,
  customDataTypes: [],
};

const inboundCapabilities: InboundFileCapabilities = {
  files: true,
  multipleFiles: true,
  transport: "data-transfer",
  customDataTypes: [],
};

function prepared(overrides: Partial<PreparedFileExport> = {}): PreparedFileExport {
  return { receiptId: "lease-1", form: "existing-file", ...overrides };
}

function file(
  id: string,
  name: string,
  mediaType: string,
  size: number | null,
): InboundFileReceipt {
  return { receiptId: id, name, mediaType, size };
}

/** A hover-time receipt: the platform has declared a type and nothing else. */
function hovering(id: string, mediaType: string): InboundFileReceipt {
  return { receiptId: id, name: null, mediaType, size: null };
}

function batch(files: InboundFileReceipt[]): InboundFileBatch {
  return {
    protocolVersion: INBOUND_FILE_PROTOCOL_VERSION,
    batchId: "batch-1",
    transport: "data-transfer",
    files,
  };
}

function exportRefusal(
  value: PreparedFileExport,
  capabilities = exportCapabilities,
): string | null {
  const result = validateFileExport(value, capabilities);
  return result.accepted ? null : result.reason;
}

function inboundRefusal(
  value: InboundFileBatch,
  constraints = {},
  capabilities = inboundCapabilities,
): string | null {
  const result = validateInboundFiles(value, constraints, capabilities);
  return result.accepted ? null : result.reason;
}

describe("display names", () => {
  /**
   * A display name is the one field with a plausible excuse to carry a
   * location, so every location shape is refused — including the Windows and
   * URL forms that contain no separator at all. Refused, not trimmed: quietly
   * presenting the last segment would hide the leak rather than stop it.
   */
  test("a name that is really a location is not presentable", () => {
    expect(isPresentableFileName("take-01.wav")).toBe(true);
    expect(isPresentableFileName("mix (final).aiff")).toBe(true);
    expect(isPresentableFileName("/Users/tom/take-01.wav")).toBe(false);
    expect(isPresentableFileName("..\\take-01.wav")).toBe(false);
    expect(isPresentableFileName("C:take-01.wav")).toBe(false);
    expect(isPresentableFileName("file:take-01.wav")).toBe(false);
    expect(isPresentableFileName("..")).toBe(false);
    expect(isPresentableFileName("")).toBe(false);
    expect(isPresentableFileName("   ")).toBe(false);
    expect(isPresentableFileName("a".repeat(EXTERNAL_FILE_MAX_NAME_LENGTH + 1))).toBe(false);
  });
});

describe("validateFileExport", () => {
  test("a receipt cannot exceed the capabilities its own adapter advertised", () => {
    expect(exportRefusal(prepared())).toBeNull();
    expect(exportRefusal(prepared({ form: "materialized-file", fileCount: 3 }))).toBe(
      "multiple-files-unsupported",
    );
    expect(exportRefusal(prepared({ form: "promised-file" }))).toBe(
      "promised-files-unsupported",
    );
    expect(
      exportRefusal(prepared(), { ...exportCapabilities, files: false }),
    ).toBe("files-unsupported");
  });

  test("multiple files pass exactly when the adapter says they can", () => {
    const many = { ...exportCapabilities, multipleFiles: true };
    expect(exportRefusal(prepared({ fileCount: 3 }), many)).toBeNull();
    expect(exportRefusal(prepared({ fileCount: 0 }), many)).toBe("count-out-of-range");
    expect(exportRefusal(prepared({ fileCount: 1.5 }), many)).toBe("count-out-of-range");
  });

  /** Both sides opt in: the adapter advertises the type, the consumer asks for it. */
  test("custom data needs both sides to name the same type", () => {
    const custom: DragExportCapabilities = {
      files: false,
      multipleFiles: false,
      promisedFiles: false,
      customDataTypes: ["application/x-loophole-clip"],
    };
    expect(
      exportRefusal(
        prepared({ form: "custom-data", dataTypes: ["application/x-loophole-clip"] }),
        custom,
      ),
    ).toBeNull();
    expect(
      exportRefusal(prepared({ form: "custom-data", dataTypes: ["application/x-other"] }), custom),
    ).toBe("custom-data-unsupported");
    expect(exportRefusal(prepared({ form: "custom-data" }), custom)).toBe(
      "custom-data-unsupported",
    );
    expect(canExportAnything(custom)).toBe(true);
    expect(canExportAnything({ ...custom, customDataTypes: [] })).toBe(false);
  });

  test("an unbounded or absent receipt id is not an export", () => {
    expect(exportRefusal(prepared({ receiptId: "" }))).toBe("no-receipt");
    expect(
      exportRefusal(prepared({ receiptId: "t".repeat(EXTERNAL_FILE_MAX_RECEIPT_LENGTH + 1) })),
    ).toBe("no-receipt");
  });

  test("a display name that is a path refuses the whole export", () => {
    expect(
      exportRefusal(prepared({ form: "materialized-file", displayName: "/tmp/take-01.wav" })),
    ).toBe("name-is-a-path");
    expect(exportRefusal(prepared({ displayName: "take-01.wav" }))).toBeNull();
  });
});

describe("validateInboundFiles", () => {
  /**
   * The platform hides sizes during hover, so an unknown size passes and the
   * real one is caught at drop. Guessing would refuse every browser file drag
   * before it could be inspected; failing to re-check would let an oversized
   * file through on hover acceptance alone.
   */
  test("an unknown hover size passes and the real drop size does not", () => {
    const constraints = { maxSize: 1_000 };
    expect(inboundRefusal(batch([file("f1", "take.wav", "audio/wav", null)]), constraints)).toBeNull();
    expect(inboundRefusal(batch([file("f1", "take.wav", "audio/wav", 2_000)]), constraints)).toBe(
      "too-large",
    );
  });

  test("count, type, identity, and name are all checked before eligibility", () => {
    const constraints = { maxFiles: 2, accept: "audio/*" };
    const two = batch([
      file("f1", "a.wav", "audio/wav", 10),
      file("f2", "b.aiff", "audio/aiff", 10),
    ]);
    expect(inboundRefusal(two, constraints)).toBeNull();

    expect(
      inboundRefusal(
        batch([
          file("f1", "a.wav", "audio/wav", 10),
          file("f2", "b.wav", "audio/wav", 10),
          file("f3", "c.wav", "audio/wav", 10),
        ]),
        constraints,
      ),
    ).toBe("too-many");
    expect(inboundRefusal(batch([file("f1", "notes.txt", "text/plain", 10)]), constraints)).toBe(
      "unsupported-type",
    );
    expect(
      inboundRefusal(
        batch([file("f1", "a.wav", "audio/wav", 10), file("f1", "b.wav", "audio/wav", 10)]),
        constraints,
      ),
    ).toBe("unidentified");
    expect(inboundRefusal(batch([file("f1", "/tmp/a.wav", "audio/wav", 10)]), constraints)).toBe(
      "name-is-a-path",
    );
    expect(inboundRefusal(batch([]), constraints)).toBe("empty");
    expect(inboundRefusal({ ...two, batchId: "" }, constraints)).toBe("unidentified");
    expect(inboundRefusal(batch([file("f1", "a.wav", "audio/wav", -1)]), constraints)).toBe(
      "malformed",
    );
  });

  /**
   * Hover discloses a type and nothing else, so an extension rule cannot be
   * answered yet and defers rather than refusing a drag the drop would
   * accept. A type rule the platform *can* answer is still enforced.
   */
  test("an undisclosed hover name defers extension rules and keeps type rules", () => {
    const byExtension = { accept: ".wav" };
    const byType = { accept: "audio/*" };

    expect(inboundRefusal(batch([hovering("f1", "audio/wav")]), byExtension)).toBeNull();
    expect(inboundRefusal(batch([hovering("f1", "audio/wav")]), byType)).toBeNull();
    expect(inboundRefusal(batch([hovering("f1", "text/plain")]), byType)).toBe("unsupported-type");
    // An undeclared type cannot be compared either, so it defers as well.
    expect(inboundRefusal(batch([hovering("f1", "")]), byType)).toBeNull();
    // The drop discloses the name the hover deferred on.
    expect(inboundRefusal(batch([file("f1", "notes.txt", "text/plain", 1)]), byExtension)).toBe(
      "unsupported-type",
    );
  });

  /**
   * A batch is assembled by an adapter that ships separately from this
   * package. One whose shape this build cannot fully understand is refused
   * before any other field is read, because none of them is trustworthy yet.
   */
  test("a batch from another protocol version is refused first", () => {
    const good = batch([file("f1", "a.wav", "audio/wav", 10)]);
    expect(inboundRefusal(good)).toBeNull();

    for (const protocolVersion of [0, INBOUND_FILE_PROTOCOL_VERSION + 1]) {
      expect(inboundRefusal({ ...good, protocolVersion })).toBe("unsupported-protocol");
    }

    // Refused *first*: a batch that is also empty and on the wrong transport
    // still reports the version, because nothing after it was trustworthy
    // enough to check.
    expect(
      inboundRefusal({
        ...good,
        protocolVersion: INBOUND_FILE_PROTOCOL_VERSION + 1,
        transport: "host",
        files: [],
      }),
    ).toBe("unsupported-protocol");
  });

  /** One window hands ownership to exactly one transport. */
  test("a batch from the transport this window did not enable is refused", () => {
    const foreign: InboundFileBatch = {
      protocolVersion: INBOUND_FILE_PROTOCOL_VERSION,
      batchId: "batch-1",
      transport: "host",
      files: [file("f1", "a.wav", "audio/wav", 10)],
    };
    expect(inboundRefusal(foreign)).toBe("malformed");
    expect(
      inboundRefusal(foreign, {}, { ...inboundCapabilities, transport: "host" }),
    ).toBeNull();
  });

  /** The transport's ceiling is not the target's to raise. */
  test("a single-file transport refuses a multi-file batch with no target limit", () => {
    const single = { ...inboundCapabilities, multipleFiles: false };
    const two = batch([file("f1", "a.wav", "audio/wav", 10), file("f2", "b.wav", "audio/wav", 10)]);
    expect(inboundRefusal(two, {}, single)).toBe("too-many");
    expect(inboundRefusal(two, { maxFiles: 5 }, single)).toBe("too-many");
    expect(inboundRefusal(two, {}, { ...inboundCapabilities, files: false })).toBe(
      "files-unsupported",
    );
  });
});
