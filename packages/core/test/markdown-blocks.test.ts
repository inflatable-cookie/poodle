import { describe, expect, test } from "bun:test";
import { marked } from "marked";

import {
  blocksFromMarked,
  markdownPlainText,
  type MarkedToken,
  type MdBlock,
} from "../src/markdown-blocks";

import fixtures from "../../contracts/headless/vectors/markdown-blocks.json" with { type: "json" };

const parse = (src: string): MdBlock[] =>
  blocksFromMarked(marked.lexer(src) as unknown as MarkedToken[]);

/**
 * The vector file is the contract between the two parsers. This side proves
 * `marked` still normalises to what is pinned; `poodle-markdown` proves
 * `pulldown-cmark` reaches the same blocks from the same source.
 *
 * A `marked` upgrade that changes tokenisation fails here rather than silently
 * putting the web target out of step with the natives.
 */
describe("markdown block fixtures", () => {
  for (const fixture of fixtures as Array<{ name: string; markdown: string; blocks: MdBlock[] }>) {
    test(fixture.name, () => {
      expect(parse(fixture.markdown)).toEqual(fixture.blocks);
    });
  }
});

describe("blocksFromMarked", () => {
  test("inline markup inside a paragraph survives as structure, not text", () => {
    expect(parse("run `bun test` and **stop**")).toEqual([
      {
        type: "paragraph",
        children: [
          { type: "text", value: "run " },
          { type: "code", value: "bun test" },
          { type: "text", value: " and " },
          { type: "strong", children: [{ type: "text", value: "stop" }] },
        ],
      },
    ]);
  });

  test("an unannotated fence reports no language rather than an empty one", () => {
    // `null` and `""` mean different things to a highlighter, and the Rust
    // mirror carries `Option<String>` to match.
    const [plain] = parse("```\nraw\n```");
    const [tagged] = parse("```rust\nfn main() {}\n```");

    expect(plain).toEqual({ type: "code", lang: null, value: "raw" });
    expect(tagged).toEqual({ type: "code", lang: "rust", value: "fn main() {}" });
  });

  test("a fence info string keeps only the language word", () => {
    expect(parse("```ts title=example.ts\nx\n```")[0]).toMatchObject({ lang: "ts" });
  });

  test("ordered lists keep their start index", () => {
    expect(parse("3. three\n4. four")[0]).toMatchObject({ ordered: true, start: 3 });
  });

  test("unknown block types degrade to text instead of vanishing", () => {
    // Dropping content silently is the worst available failure for a
    // transcript: an agent explaining HTML must not have the explanation
    // disappear because the parser called part of it raw HTML.
    const blocks = parse("<div>hello</div>");

    expect(blocks.length).toBeGreaterThan(0);
    expect(markdownPlainText(blocks)).toContain("hello");
  });

  test("empty input produces no blocks", () => {
    expect(parse("")).toEqual([]);
  });
});

describe("markdownPlainText", () => {
  test("includes code so a snippet-heavy message does not read as empty", () => {
    const text = markdownPlainText(parse("Try this:\n\n```sh\nbun test\n```"));

    expect(text).toContain("Try this:");
    expect(text).toContain("bun test");
  });

  test("flattens nested inline markup", () => {
    expect(markdownPlainText(parse("**bold [link](https://example.com)**"))).toBe("bold link");
  });
});
