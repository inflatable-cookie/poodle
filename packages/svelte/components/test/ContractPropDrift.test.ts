// Regression tests for the contract-prop-drift parser and the reverse
// drift direction (b027 Part 2). The string-literal fixtures are the real
// lines that exposed the bug: DateTimeZonePicker.svelte's
// `placeholder = "Select date, time, and zone"` and
// `defaultValue = { date: null, time: null, timeZone: null }` — the gate
// reported `and` and `time` as props because a comma inside a string literal
// read as a prop boundary.

import { readFileSync } from "node:fs";
import path from "node:path";

import { describe, expect, it } from "vitest";

import {
  componentDrift,
  contractProps,
  snippetProps,
  svelteProps,
} from "../../preview/scripts/contract-prop-drift.ts";

const svelteComponentsDir = path.join(import.meta.dirname, "../src");

describe("svelteProps string-literal parsing", () => {
  it("does not treat a comma inside a string literal as a prop boundary", () => {
    // The exact line that exposed the bug.
    const src = `
      <script lang="ts">
        interface Props { placeholder?: string; }
        let {
          value = undefined,
          defaultValue = { date: null, time: null, timeZone: null },
          placeholder = "Select date, time, and zone",
          onValueChange = undefined,
        }: Props = $props();
      </script>`;
    const props = svelteProps(src);
    expect(props.has("placeholder")).toBe(true);
    expect(props.has("defaultValue")).toBe(true);
    // String content is not props.
    expect(props.has("and")).toBe(false);
    expect(props.has("time")).toBe(false);
    // Object-literal members were already depth-skipped; unchanged.
    expect(props.has("date")).toBe(false);
    expect(props.has("timeZone")).toBe(false);
    // Event callbacks stay excluded.
    expect(props.has("onValueChange")).toBe(false);
  });

  it("parses the real DateTimeZonePicker without leaking string commas", () => {
    const src = readFileSync(
      path.join(svelteComponentsDir, "DateTimeZonePicker.svelte"),
      "utf8",
    );
    const props = svelteProps(src);
    expect(props.has("placeholder")).toBe(true);
    expect(props.has("and")).toBe(false);
    expect(props.has("time")).toBe(false);
  });
});

describe("contractProps cell parsing", () => {
  it("reads comma-joined prop cells as separate props", () => {
    const md = `### Public Props
| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| \`x\`, \`y\` | number | 0 | bindable axis values |
| \`defaultX\`, \`defaultY\` | number | 0 | reset pair |`;
    const { props } = contractProps(md);
    expect([...props].sort()).toEqual(["defaultX", "defaultY", "x", "y"]);
  });

  it("reads slash-joined prop cells as separate props", () => {
    const md = `### Public Props
| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| \`primaryHidden\` / \`secondaryHidden\` | boolean | false | no | absent, not collapsed |`;
    const { props } = contractProps(md);
    expect([...props].sort()).toEqual(["primaryHidden", "secondaryHidden"]);
  });

  it("separates target-specific props from documented ones", () => {
    const md = `### Public Props
| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| \`size\` | number | 1 | no | a real prop |
| \`class\` | string | "" | no | **Web targets only** styling passthrough |`;
    const { props, targetSpecific } = contractProps(md);
    expect([...props]).toEqual(["size"]);
    expect([...targetSpecific]).toEqual(["class"]);
  });
});

describe("snippetProps", () => {
  it("excludes Snippet-typed props, including doc-commented entries", () => {
    const src = `
      <script lang="ts">
        interface Props {
          items: string[];
          /** The question region. */
          question?: Snippet;
          plan?: Snippet<[string]>;
          onValueChange?: ((v: string) => void) | undefined;
          children?: Snippet;
        }
      </script>`;
    const snippets = snippetProps(src);
    expect([...snippets].sort()).toEqual(["children", "plan", "question"]);
  });
});

describe("componentDrift reverse direction", () => {
  it("flags an implemented prop the contract does not document", () => {
    const finding = componentDrift(
      "test",
      new Set(["documented"]),
      new Set(["documented", "undocumented"]),
      new Set(),
      new Set(),
      {},
    );
    expect(finding).not.toBeNull();
    expect(finding!.contractOnly).toEqual([]);
    expect(finding!.svelteOnly).toEqual(["undocumented"]);
  });

  it("does not count snippet slots as undocumented props", () => {
    const finding = componentDrift(
      "test",
      new Set(["documented"]),
      new Set(["documented", "children"]),
      new Set(["children"]),
      new Set(),
      {},
    );
    expect(finding).toBeNull();
  });

  it("does not flag target-specific props Svelte implements", () => {
    const finding = componentDrift(
      "test",
      new Set(["size"]),
      new Set(["size", "class"]),
      new Set(),
      new Set(["class"]),
      {},
    );
    expect(finding).toBeNull();
  });

  it("counts snippet implementations as satisfying the contract side", () => {
    // TextInput documents `leading`/`trailing`; Svelte implements them as
    // snippets — that is an implementation, not contract-only drift.
    const finding = componentDrift(
      "test",
      new Set(["leading"]),
      new Set(["leading"]),
      new Set(["leading"]),
      new Set(),
      {},
    );
    expect(finding).toBeNull();
  });
});
