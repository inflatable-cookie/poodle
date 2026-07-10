import { describe, expect, test } from "bun:test";

import {
  clampNullable,
  isValidSlugFormat,
  parseNumberish,
  parseStep,
  slugify,
  validationStatusToState,
} from "../src/input";

describe("validationStatusToState", () => {
  test("maps statuses; idle falls back", () => {
    expect(validationStatusToState("validating", "none")).toBe("pending");
    expect(validationStatusToState("valid", "none")).toBe("valid");
    expect(validationStatusToState("invalid", "none")).toBe("invalid");
    expect(validationStatusToState("idle", "valid")).toBe("valid");
  });
});

describe("parseNumberish / parseStep / clampNullable", () => {
  test("numeric coercion with null for empty and non-finite", () => {
    expect(parseNumberish("42")).toBe(42);
    expect(parseNumberish("")).toBeNull();
    expect(parseNumberish(null)).toBeNull();
    expect(parseNumberish("abc")).toBeNull();
    expect(parseNumberish("Infinity")).toBeNull();
  });

  test("step falls back to 1 for invalid or non-positive", () => {
    expect(parseStep("0.5")).toBe(0.5);
    expect(parseStep(null)).toBe(1);
    expect(parseStep("0")).toBe(1);
    expect(parseStep("-2")).toBe(1);
    expect(parseStep("x")).toBe(1);
  });

  test("clampNullable respects open bounds", () => {
    expect(clampNullable(5, 0, 10)).toBe(5);
    expect(clampNullable(-1, 0, null)).toBe(0);
    expect(clampNullable(99, null, 10)).toBe(10);
    expect(clampNullable(99, null, null)).toBe(99);
  });
});

describe("slugify / isValidSlugFormat", () => {
  test("strips diacritics, kebab-cases, collapses dashes", () => {
    expect(slugify("Héllo Wörld")).toBe("hello-world");
    // underscores are stripped by the charset pass before the separator
    // pass runs — matches the original TextInput implementation
    expect(slugify("  Foo_Bar  Baz ")).toBe("foobar-baz");
    expect(slugify("a--b---c")).toBe("a-b-c");
    expect(slugify("-lead and trail-")).toBe("lead-and-trail");
    expect(slugify("Ça va? Très bien!")).toBe("ca-va-tres-bien");
  });

  test("format validation: charset, min 2, max limit", () => {
    expect(isValidSlugFormat("valid-slug")).toBe(true);
    expect(isValidSlugFormat("a")).toBe(false);
    expect(isValidSlugFormat("-bad")).toBe(false);
    expect(isValidSlugFormat("UPPER")).toBe(false);
    expect(isValidSlugFormat("x".repeat(101))).toBe(false);
    expect(isValidSlugFormat("x".repeat(10), 5)).toBe(false);
  });
});
